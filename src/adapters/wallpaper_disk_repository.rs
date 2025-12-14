use crate::domain::models::Wallpaper;
use crate::domain::ports::WallpaperRepository;
use anyhow::{Result, anyhow};
use std::{ffi::OsStr, fs, path::Path};

pub struct WallpaperDiskRepository;

impl WallpaperDiskRepository {
    pub fn new() -> Self {
        Self
    }
}

impl WallpaperRepository for WallpaperDiskRepository {
    fn list_wallpapers(&self, path: &Path) -> Result<Vec<Wallpaper>> {
        if !path.is_dir() {
            return Err(anyhow!("Path is not a valid directory"));
        }

        let entries = fs::read_dir(path)?;
        let mut files: Vec<Wallpaper> = Vec::with_capacity(32);

        for entry_result in entries {
            let entry = entry_result?;

            let file_type = entry.file_type()?;
            if !file_type.is_file() {
                continue;
            }

            let file_name_os = entry.file_name();
            let file_name = file_name_os.to_str();

            let file_name = match file_name {
                Some(name) => name,
                None => continue,
            };

            if is_image_file(&file_name_os) {
                files.push(Wallpaper::new(
                    file_name.to_owned(),
                    entry.path().to_string_lossy().into_owned(),
                ));
            }
        }

        files.shrink_to_fit();
        Ok(files)
    }
}

fn is_image_file(file_name: &OsStr) -> bool {
    let extension = match Path::new(file_name).extension() {
        Some(ext) => ext,
        None => return false,
    };

    let ext_lowercase = extension.to_ascii_lowercase();

    matches!(
        ext_lowercase.to_str(),
        Some("jpg") | Some("jpeg") | Some("png") | Some("webp")
    )
}
