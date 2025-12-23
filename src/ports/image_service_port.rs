use std::path::Path;

use anyhow::Result;
use image::DynamicImage;

pub trait ImageServicePort {
    fn decode(path: &Path) -> Result<DynamicImage>;
}
