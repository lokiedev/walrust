use anyhow::{Context, Result};
use std::process::{Command, Output};

// Hyprctl sends its error output to stdout instead of stderr.
// So to get the error message, you can use the stdout only,
// or you can handle both for example:
// if output.stderr.is_empty() {
//    do something with the stdout
// } else {
//    do something with the stderr
// }
pub fn hyprctl(args: &[&str]) -> Result<Output> {
    Command::new("hyprctl")
        .args(args)
        .output()
        .context("Failed to run hyprctl command")
}
