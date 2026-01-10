use crate::{adapters::HyprctlWallpaperService, ports::WallpaperServicePort};

pub enum WallpaperService {
    Hyprctl(HyprctlWallpaperService),
}

impl WallpaperServicePort for WallpaperService {
    fn set_wallpaper(&self, monitor_name: &str, path: &std::path::Path) -> anyhow::Result<()> {
        match self {
            Self::Hyprctl(service) => service.set_wallpaper(monitor_name, path),
        }
    }
}
