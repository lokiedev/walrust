use ratatui::{Frame, layout::Rect};

use crate::core::Wallpaper;

pub struct Selector {
    pub wallpapers: Vec<Wallpaper>,
    pub selected: u8,
}

impl Selector {
    pub fn new() -> Self {
        Selector {
            selected: 0,
            wallpapers: Vec::new(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, section: Rect) {
        // TODO: Implement selector UI
    }
}
