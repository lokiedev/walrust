use std::{env, path::PathBuf};

use anyhow::ensure;

use crate::{
    adapters::hyprctl_wallpaper_service::HyprctlWallpaperService,
    ports::wallpaper_service_port::WallpaperServicePort,
};

mod adapters;
mod models;
mod ports;
mod tui;

fn main() -> anyhow::Result<()> {
    let mut args = env::args();

    match args.nth(1) {
        Some(path) => {
            let pathbuf = PathBuf::from(&path);

            ensure!(pathbuf.exists(), "No such file or directory");

            if pathbuf.is_dir() {
                // TUI implementation
            } else {
                let wallpaper_service: &dyn WallpaperServicePort = &HyprctlWallpaperService::new();
                let monitors = HyprctlWallpaperService::get_monitor_names()?;

                ensure!(monitors.len() > 0, "No monitor detected");

                wallpaper_service.set_wallpaper(monitors[0].as_str(), pathbuf.as_path())?;
            }

            Ok(())
        }
        None => Err(anyhow::anyhow!("No arguments are provided")),
    }
}
