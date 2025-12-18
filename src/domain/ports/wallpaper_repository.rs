use std::path::Path;

use crate::{domain::models::wallpaper::Wallpaper, error::AppError};

pub trait WallpaperRepository {
    fn list_wallpapers(&self, path: &Path) -> Result<Vec<Wallpaper>, AppError>;
}
