use std::{env, path::PathBuf};

use anyhow::ensure;
use ratatui_image::picker::Picker;

use crate::{
    adapters::HyprctlWallpaperService,
    ports::wallpaper_service_port::WallpaperServicePort,
    tui::{app::App, messages::Messages},
};

mod adapters;
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

    let monitors = HyprctlWallpaperService::get_monitor_names()?;
    ensure!(!monitors.is_empty(), "No monitor detected");

    if path.is_dir() {
        let picker = Picker::from_query_stdio()?;
        let terminal = ratatui::init();
        let mut messages = Messages::new(250);

        messages.start_event_listener();

        let app = App::new(
            messages,
            path,
            monitors[0].clone(),
            picker,
            HyprctlWallpaperService::new(),
        )?
        .run(terminal);

        ratatui::restore();

        app?;
    } else {
        let wallpaper_service: &dyn WallpaperServicePort = &HyprctlWallpaperService::new();

        wallpaper_service.set_wallpaper(&monitors[0], path.as_path())?;
    }

    Ok(())
}
