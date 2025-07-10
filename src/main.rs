mod adapters;
mod app;
mod domain;
mod shared;
mod ui;

use adapters::utils::get_home_dir;
use app::App;
use shared::common_utils::load_logger;
use std::error::Error;

const LOG_FOLDER: &str = ".cache/walrust";
const LOG_NAME: &str = "walrust.log";

fn main() -> Result<(), Box<dyn Error>> {
    load_logger(
        LOG_NAME,
        &get_home_dir()?.join(LOG_FOLDER),
        log::LevelFilter::Debug,
    )?;
    log::info!("simplelog initialized");

    let terminal = ratatui::init();
    log::info!("Raw terminal initialized");

    let app = App::new()?.run(terminal);

    ratatui::restore();

    app
}
