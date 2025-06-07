use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

use crate::core::Wallpaper;

pub struct Preview {}

impl Preview {
    pub fn new() -> Self {
        Preview {}
    }

    pub fn draw(&mut self, wallpaper: Wallpaper, frame: &mut Frame, section: Rect) {
        let bordered_block = Block::new()
            .borders(Borders::RIGHT)
            .title("Wallpaper Preview");

        // TODO: Implement image rendering

        frame.render_widget(bordered_block, section);
    }
}
