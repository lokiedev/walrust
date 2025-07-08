use anyhow::Result;

use crate::domain::models::Wallpaper;

pub trait WallpaperRepository {
    fn list_wallpapers(&self, path: &str) -> Result<Vec<Wallpaper>>;
}
