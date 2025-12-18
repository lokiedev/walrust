use anyhow::{Result, anyhow};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use serde_json::Value;

#[inline]
pub fn get_home_dir() -> Result<PathBuf> {
    std::env::home_dir().ok_or(anyhow!("Could not determine home directory"))
}

#[inline]
pub fn run_hyprctl(args: &[&str]) -> Result<()> {
    Command::new("hyprctl")
        .args(args)
        .stderr(Stdio::piped())
        .output()?;

    Ok(())
}

pub fn get_monitor() -> Result<String> {
    let output = Command::new("hyprctl").arg("monitors").arg("-j").output()?;

    if output.status.success() {
        let json_str = String::from_utf8_lossy(&output.stdout);
        let monitors: Value = serde_json::from_str(&json_str)?;

        if let Some(monitors_array) = monitors.as_array() {
            Ok(monitors_array
                .first()
                .ok_or(anyhow!("Cannot access first index of monitors_array"))?["name"]
                .as_str()
                .ok_or(anyhow!(
                    "Cannot convert first index of monitors_array.name to str"
                ))?
                .to_owned())
        } else {
            Err(anyhow!("Failed to convert serde_json: Value to array"))
        }
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("{}", error_message))
    }
}
