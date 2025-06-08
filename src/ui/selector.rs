use ratatui::{
    Frame,
    layout::Rect,
    widgets::{List, ListItem, ListState, Widget},
};

use crate::core::Wallpaper;

pub struct Selector {
    pub wallpapers: Vec<Wallpaper>,
    pub selected: u8,
    pub list_state: ListState,
}

impl Selector {
    pub fn new(wallpapers: Vec<Wallpaper>) -> Self {
        Selector {
            selected: 0,
            wallpapers,
            list_state: ListState::default(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, section: Rect) {
        // TODO: Implement selector UI

        let wallpaper_list = List::new(
            self.wallpapers
                .iter()
                .map(|i| ListItem::from(i.name.to_owned())),
        );

        frame.render_stateful_widget(wallpaper_list, section, &mut self.list_state);
    }
}
