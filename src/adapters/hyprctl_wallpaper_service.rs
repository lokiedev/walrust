use std::{
    path::Path,
    process::{Command, Output},
};

use anyhow::{Context, Result, anyhow, ensure};
use serde_json::Value;

use crate::ports::wallpaper_service_port::WallpaperServicePort;

pub struct HyprctlWallpaperService {}

impl HyprctlWallpaperService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_monitor_names() -> Result<Vec<String>> {
        let arg = "monitors"; // Lists active output or monitor
        let flag = "-j"; // Output in JSON

        let output = Self::hyprctl(&[arg, flag])?;

        if !output.status.success() {
            let error_message = String::from_utf8(output.stderr).unwrap();
            return Err(anyhow!("hyprctl command returned error: {}", error_message));
        }

        let stdout_utf8 = String::from_utf8_lossy(&output.stdout);
        let monitors: Value = serde_json::from_str(&stdout_utf8)
            .context("Failed to convert hyprctl output to JSON")?;

        if let Some(monitors_array) = monitors.as_array() {
            let name_properties = "name";
            let monitor_names: Vec<String> = monitors_array
                .into_iter()
                .filter_map(|monitor| monitor[name_properties].as_str())
                .map(|s| s.to_owned())
                .collect();
            return Ok(monitor_names);
        } else {
            return Err(anyhow!("hyprctl command returned nothing or not an array"));
        }
    }

    fn hyprctl(args: &[&str]) -> Result<Output> {
        Command::new("hyprctl")
            .args(args)
            .output()
            .context("Failed to run hyprctl command")
    }
}

impl WallpaperServicePort for HyprctlWallpaperService {
    fn set_wallpaper(monitor_name: &str, path: &Path) -> Result<()> {
        let command = "hyprpaper";

        let unload_output = Self::hyprctl(&[command, "unload", "all"])
            .with_context(|| format!("Failed to unload all {} images", command))?;

        ensure!(
            unload_output.status.success(),
            "hyprctl command returned error: {}",
            String::from_utf8_lossy(&unload_output.stderr)
        );

        let path_string = path.display().to_string();

        let preload_output = Self::hyprctl(&[command, "preload", &path_string])
            .with_context(|| format!("Failed to preload {} to {}", &path_string, command))?;

        ensure!(
            preload_output.status.success(),
            "hyprctl command returned error: {}",
            String::from_utf8_lossy(&preload_output.stderr)
        );

        let change_wallpaper_output = Self::hyprctl(&[
            command,
            "wallpaper",
            &format!("{}, {}", monitor_name, path_string),
        ])
        .context("Failed to change wallpaper")?;

        ensure!(
            change_wallpaper_output.status.success(),
            "hyprctl command returned error: {}",
            String::from_utf8_lossy(&change_wallpaper_output.stderr)
        );

        Ok(())
    }
}
