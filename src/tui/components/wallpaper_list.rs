use std::path::{Path, PathBuf};

use anyhow::{Ok, Result};
use ratatui::{
    Frame,
    crossterm::event::{self, KeyCode},
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::{models::image_file::ImageFile, ports::ImageRepository, tui::messages::MessageState};

pub struct WallpaperListComponent {
    // Data or states
    pub images: Vec<ImageFile>,
    list_state: ListState,
}

impl WallpaperListComponent {
    pub fn new<R: ImageRepository>(image_repository: R, dir_path: PathBuf) -> anyhow::Result<Self> {
        let images = image_repository.list_images(&dir_path)?;
        let mut list_state = ListState::default();

        list_state.select_first();

        Ok(Self { images, list_state })
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let list_item: Vec<ListItem> = self
            .images
            .iter()
            .map(|i| ListItem::from(i.name.clone()))
            .collect();
        let wallpaper_list_widget = List::new(list_item).highlight_symbol("> ");
        let block_widget = Block::bordered().borders(Borders::LEFT);

        let list_area = block_widget.inner(area);

        frame.render_widget(block_widget, area);
        frame.render_stateful_widget(wallpaper_list_widget, list_area, &mut self.list_state);
    }

    pub fn event(&mut self, key: event::KeyEvent) -> Result<MessageState> {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                self.list_state.select_next();
                Ok(MessageState::Consumed)
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.list_state.select_previous();
                Ok(MessageState::Consumed)
            }
            _ => Ok(MessageState::NotConsumed),
        }
    }

    pub fn get_selected(&self) -> Option<&Path> {
        self.list_state
            .selected()
            .and_then(|index| self.images.get(index))
            .map(|image_file| image_file.path.as_path())
    }
}
