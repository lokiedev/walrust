use crate::ports::MonitorProviderPort;

use anyhow::{Context, Result, anyhow, ensure};
use serde_json::Value;

pub struct HyprctlMonitorProvider;

impl MonitorProviderPort for HyprctlMonitorProvider {
    fn get_monitors(&self) -> Result<Vec<String>> {
        let arg = "monitors"; // Lists active output or monitor
        let flag = "-j"; // Output in JSON

        let output = super::utils::hyprctl(&[arg, flag])?;

        ensure!(
            output.status.success(),
            "hyprctl command failed and returned: {}",
            if output.stderr.is_empty() {
                String::from_utf8_lossy(&output.stdout)
            } else {
                String::from_utf8_lossy(&output.stderr)
            }
        );

        let stdout_utf8 = String::from_utf8_lossy(&output.stdout);
        let monitors: Value = serde_json::from_str(&stdout_utf8)
            .context("Failed to convert hyprctl output to JSON")?;

        if let Some(monitors_array) = monitors.as_array() {
            let name_properties = "name";
            let monitor_names: Vec<String> = monitors_array
                .iter()
                .filter_map(|monitor| monitor[name_properties].as_str())
                .map(|s| s.to_owned())
                .collect();
            Ok(monitor_names)
        } else {
            Err(anyhow!("hyprctl command returned nothing or not an array"))
        }
    }
}
