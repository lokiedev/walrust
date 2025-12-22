use anyhow::Result;
use std::path::Path;

use crate::models::image_file::ImageFile;

pub trait ImageRepository {
    fn list_images(&self, path: &Path) -> Result<Vec<ImageFile>>;
}
