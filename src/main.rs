mod core;
mod ui;

use std::error::Error;
use ui::App;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = color_eyre::install();

    let terminal = ratatui::init();
    let app = App::new()?.run(terminal);

    ratatui::restore();

    Ok(app?)
}
