mod core;
mod ui;

use std::io;
use ui::App;

fn main() -> io::Result<()> {
    let _ = color_eyre::install();

    let terminal = ratatui::init();
    let app = App::new().run(terminal);

    ratatui::restore();

    app
}
