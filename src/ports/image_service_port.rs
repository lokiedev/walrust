use std::path::Path;

use anyhow::Result;
use image::DynamicImage;

pub trait ImageServicePort {
    fn decode(&self, path: &Path) -> Result<DynamicImage>;
}
