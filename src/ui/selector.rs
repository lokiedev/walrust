use ratatui::{
    Frame,
    layout::Rect,
    widgets::{List, ListItem, Widget},
};

use crate::core::Wallpaper;

pub struct Selector {
    pub wallpapers: Vec<Wallpaper>,
    pub selected: u8,
}

impl Selector {
    pub fn new(wallpapers: Vec<Wallpaper>) -> Self {
        Selector {
            selected: 0,
            wallpapers,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, section: Rect) {
        // TODO: Implement selector UI

        List::new(
            self.wallpapers
                .iter()
                .map(|i| ListItem::from(i.name.to_owned())),
        )
        .render(section, frame.buffer_mut());
    }
}
