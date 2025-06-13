use crate::core::wallpaper::Wallpaper;
use simplelog::*;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Loader {}

impl Loader {
    pub fn load_wallpaper(path: &str) -> io::Result<Vec<Wallpaper>> {
        let path = Path::new(path);

        if !path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Path: {}, is not a valid directory!",
                    path.to_str().unwrap()
                ),
            ));
        }

        let mut files: Vec<Wallpaper> = Vec::new();
        let entries = fs::read_dir(path)?;

        for entry_result in entries {
            let entry = entry_result?;

            if entry.file_type()?.is_file() {
                let file_name = match entry.file_name().into_string() {
                    Ok(name) => name,
                    Err(_) => continue,
                };

                if Self::is_image_file(&file_name) {
                    files.push(Wallpaper::new(
                        file_name,
                        entry.path().to_string_lossy().into_owned(),
                    ));
                }
            }
        }

        Ok(files)
    }

    pub fn load_logger(
        file_name: String,
        folder_path: PathBuf,
        level_filter: LevelFilter,
    ) -> io::Result<()> {
        if !folder_path.exists() {
            fs::create_dir_all(&folder_path)?;
        }

        let _ = CombinedLogger::init(vec![WriteLogger::new(
            level_filter,
            Config::default(),
            fs::File::create(folder_path.join(file_name))?,
        )]);

        Ok(())
    }

    fn is_image_file(file_name: &str) -> bool {
        const IMAGE_EXTENSIONS: &[&str] = &[".jpg", ".jpeg", ".png", ".webp"];

        let file_name_lowercase = file_name.to_ascii_lowercase();

        IMAGE_EXTENSIONS
            .iter()
            .any(|&ext| file_name_lowercase.ends_with(ext))
    }
}
