use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{List, ListItem, ListState},
};

use crate::core::{Action, Wallpaper};

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

    pub fn init(&mut self) {
        self.list_state.select_first();
    }

    pub fn draw(&mut self, frame: &mut Frame, section: Rect) {
        // TODO: Implement selector UI

        let wallpaper_list = List::new(
            self.wallpapers
                .iter()
                .map(|i| ListItem::from(i.name.to_owned())),
        )
        .highlight_symbol("> ");

        frame.render_stateful_widget(wallpaper_list, section, &mut self.list_state);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => Some(Action::PreviousItem),
            KeyCode::Down | KeyCode::Char('j') => Some(Action::NextItem),
            _ => None,
        }
    }
}
