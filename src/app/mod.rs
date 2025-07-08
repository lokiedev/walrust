pub mod action;

pub use action::*;

use crate::{
    adapters::WallpaperDiskRepository,
    adapters::utils::get_home_dir,
    domain::wallpaper_service::WallpaperService,
    ports::UIComponent,
    ui::{Preview, Renderer, Selector},
};
use anyhow::{Result, anyhow};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;
use std::{error::Error, path::PathBuf, time::Duration};

const TARGET_FPS: u64 = 60;
const FRAME_DURATION_MS: u64 = 1000 / TARGET_FPS; // ~60fps

const DEFAULT_WALLPAPER_PATH: &str = "pictures/wallpapers";

type AppResult<T> = Result<T, Box<dyn Error>>;

pub struct App {
    renderer: Renderer,
    wallpaper_service: WallpaperService<WallpaperDiskRepository>,
    should_quit: bool,
}

impl App {
    pub fn new() -> AppResult<Self> {
        let wallpaper_service = WallpaperService::new(WallpaperDiskRepository::new());
        let selector = Selector::new();
        let preview = Preview::new()?;

        log::info!("App object created");

        let mut app = App {
            renderer: Renderer::new(preview, selector),
            wallpaper_service,
            should_quit: false,
        };

        app.load_wallpaper(DEFAULT_WALLPAPER_PATH)?;
        app.renderer.selector.init();

        Ok(app)
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
                self.dispatch_action(&action)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn dispatch_action(&mut self, action: &Action) -> AppResult<()> {
        match action {
            Action::Quit => self.should_quit = true,
            _ => self.renderer.dispatch_action(action),
        }

        Ok(())
    }

    #[inline]
    fn dispatch_key(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            _ => self.renderer.selector.dispatch_key(key),
        }
    }

    fn load_wallpaper(&mut self, dir_path: &str) -> Result<()> {
        let wallpaper_dir: PathBuf = get_home_dir()
            .expect("Failed to get home directory")
            .join(dir_path);
        let dir_str = wallpaper_dir
            .to_str()
            .ok_or(anyhow!("Invalid wallpaper path"))?;
        if let Ok(wallpapers) = self.wallpaper_service.get_wallpapers(dir_str) {
            self.renderer.selector.update_wallpapers(wallpapers);
        }

        Ok(())
    }
}
