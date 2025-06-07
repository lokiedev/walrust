use ratatui::{Frame, layout::Rect};

use crate::core::Wallpaper;

pub struct Selector {
    pub wallpapers: Option<Vec<Wallpaper>>,
    pub selected: Option<u8>,
}

impl Selector {
    pub fn new() -> Self {
        Selector {
            selected: Some(0),
            wallpapers: Some(Vec::new()),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, section: Rect) {
        // TODO: Implement selector UI
    }
}
