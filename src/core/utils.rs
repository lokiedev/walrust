use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use serde_json::Value;

#[inline]
pub fn get_home_dir() -> Result<PathBuf, Box<dyn Error>> {
    std::env::home_dir().ok_or("Could not determine home directory".into())
}

pub fn change_wallpaper(path: &str) -> Result<(), Box<dyn Error>> {
    let unload_output = Command::new("hyprctl")
        .args(["hyprpaper", "unload", "all"])
        .stderr(Stdio::piped())
        .output()?;

    if !unload_output.status.success() {
        return Err(String::from_utf8_lossy(&unload_output.stderr).into());
    }
    log::info!("Unloading wallpaper success");

    let preload_output = Command::new("hyprctl")
        .args(["hyprpaper", "preload", path])
        .stderr(Stdio::piped())
        .output()?;

    if !preload_output.status.success() {
        return Err(String::from_utf8_lossy(&preload_output.stderr).into());
    }
    log::info!("Preloading {} success", path);

    let monitor = get_monitor()?;

    let set_wallpaper_output = Command::new("hyprctl")
        .args(["hyprpaper", "wallpaper", &format!("{},{}", monitor, path)])
        .stderr(Stdio::piped())
        .output()?;

    if set_wallpaper_output.status.success() {
        log::info!("Changing wallpaper to {} success", path);
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&set_wallpaper_output.stderr).into())
    }
}

pub fn get_monitor() -> Result<String, Box<dyn Error>> {
    let output = Command::new("hyprctl").arg("monitors").arg("-j").output()?;

    if output.status.success() {
        let json_str = String::from_utf8_lossy(&output.stdout);
        let monitors: Value = serde_json::from_str(&json_str)?;

        if let Some(monitors_array) = monitors.as_array() {
            Ok(monitors_array
                .first()
                .ok_or("Cannot access first index of monitors_array")?["name"]
                .as_str()
                .ok_or("Cannot convert first index of monitors_array.name to str")?
                .to_owned())
        } else {
            Err("Failed to convert serde_json: Value to array".into())
        }
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into())
    }
}
