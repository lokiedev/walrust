use crate::{
    error::AppError,
    utils::{get_monitor, run_hyprctl},
};

pub fn change_wallpaper(path: &str) -> Result<(), AppError> {
    run_hyprctl(&["hyprpaper", "unload", "all"])?;
    run_hyprctl(&["hyprpaper", "preload", path])?;

    let monitor = get_monitor()?;
    run_hyprctl(&["hyprpaper", "wallpaper", &format!("{}, {}", monitor, path)])?;

    Ok(())
}
