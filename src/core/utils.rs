use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use serde_json::Value;

#[inline]
pub fn get_home_dir() -> Result<PathBuf, Box<dyn Error>> {
    std::env::home_dir().ok_or("Could not determine home directory".into())
}

pub fn change_wallpaper(path: &str) -> Result<(), Box<dyn Error>> {
    run_hyprctl(&["hyprpaper", "unload", "all"])?;
    run_hyprctl(&["hyprpaper", "preload", path])?;

    let monitor = get_monitor()?;
    run_hyprctl(&["hyprpaper", "wallpaper", &format!("{}, {}", monitor, path)])?;

    Ok(())
}

#[inline]
fn run_hyprctl(args: &[&str]) -> Result<(), Box<dyn Error>> {
    Command::new("hyprctl")
        .args(args)
        .stderr(Stdio::piped())
        .output()?;

    Ok(())
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
