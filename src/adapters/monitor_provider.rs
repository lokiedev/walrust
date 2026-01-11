use crate::{
    adapters::hyprctl::HyprctlMonitorProvider, models::desktop::Desktop, ports::MonitorProviderPort,
};

pub enum MonitorProvider {
    Hyprctl(HyprctlMonitorProvider),
}

impl MonitorProvider {
    pub fn from(desktop: &Desktop) -> Option<Self> {
        match desktop {
            Desktop::Hyprland => Some(MonitorProvider::Hyprctl(HyprctlMonitorProvider)),
            _ => None,
        }
    }
}

impl MonitorProviderPort for MonitorProvider {
    fn get_monitors(&self) -> anyhow::Result<Vec<String>> {
        match self {
            Self::Hyprctl(provider) => provider.get_monitors(),
        }
    }
}
