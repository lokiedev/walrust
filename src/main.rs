mod adapters;
mod app;
mod domain;
mod ui;

use adapters::utils::get_home_dir;
use app::App;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};

use crate::adapters::utils::{change_wallpaper, is_image_file};

const LOG_FOLDER: &str = ".cache/walrust";
const LOG_NAME: &str = "walrust.log";
const DEFAULT_WALLPAPER_PATH: &str = "";

fn main() -> Result<(), Box<dyn Error>> {
    load_logger(
        LOG_NAME,
        &get_home_dir()?.join(LOG_FOLDER),
        log::LevelFilter::Debug,
    )?;
    log::info!("simplelog initialized");

    log::info!("Getting path argument");
    let path = get_path_argument();
    if !path.exists() {
        log::error!("No such file or directory");
        return Err("No such file or directory".into());
    }

    if path.is_file() {
        return handle_file_argument(&path);
    }

    let terminal = ratatui::init();
    log::info!("Raw terminal initialized");

    let app = App::new(get_path_argument())?.run(terminal);

    ratatui::restore();

    app
}

fn handle_file_argument(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    if !is_image_file(path.as_os_str()) {
        log::error!("The specified file is not an image");
        return Err("The specified file is not an image".into());
    }

    change_wallpaper(
        path.to_str()
            .or_else(|| Some(""))
            .expect("Failed to change path to string"),
    )?;

    log::info!("Wallpaper changed succesfully");

    Ok(())
}

pub fn get_path_argument() -> PathBuf {
    env::args()
        .nth(1)
        .map_or_else(PathBuf::new, |path| PathBuf::from(path))
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
