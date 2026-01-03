use std::{
    io::{self, Write},
    path::Path,
};

use anyhow::{Context, Result, ensure};

use crate::ports::WallpaperServicePort;

pub struct Cli {}

impl Cli {
    pub fn run<T: WallpaperServicePort>(
        wallpaper_service: T,
        monitors: &[String],
        image_path: &Path,
    ) -> Result<()> {
        if monitors.len() == 1 {
            return wallpaper_service.set_wallpaper(&monitors[0], image_path);
        }

        let selected_monitor = Self::choose_monitor_interface(&monitors)?;

        ensure!(
            selected_monitor > 0 && selected_monitor <= monitors.len(),
            "Invalid input, expected number between 1 and {}",
            monitors.len()
        );

        wallpaper_service.set_wallpaper(&monitors[selected_monitor - 1], image_path)
    }

    fn choose_monitor_interface(monitors: &[String]) -> Result<usize> {
        let mut selected_monitor = String::new();

        println!("Choose monitor to change wallpaper:");
        for (i, monitor) in monitors.iter().enumerate() {
            println!("[{}] {monitor}", i + 1);
        }
        print!("==> ");

        io::stdout().flush()?;
        io::stdin().read_line(&mut selected_monitor)?;

        let selected_monitor = selected_monitor.trim();

        selected_monitor
            .parse::<usize>()
            .with_context(|| "Expected number input")
    }
}
