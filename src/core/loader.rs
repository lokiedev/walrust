use crate::core::wallpaper::Wallpaper;
use std::fs;
use std::io;
use std::path::Path;

pub struct Loader {}

impl Loader {
    pub fn wallpaper(path: &str) -> io::Result<Vec<Wallpaper>> {
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

                if is_image(&file_name) {
                    files.push(Wallpaper::new(
                        file_name,
                        entry.path().to_string_lossy().into_owned(),
                    ));
                }
            }
        }

        Ok(files)
    }
}

fn is_image(file_name: &str) -> bool {
    const IMAGE_EXTENSIONS: &[&str] = &[".jpg", ".jpeg", ".png", ".webp"];

    let file_name_lowercase = file_name.to_ascii_lowercase();

    IMAGE_EXTENSIONS
        .iter()
        .any(|&ext| file_name_lowercase.ends_with(ext))
}
