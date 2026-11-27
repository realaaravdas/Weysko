use std::fs::File;
use std::io::Write;

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};
use weysko::studio::branding::render_logo_rgba;

fn main() {
    let sizes = [16u32, 32, 48, 64, 256];
    let mut frames: Vec<(u32, Vec<u8>)> = Vec::with_capacity(sizes.len());

    for &sz in &sizes {
        let rgba = render_logo_rgba(sz);
        let mut png_bytes = Vec::new();
        PngEncoder::new(&mut png_bytes)
            .write_image(&rgba, sz, sz, ExtendedColorType::Rgba8)
            .expect("encode PNG frame");
        frames.push((sz, png_bytes));
    }

    // Standard Windows ICO container with PNG frames
    let mut out = File::create("assets/weysko.ico").expect("create assets/weysko.ico");
    // ICONDIR: reserved(2)=0, type(2)=1 (ICO), count(2)
    out.write_all(&0u16.to_le_bytes()).unwrap();
    out.write_all(&1u16.to_le_bytes()).unwrap();
    out.write_all(&(frames.len() as u16).to_le_bytes()).unwrap();

    let mut offset = 6u32 + (frames.len() as u32) * 16;
    for (sz, png) in &frames {
        let dim = if *sz >= 256 { 0u8 } else { *sz as u8 };
        let entry: [u8; 16] = [
            dim, // width
            dim, // height
            0,   // color count
            0,   // reserved
            1, 0, // planes
            32, 0, // bpp
            (png.len() as u32).to_le_bytes()[0],
            (png.len() as u32).to_le_bytes()[1],
            (png.len() as u32).to_le_bytes()[2],
            (png.len() as u32).to_le_bytes()[3],
            offset.to_le_bytes()[0],
            offset.to_le_bytes()[1],
            offset.to_le_bytes()[2],
            offset.to_le_bytes()[3],
        ];
        out.write_all(&entry).unwrap();
        offset += png.len() as u32;
    }

    for (_, png) in &frames {
        out.write_all(png).unwrap();
    }
    println!("Generated assets/weysko.ico ({} frames)", frames.len());
}
