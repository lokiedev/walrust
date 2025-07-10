use crate::{
    adapters::utils::change_wallpaper, app::Action, domain::models::Wallpaper,
    domain::ports::UIComponent,
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{List, ListItem, ListState},
};

pub struct Selector {
    pub wallpapers: Vec<Wallpaper>,
    list_state: ListState,
}

impl Selector {
    pub fn new() -> Self {
        Selector {
            wallpapers: Vec::new(),
            list_state: ListState::default(),
        }
    }

    pub fn update_wallpapers(&mut self, wallpapers: Vec<Wallpaper>) {
        self.wallpapers = wallpapers;
    }

    pub fn init(&mut self) {
        self.list_state.select_first();
    }

    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    pub fn get_selected_wallpaper(&self) -> Option<&Wallpaper> {
        self.list_state
            .selected()
            .and_then(|i| self.wallpapers.get(i))
    }
}

impl UIComponent for Selector {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        // TODO: Implement selector UI

        let wallpaper_list = List::new(
            self.wallpapers
                .iter()
                .map(|i| ListItem::from(i.name.to_owned())),
        )
        .highlight_symbol("> ");

        frame.render_stateful_widget(wallpaper_list, area, &mut self.list_state);
    }

    #[inline]
    fn dispatch_action(&mut self, action: &Action) {
        match action {
            Action::NextItem => self.select_next(),
            Action::PreviousItem => self.select_previous(),
            Action::SelectItem(path) => {
                if let Err(e) = change_wallpaper(&path) {
                    log::error!("Failed to change wallpaper: {}", e);
                }
            }
            _ => {}
        }
    }

    fn dispatch_key(&mut self, key: KeyEvent) -> Option<Action> {
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
}
