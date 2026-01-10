use std::{env, path::PathBuf};

use anyhow::ensure;
use ratatui_image::picker::Picker;

use crate::{
    adapters::{HyprctlMonitorProvider, HyprctlWallpaperService, MonitorProvider},
    cli::Cli,
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

    let monitors = MonitorProvider::Hyprctl(HyprctlMonitorProvider).get_monitors()?;
    ensure!(!monitors.is_empty(), "No monitor detected");

    if path.is_dir() {
        let picker = Picker::from_query_stdio()?;
        let terminal = ratatui::init();
        let mut messages = Messages::new(250);

        messages.start_event_listener();

        let app = App::new(
            messages,
            path,
            monitors,
            picker,
            HyprctlWallpaperService::new(),
        )?
        .run(terminal);

        ratatui::restore();

        app
    } else {
        let wallpaper_service: HyprctlWallpaperService = HyprctlWallpaperService::new();

        Cli::run(wallpaper_service, &monitors, &path)
    }
}
