use std::path::Path;

use anyhow::Result;

pub trait WallpaperPort {
    fn set_wallpaper(path: &Path) -> Result<()>;
}
