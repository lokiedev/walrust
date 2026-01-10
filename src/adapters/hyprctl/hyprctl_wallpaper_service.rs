use std::{
    path::Path,
    process::{Command, Output},
};

use anyhow::{Context, Result, ensure};

use crate::ports::wallpaper_service_port::WallpaperServicePort;

pub struct HyprctlWallpaperService {}

impl HyprctlWallpaperService {
    pub fn new() -> Self {
        Self {}
    }

    // Hyprctl sends its error output to stdout instead of stderr.
    // So to get the error message, you can use the stdout only,
    // or you can handle both for example:
    // if output.stderr.is_empty() {
    //    do something with the stdout
    // } else {
    //    do something with the stderr
    // }
    fn hyprctl(args: &[&str]) -> Result<Output> {
        Command::new("hyprctl")
            .args(args)
            .output()
            .context("Failed to run hyprctl command")
    }
}

impl WallpaperServicePort for HyprctlWallpaperService {
    fn set_wallpaper(&self, monitor_name: &str, path: &Path) -> Result<()> {
        let command = "hyprpaper";

        let _ = Self::hyprctl(&[command, "unload", "all"])
            .with_context(|| format!("Failed to unload all {} images", command))?;

        let path_string = path.display().to_string();

        let _ = Self::hyprctl(&[command, "preload", &path_string])
            .with_context(|| format!("Failed to preload {} to {}", &path_string, command))?;

        let change_wallpaper_output = Self::hyprctl(&[
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
