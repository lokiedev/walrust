use crate::core::wallpaper::Wallpaper;
use simplelog::*;
use std::error::Error;
use std::ffi::OsStr;
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
                format!("Path: {:?}, is not a valid directory!", path),
            ));
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

            if Self::is_image_file(&file_name_os) {
                files.push(Wallpaper::new(
                    file_name.to_owned(),
                    entry.path().to_string_lossy().into_owned(),
                ));
            }
        }

        files.shrink_to_fit();
        Ok(files)
    }

    pub fn load_logger(
        file_name: &str,
        folder_path: &PathBuf,
        level_filter: LevelFilter,
    ) -> Result<(), Box<dyn Error>> {
        if !folder_path.exists() {
            fs::create_dir_all(folder_path)?;
        }

        let log_file_path = folder_path.join(file_name);
        let log_file = fs::File::create(log_file_path)?;

        CombinedLogger::init(vec![WriteLogger::new(
            level_filter,
            Config::default(),
            log_file,
        )])?;

        Ok(())
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
}
