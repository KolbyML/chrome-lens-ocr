use std::{io::Cursor, path::Path};

use anyhow::{Context, Result};
use image::{ImageFormat, ImageReader};

use crate::constants::DEFAULT_IMAGE_MAX_DIMENSION;

pub struct ProcessedImage {
    pub bytes: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

pub fn process_image_from_path<P: AsRef<Path>>(path: P) -> Result<ProcessedImage> {
    let img = ImageReader::open(path)
        .context("Failed to open image file")?
        .with_guessed_format()
        .context("Failed to guess image format")?
        .decode()
        .context("Failed to decode image")?;

    process_image_internal(img)
}

pub fn process_image_from_bytes(data: &[u8]) -> Result<ProcessedImage> {
    let img = image::load_from_memory(data).context("Failed to load image from memory")?;
    process_image_internal(img)
}

fn process_image_internal(mut img: image::DynamicImage) -> Result<ProcessedImage> {
    // Resize if larger than max dimension
    let (w, h) = (img.width(), img.height());
    if w > DEFAULT_IMAGE_MAX_DIMENSION || h > DEFAULT_IMAGE_MAX_DIMENSION {
        img = img.resize(
            DEFAULT_IMAGE_MAX_DIMENSION,
            DEFAULT_IMAGE_MAX_DIMENSION,
            image::imageops::FilterType::Lanczos3,
        );
    }

    let (final_w, final_h) = (img.width() as i32, img.height() as i32);

    // Convert to RGBA and save as PNG bytes
    let mut bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);

    // Lens expects PNG (or supported formats), mapping Python's logic
    img.write_to(&mut cursor, ImageFormat::Png)
        .context("Failed to write image to buffer")?;

    Ok(ProcessedImage {
        bytes,
        width: final_w,
        height: final_h,
    })
}
