use std::path::Path;

use anyhow::{bail, Context};
use fehler::throws;
use pngme_lib::png::Png;

#[throws(anyhow::Error)]
pub fn validate_png_path(path: &Path) {
    if !path.is_file() {
        bail!("Entered path is not a valid file.")
    }
}

#[throws(anyhow::Error)]
pub fn parse_png_from_file(path: &Path) -> Png {
    validate_png_path(path)?;

    let png_file = std::fs::read(path).context("failed to read png file")?;
    let png = Png::try_from(png_file.as_slice()).context("failed to parse png file")?;

    png
}

#[throws(anyhow::Error)]
pub fn save_png_to_file(png: Png, path: &Path) {
    std::fs::write(path, png.as_bytes()).context("failed to write png file")?;
}
