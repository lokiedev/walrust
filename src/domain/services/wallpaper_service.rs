use anyhow::Result;
use std::path::Path;

use crate::{
    domain::models::wallpaper::Wallpaper, domain::ports::wallpaper_repository::WallpaperRepository,
};

pub struct WallpaperService<R: WallpaperRepository> {
    repository: R,
}

impl<R: WallpaperRepository> WallpaperService<R> {
    pub fn new(repository: R) -> Self {
        WallpaperService { repository }
    }

    pub fn get_wallpapers(&self, path: &Path) -> Result<Vec<Wallpaper>> {
        self.repository.list_wallpapers(path)
    }
}
