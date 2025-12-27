use std::path::{Path, PathBuf};

use anyhow::Result;
use ratatui::{
    Frame,
    crossterm::event::{self, KeyCode},
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::{
    models::image_file::ImageFile,
    ports::{ImageRepository, WallpaperServicePort},
    tui::constant::{Action, ActionState},
};

pub struct WallpaperListComponent<'a, S, R> {
    list_item: Vec<ListItem<'a>>,
    list_state: ListState,
    images: Vec<ImageFile>,
    wallpaper_service: S,
    image_repo: R,
    monitor: String,
    path: PathBuf,
}

impl<'a, S, R> WallpaperListComponent<'a, S, R>
where
    S: WallpaperServicePort,
    R: ImageRepository,
{
    pub fn new(monitor: String, path: &Path, image_repo: R, wallpaper_service: S) -> Self {
        Self {
            list_state: ListState::default(),
            list_item: Vec::new(),
            images: Vec::new(),
            image_repo,
            wallpaper_service,
            monitor,
            path: path.to_path_buf(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        if self.list_item.is_empty() {
            self.list_item = self
                .images
                .iter()
                .map(|i| ListItem::from(i.name.clone()))
                .collect();
        }

        let wallpaper_list_widget = List::new(self.list_item.clone()).highlight_symbol("> ");
        let block_widget = Block::bordered().borders(Borders::LEFT);
        let list_area = block_widget.inner(area);

        frame.render_widget(block_widget, area);
        frame.render_stateful_widget(wallpaper_list_widget, list_area, &mut self.list_state);
    }

    pub fn execute(&mut self, action: Action) -> Result<ActionState> {
        match action {
            Action::Select(selected) => {
                if let Some(path) = selected {
                    self.change_wallpaper(&path)?;
                }
            }
            Action::Start => {
                self.list_state.select_first();
                self.images = self.image_repo.list_images(&self.path)?;
                return Ok(ActionState::NotConsumed);
            }
            Action::Next => {
                self.list_state.select_next();
                return Ok(ActionState::NotConsumed);
            }
            Action::Previous => {
                self.list_state.select_previous();
                return Ok(ActionState::NotConsumed);
            }
            _ => return Ok(ActionState::NotConsumed),
        }

        Ok(ActionState::Consumed)
    }

    pub fn handle_key(&self, key: event::KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Enter => {
                if let Some(index) = self.list_state.selected() {
                    let selected = self.images[index].path.clone();

                    return Some(Action::Select(Some(selected)));
                }

                Some(Action::Select(None))
            }
            KeyCode::Up | KeyCode::Char('k') => Some(Action::Previous),
            KeyCode::Down | KeyCode::Char('j') => Some(Action::Next),
            _ => None,
        }
    }

    fn change_wallpaper(&self, image: &Path) -> Result<()> {
        self.wallpaper_service.set_wallpaper(&self.monitor, image)
    }
}
