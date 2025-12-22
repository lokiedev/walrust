use std::{io, path::Path};

use crate::models::image_file::ImageFile;

pub trait ImageRepository {
    fn list_images(&self, path: &Path) -> Result<Vec<ImageFile>, io::Error>;
}
