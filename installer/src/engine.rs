use std::env;
use std::fs;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};

use zip::ZipArchive;

/// Embedded compressed application payload compiled in by `installer/build.rs`.
pub const EMBEDDED_PAYLOAD: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/weysko-payload.zip"));

#[derive(Debug, Clone)]
pub struct InstallConfig {
    pub install_dir: PathBuf,
    pub create_start_menu: bool,
    pub create_desktop_shortcut: bool,
    pub add_to_path: bool,
    pub associate_files: bool,
    pub register_uninstaller: bool,
}

impl Default for InstallConfig {
    fn default() -> Self {
        Self {
            install_dir: default_install_dir(),
            create_start_menu: true,
            create_desktop_shortcut: true,
            add_to_path: true,
            associate_files: true,
            register_uninstaller: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstallSummary {
    pub install_dir: PathBuf,
    pub exe_path: PathBuf,
    pub installed_bytes: u64,
}

pub fn default_install_dir() -> PathBuf {
    if let Ok(local_app) = env::var("LOCALAPPDATA") {
        PathBuf::from(local_app).join("Programs").join("Weysko")
    } else if let Ok(profile) = env::var("USERPROFILE") {
        PathBuf::from(profile).join("Weysko")
    } else {
        PathBuf::from("C:\\Weysko")
    }
}

/// Extracts payload files, copies uninstaller, creates shortcuts, and updates Windows Registry.
pub fn run_install(
    config: &InstallConfig,
    progress_cb: impl Fn(f32, &str),
) -> Result<InstallSummary, String> {
    progress_cb(0.10, "Creating installation directory...");
    fs::create_dir_all(&config.install_dir)
        .map_err(|e| format!("Failed to create {}: {}", config.install_dir.display(), e))?;

    progress_cb(0.25, "Extracting Weysko application files...");
    let installed_bytes = extract_payload_zip(EMBEDDED_PAYLOAD, &config.install_dir)
        .map_err(|e| format!("Extraction failed: {}", e))?;

    progress_cb(0.50, "Installing uninstaller...");
    let uninstall_exe = config.install_dir.join("uninstall.exe");
    if let Ok(current_exe) = env::current_exe() {
        let same = fs::canonicalize(&current_exe).ok() == fs::canonicalize(&uninstall_exe).ok();
        if !same {
            fs::copy(&current_exe, &uninstall_exe)
                .map_err(|e| format!("Failed to write uninstaller: {}", e))?;
        }
    }

    let weysko_exe = config.install_dir.join("weysko.exe");
    let ico_path = config.install_dir.join("weysko.ico");

    #[cfg(windows)]
    {
        if config.create_start_menu {
            progress_cb(0.64, "Creating Start Menu shortcuts...");
            create_start_menu_shortcuts(&config.install_dir, &weysko_exe, &uninstall_exe, &ico_path)?;
        }

        if config.create_desktop_shortcut {
            progress_cb(0.74, "Creating Desktop shortcut...");
            create_desktop_shortcut(&config.install_dir, &weysko_exe, &ico_path)?;
        }

        if config.register_uninstaller {
            progress_cb(0.84, "Registering in Windows Apps & Features...");
            register_uninstall_entry(&config.install_dir, &uninstall_exe, &ico_path, installed_bytes)?;
        }

        if config.add_to_path {
            progress_cb(0.90, "Adding Weysko CLI to User PATH...");
            add_to_user_path(&config.install_dir)?;
        }

        if config.associate_files {
            progress_cb(0.95, "Associating .weysko and .weysko.json project files...");
            register_file_association(&weysko_exe, &ico_path)?;
        }
    }

    #[cfg(not(windows))]
    {
        let _ = (&weysko_exe, &ico_path);
    }

    progress_cb(1.0, "Installation complete.");
    Ok(InstallSummary {
        install_dir: config.install_dir.clone(),
        exe_path: weysko_exe,
        installed_bytes,
    })
}

pub fn extract_payload_zip(zip_bytes: &[u8], dest_dir: &Path) -> io::Result<u64> {
    let reader = Cursor::new(zip_bytes);
    let mut archive =
        ZipArchive::new(reader).map_err(|e| io::Error::other(e.to_string()))?;
    let mut total = 0u64;

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| io::Error::other(e.to_string()))?;
        let Some(rel) = entry.enclosed_name() else {
            continue;
        };
        let out_path = dest_dir.join(rel);
        if entry.is_dir() {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut out_file = fs::File::create(&out_path)?;
            total += io::copy(&mut entry, &mut out_file)?;
        }
    }
    Ok(total)
}

/// Removes shortcuts, registry keys, PATH entry, and application files.
pub fn run_uninstall(install_dir: &Path, progress_cb: impl Fn(f32, &str)) -> Result<(), String> {
    #[cfg(windows)]
    {
        progress_cb(0.25, "Removing Start Menu & Desktop shortcuts...");
        remove_shortcuts();

        progress_cb(0.50, "Removing Windows registry entries...");
        remove_registry_entries();

        progress_cb(0.70, "Removing Weysko from User PATH...");
        let _ = remove_from_user_path(install_dir);
    }

    #[cfg(not(windows))]
    {
        let _ = progress_cb(0.5, "Cleaning up...");
    }

    progress_cb(0.90, "Deleting installed files...");
    for fname in ["weysko.exe", "README.md", "LICENSE", "weysko.ico"] {
        let p = install_dir.join(fname);
        if p.exists() {
            let _ = fs::remove_file(&p);
        }
    }

    #[cfg(windows)]
    schedule_self_delete(install_dir);

    progress_cb(1.0, "Weysko has been uninstalled.");
    Ok(())
}

/// Appends `dir` to a semicolon-separated Windows PATH string if not already present.
pub fn add_dir_to_path_str(existing: &str, dir: &str) -> (String, bool) {
    let target = dir.trim().trim_end_matches('\\');
    let parts: Vec<&str> = existing
        .split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if parts
        .iter()
        .any(|p| p.trim_end_matches('\\').eq_ignore_ascii_case(target))
    {
        return (existing.to_string(), false);
    }
    let mut updated = parts.into_iter().map(str::to_string).collect::<Vec<_>>();
    updated.push(target.to_string());
    (updated.join(";"), true)
}

/// Removes `dir` from a semicolon-separated Windows PATH string.
pub fn remove_dir_from_path_str(existing: &str, dir: &str) -> (String, bool) {
    let target = dir.trim().trim_end_matches('\\');
    let mut removed = false;
    let kept: Vec<&str> = existing
        .split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter(|p| {
            let matches = p.trim_end_matches('\\').eq_ignore_ascii_case(target);
            if matches {
                removed = true;
            }
            !matches
        })
        .collect();
    (kept.join(";"), removed)
}

#[cfg(windows)]
fn start_menu_dir() -> Option<PathBuf> {
    env::var("APPDATA").ok().map(|appdata| {
        PathBuf::from(appdata)
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join("Weysko")
    })
}

#[cfg(windows)]
fn desktop_lnk_path() -> Option<PathBuf> {
    env::var("USERPROFILE")
        .ok()
        .map(|home| PathBuf::from(home).join("Desktop").join("Weysko.lnk"))
}

#[cfg(windows)]
fn create_shortcut(
    lnk_path: &Path,
    target_path: &Path,
    args: &str,
    work_dir: &Path,
    ico_path: &Path,
    desc: &str,
) -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    use std::process::Command;

    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    if let Some(parent) = lnk_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let esc = |s: &str| s.replace('\'', "''");
    let script = format!(
        "$ws = New-Object -ComObject WScript.Shell; \
         $sc = $ws.CreateShortcut('{}'); \
         $sc.TargetPath = '{}'; \
         $sc.Arguments = '{}'; \
         $sc.WorkingDirectory = '{}'; \
         $sc.IconLocation = '{},0'; \
         $sc.Description = '{}'; \
         $sc.Save()",
        esc(&lnk_path.to_string_lossy()),
        esc(&target_path.to_string_lossy()),
        esc(args),
        esc(&work_dir.to_string_lossy()),
        esc(&ico_path.to_string_lossy()),
        esc(desc),
    );

    let out = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-WindowStyle", "Hidden", "-Command", &script])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| format!("Failed to invoke shortcut helper: {}", e))?;

    if !out.status.success() {
        return Err(format!(
            "Shortcut creation failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(())
}

#[cfg(windows)]
fn create_start_menu_shortcuts(
    install_dir: &Path,
    weysko_exe: &Path,
    uninstall_exe: &Path,
    ico_path: &Path,
) -> Result<(), String> {
    if let Some(dir) = start_menu_dir() {
        create_shortcut(
            &dir.join("Weysko.lnk"),
            weysko_exe,
            "",
            install_dir,
            ico_path,
            "Weysko — Pure-Rust Visual Website Studio",
        )?;
        create_shortcut(
            &dir.join("Uninstall Weysko.lnk"),
            uninstall_exe,
            "--uninstall",
            install_dir,
            ico_path,
            "Uninstall Weysko Visual Website Studio",
        )?;
    }
    Ok(())
}

#[cfg(windows)]
fn create_desktop_shortcut(
    install_dir: &Path,
    weysko_exe: &Path,
    ico_path: &Path,
) -> Result<(), String> {
    if let Some(lnk) = desktop_lnk_path() {
        create_shortcut(
            &lnk,
            weysko_exe,
            "",
            install_dir,
            ico_path,
            "Weysko — Pure-Rust Visual Website Studio",
        )?;
    }
    Ok(())
}

#[cfg(windows)]
fn remove_shortcuts() {
    if let Some(dir) = start_menu_dir() {
        let _ = fs::remove_dir_all(dir);
    }
    if let Some(lnk) = desktop_lnk_path() {
        let _ = fs::remove_file(lnk);
    }
}

#[cfg(windows)]
fn register_uninstall_entry(
    install_dir: &Path,
    uninstall_exe: &Path,
    ico_path: &Path,
    installed_bytes: u64,
) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu
        .create_subkey(r"Software\Microsoft\Windows\CurrentVersion\Uninstall\Weysko")
        .map_err(|e| format!("Registry error: {}", e))?;

    let size_kb = ((installed_bytes / 1024).max(1)) as u32;
    let uninstall_cmd = format!("\"{}\" --uninstall", uninstall_exe.display());

    key.set_value("DisplayName", &"Weysko Visual Website Studio")
        .map_err(|e| e.to_string())?;
    key.set_value("DisplayVersion", &"1.3.2")
        .map_err(|e| e.to_string())?;
    key.set_value("Publisher", &"Weysko Project")
        .map_err(|e| e.to_string())?;
    key.set_value("DisplayIcon", &ico_path.to_string_lossy().as_ref())
        .map_err(|e| e.to_string())?;
    key.set_value("InstallLocation", &install_dir.to_string_lossy().as_ref())
        .map_err(|e| e.to_string())?;
    key.set_value("UninstallString", &uninstall_cmd)
        .map_err(|e| e.to_string())?;
    key.set_value("URLInfoAbout", &"https://realaaravdas.github.io/Weysko/")
        .map_err(|e| e.to_string())?;
    key.set_value("NoModify", &1u32).map_err(|e| e.to_string())?;
    key.set_value("NoRepair", &1u32).map_err(|e| e.to_string())?;
    key.set_value("EstimatedSize", &size_kb)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(windows)]
fn register_file_association(weysko_exe: &Path, ico_path: &Path) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    for ext_subkey in [
        r"Software\Classes\.weysko",
        r"Software\Classes\.weysko.json",
    ] {
        let (ext_key, _) = hkcu
            .create_subkey(ext_subkey)
            .map_err(|e| e.to_string())?;
        ext_key
            .set_value("", &"Weysko.Project")
            .map_err(|e| e.to_string())?;
    }

    let (prog_key, _) = hkcu
        .create_subkey(r"Software\Classes\Weysko.Project")
        .map_err(|e| e.to_string())?;
    prog_key
        .set_value("", &"Weysko Website Project")
        .map_err(|e| e.to_string())?;

    let (icon_key, _) = prog_key
        .create_subkey("DefaultIcon")
        .map_err(|e| e.to_string())?;
    icon_key
        .set_value("", &format!("{},0", ico_path.display()))
        .map_err(|e| e.to_string())?;

    let (cmd_key, _) = prog_key
        .create_subkey(r"shell\open\command")
        .map_err(|e| e.to_string())?;
    cmd_key
        .set_value("", &format!("\"{}\" \"%1\"", weysko_exe.display()))
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(windows)]
fn add_to_user_path(install_dir: &Path) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::{RegKey, RegValue};

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env_key, _) = hkcu
        .create_subkey("Environment")
        .map_err(|e| e.to_string())?;
    let current: String = env_key.get_value("Path").unwrap_or_default();
    let (updated, changed) = add_dir_to_path_str(&current, &install_dir.to_string_lossy());
    if changed {
        let wide: Vec<u16> = updated.encode_utf16().chain(std::iter::once(0)).collect();
        let bytes: Vec<u8> = wide.iter().flat_map(|u| u.to_le_bytes()).collect();
        env_key
            .set_raw_value(
                "Path",
                &RegValue {
                    bytes: bytes.into(),
                    vtype: RegType::REG_EXPAND_SZ,
                },
            )
            .map_err(|e| e.to_string())?;
        broadcast_env_change();
    }
    Ok(())
}

#[cfg(windows)]
fn remove_from_user_path(install_dir: &Path) -> Result<(), String> {
    use winreg::enums::*;
    use winreg::{RegKey, RegValue};

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(env_key) = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE) {
        let current: String = env_key.get_value("Path").unwrap_or_default();
        let (updated, changed) = remove_dir_from_path_str(&current, &install_dir.to_string_lossy());
        if changed {
            let wide: Vec<u16> = updated.encode_utf16().chain(std::iter::once(0)).collect();
            let bytes: Vec<u8> = wide.iter().flat_map(|u| u.to_le_bytes()).collect();
            let _ = env_key.set_raw_value(
                "Path",
                &RegValue {
                    bytes: bytes.into(),
                    vtype: RegType::REG_EXPAND_SZ,
                },
            );
            broadcast_env_change();
        }
    }
    Ok(())
}

#[cfg(windows)]
fn remove_registry_entries() {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let _ = hkcu.delete_subkey_all(r"Software\Microsoft\Windows\CurrentVersion\Uninstall\Weysko");
    let _ = hkcu.delete_subkey_all(r"Software\Classes\.weysko");
    let _ = hkcu.delete_subkey_all(r"Software\Classes\.weysko.json");
    let _ = hkcu.delete_subkey_all(r"Software\Classes\Weysko.Project");
}

#[cfg(windows)]
fn broadcast_env_change() {
    extern "system" {
        fn SendMessageTimeoutW(
            hwnd: isize,
            msg: u32,
            wparam: usize,
            lparam: *const u16,
            flags: u32,
            timeout: u32,
            result: *mut usize,
        ) -> isize;
    }
    const HWND_BROADCAST: isize = 0xFFFF;
    const WM_SETTINGCHANGE: u32 = 0x001A;
    const SMTO_ABORTIFHUNG: u32 = 0x0002;
    let env_wide: Vec<u16> = "Environment\0".encode_utf16().collect();
    let mut res = 0usize;
    unsafe {
        let _ = SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            0,
            env_wide.as_ptr(),
            SMTO_ABORTIFHUNG,
            1000,
            &mut res,
        );
    }
}

#[cfg(windows)]
fn schedule_self_delete(install_dir: &Path) {
    use std::os::windows::process::CommandExt;
    use std::process::Command;

    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    let dir_str = install_dir.to_string_lossy().replace('"', "");
    let cmd = format!(
        "ping 127.0.0.1 -n 3 > NUL & rmdir /S /Q \"{}\"",
        dir_str
    );
    let _ = Command::new("cmd")
        .args(["/C", &cmd])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_helpers_add_dedup_and_remove_cleanly() {
        let (p1, ch1) = add_dir_to_path_str(r"C:\Windows\System32;C:\Tools\", r"C:\Apps\Weysko");
        assert!(ch1);
        assert_eq!(p1, r"C:\Windows\System32;C:\Tools\;C:\Apps\Weysko");

        // Case-insensitive & trailing-slash deduplication
        let (p2, ch2) = add_dir_to_path_str(&p1, r"c:\apps\weysko\");
        assert!(!ch2);
        assert_eq!(p2, p1);

        let (p3, ch3) = remove_dir_from_path_str(&p1, r"C:\Apps\Weysko\");
        assert!(ch3);
        assert_eq!(p3, r"C:\Windows\System32;C:\Tools\");
    }

    #[test]
    fn default_install_dir_ends_with_weysko() {
        let dir = default_install_dir();
        assert_eq!(dir.file_name().and_then(|n| n.to_str()), Some("Weysko"));
    }
}
