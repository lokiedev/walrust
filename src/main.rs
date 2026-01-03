use std::{
    env,
    io::{self, Write},
    path::PathBuf,
};

use anyhow::{bail, ensure};
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
            monitors,
            picker,
            HyprctlWallpaperService::new(),
        )?
        .run(terminal);

        ratatui::restore();

        app?;
    } else {
        let wallpaper_service: &dyn WallpaperServicePort = &HyprctlWallpaperService::new();
        let mut selected_monitor = String::new();

        println!("Choose monitor to change wallpaper:");
        for (i, monitor) in monitors.iter().enumerate() {
            println!("[{}] {monitor}", i + 1);
        }
        print!("==> ");

        io::stdout().flush()?;
        io::stdin().read_line(&mut selected_monitor)?;

        let selected_monitor = selected_monitor.trim();

        match selected_monitor.parse::<usize>() {
            Ok(i) => {
                if i - 1 >= monitors.len() {
                    bail!("Please choose available monitor only");
                }
                wallpaper_service.set_wallpaper(&monitors[i - 1], path.as_path())?;
            }
            Err(_) => {
                bail!("Expected a number input");
            }
        }
    }

    Ok(())
}
