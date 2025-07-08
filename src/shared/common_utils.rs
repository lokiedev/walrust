use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

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
