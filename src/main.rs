use std::{env, path::PathBuf};

use anyhow::ensure;

use crate::{
    adapters::HyprctlWallpaperService, ports::wallpaper_service_port::WallpaperServicePort,
    tui::app::App,
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

            let monitors = HyprctlWallpaperService::get_monitor_names()?;
            ensure!(!monitors.is_empty(), "No monitor detected");

            if pathbuf.is_dir() {
                let monitor = monitors[0].clone();
                App::new(&pathbuf, monitor).run()?;
            } else {
                let wallpaper_service: &dyn WallpaperServicePort = &HyprctlWallpaperService::new();

                wallpaper_service.set_wallpaper(monitors[0].as_str(), pathbuf.as_path())?;
            }

            Ok(())
        }
        None => Err(anyhow::anyhow!("No arguments are provided")),
    }
}
