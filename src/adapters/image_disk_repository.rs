use std::{fs, io, path::Path};

use crate::{models::image_file::ImageFile, ports::image_repository::ImageRepository};

pub struct ImageDiskRepository {
    image_extensions: Vec<String>,
}

impl ImageDiskRepository {
    fn is_supported_image(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.image_extensions.contains(&ext.to_lowercase()))
            .unwrap_or(false)
    }
}

impl ImageRepository for ImageDiskRepository {
    fn list_images(&self, path: &Path) -> Result<Vec<ImageFile>, io::Error> {
        let dir = fs::read_dir(path)?;
        let mut images: Vec<ImageFile> = Vec::new();

        for file_result in dir {
            let file = file_result?;
            let file_type = file.file_type()?;

            if !file_type.is_file() {
                continue;
            }

            if !self.is_supported_image(&file.path()) {
                continue;
            }

            let file_name = file.file_name();
            let file_name_str = file_name.to_str().unwrap();
            let file_path = file.path();

            images.push(ImageFile::new(file_name_str.to_owned(), file_path));
        }

        Ok(images)
    }
}
