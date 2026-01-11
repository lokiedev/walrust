use std::{env, path::PathBuf};

use anyhow::{anyhow, ensure};
use ratatui_image::picker::Picker;

use crate::{
    adapters::{MonitorProvider, WallpaperService},
    cli::Cli,
    models::desktop::Desktop,
    ports::MonitorProviderPort,
    tui::{app::App, messages::Messages},
};

mod adapters;
mod cli;
mod models;
mod ports;
mod tui;

fn main() -> anyhow::Result<()> {
    let mut args = env::args();
    let path = args
        .nth(1)
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("No arguments are provided"))?;

    ensure!(path.exists(), "No such file or directory");

    let desktop = Desktop::detect();
    ensure!(!desktop.is_unknown(), "Your WM is unknown or not supported");

    let monitors = MonitorProvider::from(&desktop)
        .ok_or(anyhow!("Your WM is not supported"))?
        .get_monitors()?;
    ensure!(!monitors.is_empty(), "No monitor detected");

    let wallpaper_service =
        WallpaperService::from(&desktop).ok_or(anyhow!("Your WM is not supported"))?;

    if path.is_dir() {
        let picker = Picker::from_query_stdio()?;
        let terminal = ratatui::init();
        let mut messages = Messages::new(250);

        messages.start_event_listener();

        let app = App::new(messages, path, monitors, picker, wallpaper_service)?.run(terminal);

        ratatui::restore();

        app
    } else {
        Cli::run(wallpaper_service, &monitors, &path)
    }
}
