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

    let terminal = ratatui::init();
    log::info!("Raw terminal initialized");

    let app = App::new(get_path_argument())?.run(terminal);

    ratatui::restore();

    app
}

pub fn get_path_argument() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    let path_nth = 1;

    if let Some(path) = args.get(path_nth) {
        PathBuf::from(path)
    } else {
        PathBuf::from("")
    }
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
