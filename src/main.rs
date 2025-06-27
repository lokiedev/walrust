mod core;
mod ui;

use std::error::Error;
use ui::App;

use crate::core::{Loader, get_home_dir};

const LOG_FOLDER: &str = ".cache/walrust";
const LOG_NAME: &str = "walrust.log";

fn main() -> Result<(), Box<dyn Error>> {
    Loader::load_logger(
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
