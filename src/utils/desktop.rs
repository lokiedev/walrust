use std::path::PathBuf;
use std::process::{Command, Stdio};

use serde_json::Value;

use crate::error::AppError;

#[inline]
pub fn get_home_dir() -> Result<PathBuf, AppError> {
    std::env::home_dir().ok_or(AppError::HomeDirNotFound)
}

#[inline]
pub fn run_hyprctl(args: &[&str]) -> Result<(), AppError> {
    Command::new("hyprctl")
        .args(args)
        .stderr(Stdio::piped())
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn get_monitor() -> Result<String, AppError> {
    let output = Command::new("hyprctl").arg("monitors").arg("-j").output()?;

    if !output.status.success() {
        let error_message = String::from_utf8(output.stderr)
            .unwrap_or_else(|_| "Failed to convert hyprctl error message to String".to_string());

        return Err(AppError::MonitorDetection(format!(
            "hyprctl returned error: {}",
            error_message
        )));
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let monitors: Value = serde_json::from_str(&json_str)?;

    if let Some(monitors_array) = monitors.as_array() {
        Ok(monitors_array
            .first()
            .ok_or_else(|| AppError::MonitorDetection("No monitor found".to_string()))?["name"]
            .as_str()
            .ok_or_else(|| {
                AppError::MonitorDetection("Failed to convert monitor name".to_string())
            })?
            .to_owned())
    } else {
        Err(AppError::MonitorDetection(
            "Monitor data is not an array".to_string(),
        ))
    }
}
