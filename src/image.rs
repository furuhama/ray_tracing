use std::fs::File;
use std::io::{self, Write};

use crate::vec3_glam::ColorGlam;

pub fn write_ppm(path: &str, width: u32, height: u32, pixels: &[ColorGlam]) -> io::Result<()> {
    let mut file = File::create(path)?;

    // PPMヘッダーの書き込み
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "255")?;

    // ピクセルデータの書き込み
    for pixel in pixels {
        let (r, g, b) = pixel.to_rgb();
        writeln!(file, "{} {} {}", r, g, b)?;
    }

    Ok(())
}
