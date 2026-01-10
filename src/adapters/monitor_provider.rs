use crate::{adapters::hyprctl::HyprctlMonitorProvider, ports::MonitorProviderPort};

pub enum MonitorProvider {
    Hyprctl(HyprctlMonitorProvider),
}

impl MonitorProviderPort for MonitorProvider {
    fn get_monitors(&self) -> anyhow::Result<Vec<String>> {
        match self {
            Self::Hyprctl(provider) => provider.get_monitors(),
        }
    }
}
