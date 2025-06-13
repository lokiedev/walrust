use crate::core::{Action, Wallpaper};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{List, ListItem, ListState},
};

pub struct Selector {
    pub wallpapers: Vec<Wallpaper>,
    pub list_state: ListState,
}

impl Selector {
    pub fn new(wallpapers: Vec<Wallpaper>) -> Self {
        Selector {
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
            KeyCode::Enter => {
                let selected_wallpaper = self.get_selected_wallpaper()?;

                Some(Action::SelectItem(selected_wallpaper.path.clone()))
            }
            _ => None,
        }
    }

    pub fn get_selected_wallpaper(&mut self) -> Option<&Wallpaper> {
        self.list_state
            .selected()
            .and_then(|i| self.wallpapers.get(i))
    }
}
