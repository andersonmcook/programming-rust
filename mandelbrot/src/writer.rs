use image::codecs::png::PngEncoder;
use image::error::ImageResult;
use image::{ColorType, ImageEncoder};
use std::fs::File;

pub fn write(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> ImageResult<()> {
    PngEncoder::new(File::create(filename)?).write_image(
        pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::L8,
    )
}
