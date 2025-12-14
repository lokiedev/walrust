use anyhow::Result;
use std::path::Path;

use crate::domain::models::wallpaper::Wallpaper;

pub trait WallpaperRepository {
    fn list_wallpapers(&self, path: &Path) -> Result<Vec<Wallpaper>>;
}
