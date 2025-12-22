use std::path::Path;

use anyhow::Result;

pub trait WallpaperServicePort {
    fn set_wallpaper(path: &Path) -> Result<()>;
}
