mod action;
mod adapters;
mod app;
mod domain;
mod error;
mod ui;
mod utils;

use app::App;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use std::path::PathBuf;
use std::{env, fs};
use utils::{change_wallpaper, get_home_dir, is_image_file};

use crate::app::AppResult;
use crate::error::AppError;

const LOG_NAME: &str = "walrust.log";
const LOG_FOLDER: &str = ".cache/walrust";
const LOG_LEVEL: LevelFilter = log::LevelFilter::Debug;

fn main() -> AppResult<()> {
    setup_logger(LOG_NAME, &get_home_dir()?.join(LOG_FOLDER), LOG_LEVEL)?;
    log::info!("Logger initialized");

    let path = get_path_argument();
    log::debug!("Path argument: {:?}", path);

    if !path.exists() {
        log::error!("Path does not exist: {:?}", path);
        return Err(AppError::InvalidPath(
            "No such file or directory".to_string(),
        ));
    }

    if path.is_file() {
        log::info!("Handling file argument: {:?}", path);
        return handle_file_argument(path);
    }

    log::info!("Path is a directory, initializing TUI");
    let terminal = ratatui::init();
    log::info!("Terminal initialized");

    let app = App::new(path).run(terminal);

    ratatui::restore();
    log::info!("Terminal restored");

    app
}

fn handle_file_argument(path: PathBuf) -> AppResult<()> {
    if !is_image_file(&path) {
        log::error!("The specified file is not an image");
        return Err(AppError::InvalidPath(
            "The specified file is not a supported image".to_string(),
        ));
    }

    change_wallpaper(
        path.to_str()
            .or_else(|| Some(""))
            .expect("Failed to change path to string"),
    )?;

    log::info!("Wallpaper changed succesfully");

    Ok(())
}

fn get_path_argument() -> PathBuf {
    env::args()
        .nth(1)
        .map_or_else(PathBuf::new, |path| PathBuf::from(path))
}

fn setup_logger(
    file_name: &str,
    folder_path: &PathBuf,
    level_filter: LevelFilter,
) -> AppResult<()> {
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
