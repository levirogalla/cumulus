use std::io::{self, Cursor};

use image::ImageReader;

pub fn generate_thumbnail(original: &[u8], size: u32) -> Result<Vec<u8>, ThumbnailError> {
    let reader = ImageReader::new(Cursor::new(original)).with_guessed_format()?;
    let img = reader.decode()?;
    let thumb = img.thumbnail(size, size);

    let mut buffer: Vec<u8> = Vec::new();
    thumb
        .write_to(Cursor::new(&mut buffer), image::ImageFormat::Jpeg)
        .expect("Unable to write image to return buffer");

    Ok(buffer)
}

#[derive(Debug, thiserror::Error)]
pub enum ThumbnailError {
    #[error("Format Error")]
    FormatError(#[from] io::Error),

    #[error("Decode Error")]
    DecodeError(#[from] image::ImageError),
}
