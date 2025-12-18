pub mod action;

pub use action::*;

use crate::{
    adapters::WallpaperDiskRepository,
    domain::ports::UIComponent,
    domain::services::WallpaperService,
    ui::{Preview, Renderer, Selector},
};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;
use std::{path::PathBuf, time::Duration};
use thiserror::Error;

const TARGET_FPS: u64 = 60;
const FRAME_DURATION_MS: u64 = 1000 / TARGET_FPS; // ~60fps

type AppResult<T> = Result<T, WalrustError>;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum WalrustError {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
}

pub struct App {
    path: PathBuf,
    renderer: Renderer,
    wallpaper_service: WallpaperService<WallpaperDiskRepository>,
    should_quit: bool,
}

impl App {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let wallpaper_service = WallpaperService::new(WallpaperDiskRepository::new());
        let selector = Selector::new();
        let preview = Preview::new();

        let mut app = App {
            path: path.into(),
            renderer: Renderer::new(preview, selector),
            wallpaper_service,
            should_quit: false,
        };

        app.load_wallpaper();
        app.renderer.selector.init();

        app
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> AppResult<()> {
        while !self.should_quit {
            self.handle_events()?;

            // Render frame
            let _ = terminal.draw(|frame| self.renderer.render(frame));

            // Frame rate control
            std::thread::sleep(Duration::from_millis(FRAME_DURATION_MS));
        }

        Ok(())
    }

    #[inline]
    fn handle_events(&mut self) -> AppResult<()> {
        if !crossterm::event::poll(Duration::ZERO)? {
            return Ok(());
        }

        if let Event::Key(key) = event::read()? {
            log::debug!("Key '{}' pressed", &key.code);

            if let Some(action) = self.dispatch_key(key) {
                self.dispatch_action(&action);
            }
        }

        Ok(())
    }

    #[inline]
    fn dispatch_action(&mut self, action: &Action) {
        match action {
            Action::Quit => self.should_quit = true,
            _ => self.renderer.dispatch_action(action),
        }
    }

    #[inline]
    fn dispatch_key(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            _ => self.renderer.selector.dispatch_key(key),
        }
    }

    fn load_wallpaper(&mut self) {
        if let Ok(wallpapers) = self.wallpaper_service.get_wallpapers(self.path.as_path()) {
            self.renderer.selector.update_wallpapers(wallpapers);
        }
    }
}
