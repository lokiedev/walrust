use anyhow::{Context, Result};
use std::process::{Command, Output};

pub fn hyprctl(args: &[&str]) -> Result<Output> {
    Command::new("hyprctl")
        .args(args)
        .output()
        .context("Failed to run hyprctl command")
}
