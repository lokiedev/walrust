use crate::{
    adapters::HyprctlWallpaperService, models::desktop::Desktop, ports::WallpaperServicePort,
};

pub enum WallpaperService {
    Hyprctl(HyprctlWallpaperService),
}

impl WallpaperService {
    pub fn from(desktop: &Desktop) -> Option<Self> {
        match desktop {
            Desktop::Hyprland => Some(WallpaperService::Hyprctl(HyprctlWallpaperService)),
            _ => None,
        }
    }
}

impl WallpaperServicePort for WallpaperService {
    fn set_wallpaper(&self, monitor_name: &str, path: &std::path::Path) -> anyhow::Result<()> {
        match self {
            Self::Hyprctl(service) => service.set_wallpaper(monitor_name, path),
        }
    }
}
