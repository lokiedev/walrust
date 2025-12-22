use std::path::Path;

use anyhow::Result;

pub trait WallpaperServicePort {
    fn set_wallpaper(monitor_name: &str, path: &Path) -> Result<()>;
}
