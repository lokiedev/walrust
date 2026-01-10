use std::path::Path;

use anyhow::{Context, Result};
use image::{DynamicImage, ImageReader};

use crate::ports::image_service_port::ImageServicePort;

#[derive(Clone)]
pub struct ImageService;

impl ImageServicePort for ImageService {
    fn decode(&self, path: &Path) -> Result<DynamicImage> {
        ImageReader::open(path)
            .with_context(|| format!("Failed to open and read image: {:?}", path))?
            .decode()
            .with_context(|| format!("Failed to decode image: {:?}", path))
    }
}
