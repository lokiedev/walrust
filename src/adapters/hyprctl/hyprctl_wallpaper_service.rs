use anyhow::{Context, Result, ensure};
use std::{
    path::Path,
    process::{Command, Output},
};

use crate::ports::wallpaper_service_port::WallpaperServicePort;

use super::utils;

pub struct HyprctlWallpaperService {}

impl HyprctlWallpaperService {
    pub fn new() -> Self {
        Self {}
    }
}

impl WallpaperServicePort for HyprctlWallpaperService {
    fn set_wallpaper(&self, monitor_name: &str, path: &Path) -> Result<()> {
        let command = "hyprpaper";

        let _ = utils::hyprctl(&[command, "unload", "all"])
            .with_context(|| format!("Failed to unload all {} images", command))?;

        let path_string = path.display().to_string();

        let _ = utils::hyprctl(&[command, "preload", &path_string])
            .with_context(|| format!("Failed to preload {} to {}", &path_string, command))?;

        let change_wallpaper_output = utils::hyprctl(&[
            command,
            "wallpaper",
            &format!("{}, {}", monitor_name, path_string),
        ])
        .context("Failed to change wallpaper")?;

        ensure!(
            change_wallpaper_output.status.success(),
            "hyprctl command failed and returned: {}",
            if change_wallpaper_output.stderr.is_empty() {
                String::from_utf8_lossy(&change_wallpaper_output.stdout)
            } else {
                String::from_utf8_lossy(&change_wallpaper_output.stderr)
            }
        );

        Ok(())
    }
}
