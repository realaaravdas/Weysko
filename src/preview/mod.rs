use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use tiny_http::{Header, Response, Server, StatusCode};

use crate::generator::GeneratedSite;

/// Embedded pure-Rust HTTP server for live browser preview of the compiled website.
pub struct PreviewServer {
    port: u16,
    site: Arc<RwLock<GeneratedSite>>,
    shutdown: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl PreviewServer {
    /// Starts the preview HTTP server on `127.0.0.1:<port>` (or an ephemeral port if `None`).
    pub fn start(site: GeneratedSite, port: Option<u16>) -> io::Result<Self> {
        let bind_addr = format!("127.0.0.1:{}", port.unwrap_or(0));
        let server = Server::http(&bind_addr)
            .map_err(|e| io::Error::new(io::ErrorKind::AddrInUse, e.to_string()))?;

        let actual_port = match server.server_addr() {
            tiny_http::ListenAddr::IP(addr) => addr.port(),
            #[cfg(unix)]
            _ => port.unwrap_or(8080),
        };

        let shared_site = Arc::new(RwLock::new(site));
        let shutdown = Arc::new(AtomicBool::new(false));

        let site_ref = Arc::clone(&shared_site);
        let shutdown_ref = Arc::clone(&shutdown);

        let handle = thread::spawn(move || {
            while !shutdown_ref.load(Ordering::Relaxed) {
                match server.recv_timeout(Duration::from_millis(120)) {
                    Ok(Some(req)) => {
                        let raw_url = req.url().split('?').next().unwrap_or("/");
                        let rel_path = match raw_url {
                            "/" | "" => "index.html",
                            other => other.trim_start_matches('/'),
                        };

                        let guard = site_ref.read().unwrap();
                        if let Some(bytes) = guard.files.get(rel_path) {
                            let ctype = content_type_for_path(rel_path);
                            let resp = Response::from_data(bytes.clone())
                                .with_status_code(StatusCode(200))
                                .with_header(
                                    Header::from_bytes(&b"Content-Type"[..], ctype.as_bytes())
                                        .unwrap(),
                                )
                                .with_header(
                                    Header::from_bytes(&b"Cache-Control"[..], &b"no-store"[..])
                                        .unwrap(),
                                );
                            let _ = req.respond(resp);
                        } else if let Some(not_found) = guard.files.get("404.html") {
                            let resp = Response::from_data(not_found.clone())
                                .with_status_code(StatusCode(404))
                                .with_header(
                                    Header::from_bytes(
                                        &b"Content-Type"[..],
                                        &b"text/html; charset=utf-8"[..],
                                    )
                                    .unwrap(),
                                );
                            let _ = req.respond(resp);
                        } else {
                            let resp = Response::from_string("Not Found")
                                .with_status_code(StatusCode(404));
                            let _ = req.respond(resp);
                        }
                    }
                    Ok(None) => {}
                    Err(_) => break,
                }
            }
        });

        Ok(Self {
            port: actual_port,
            site: shared_site,
            shutdown,
            handle: Some(handle),
        })
    }

    /// Returns the localhost URL for opening in a browser.
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    /// Hot-swaps the served `GeneratedSite` in memory with zero restart.
    pub fn update_site(&self, next: GeneratedSite) {
        if let Ok(mut guard) = self.site.write() {
            *guard = next;
        }
    }

    /// Gracefully stops the background server thread.
    pub fn stop(mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

impl Drop for PreviewServer {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

fn content_type_for_path(path: &str) -> &'static str {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if lower.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if lower.ends_with(".js") {
        "application/javascript; charset=utf-8"
    } else if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".svg") {
        "image/svg+xml"
    } else if lower.ends_with(".xml") {
        "application/xml; charset=utf-8"
    } else {
        "application/octet-stream"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::compile_site;
    use crate::templates::StarterTemplate;
    use std::io::{Read, Write};
    use std::net::TcpStream;

    #[test]
    fn preview_server_serves_html_and_css_over_localhost() {
        let project = StarterTemplate::MinimalistPortfolio.build();
        let site = compile_site(&project);
        let server = PreviewServer::start(site, None).unwrap();
        let addr = format!("127.0.0.1:{}", server.port());

        let mut stream = TcpStream::connect(&addr).unwrap();
        stream
            .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
            .unwrap();
        let mut resp = String::new();
        stream.read_to_string(&mut resp).unwrap();

        assert!(resp.starts_with("HTTP/1.1 200 OK"));
        assert!(resp.contains("<!DOCTYPE html>"));
        assert!(resp.contains("styles.css"));

        let mut css_stream = TcpStream::connect(&addr).unwrap();
        css_stream
            .write_all(b"GET /styles.css HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
            .unwrap();
        let mut css_resp = String::new();
        css_stream.read_to_string(&mut css_resp).unwrap();
        assert!(css_resp.starts_with("HTTP/1.1 200 OK"));
        assert!(css_resp.contains("--ws-bg:"));

        server.stop();
    }
}
