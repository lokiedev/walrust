use crate::{
    core::{Action, Loader, Wallpaper, change_wallpaper, get_home_dir},
    ui::{Preview, Selector},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    text::Line,
    widgets::Block,
};
use std::{error::Error, path::PathBuf, time::Duration};

const TARGET_FPS: u64 = 60;
const FRAME_DURATION_MS: u64 = 1000 / TARGET_FPS; // ~60fps

const PREVIEW_WIDTH_PERCENT: u16 = 40;
const SELECTOR_WIDTH_PERCENT: u16 = 60;
const DEFAULT_WALLPAPER_PATH: &str = "pictures/wallpapers";

static VERTICAL_LAYOUT: [Constraint; 1] = [Constraint::Fill(1)];
static HORIZONTAL_LAYOUT: [Constraint; 2] = [
    Constraint::Percentage(PREVIEW_WIDTH_PERCENT),
    Constraint::Percentage(SELECTOR_WIDTH_PERCENT),
];

type AppResult<T> = Result<T, Box<dyn Error>>;

pub struct App {
    selector: Selector,
    preview: Preview,
    should_quit: bool,
}

impl App {
    pub fn new() -> AppResult<Self> {
        // Create and initialize selector component
        let wallpapers = Self::load_wallpaper(DEFAULT_WALLPAPER_PATH)?;
        let mut selector = Selector::new(wallpapers);
        selector.init();

        // Create preview component
        let preview = Preview::new()?;

        log::info!("App object created");
        Ok(App {
            selector,
            preview,
            should_quit: false,
        })
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> AppResult<()> {
        // Application main loop
        while !self.should_quit {
            self.handle_events()?;

            let _ = terminal.draw(|frame| self.draw(frame));

            // Limit loop speed
            std::thread::sleep(Duration::from_millis(FRAME_DURATION_MS));
        }

        Ok(())
    }

    // Main draw method (each UI component must has this method)
    fn draw(&mut self, frame: &mut Frame) {
        let content_area = Self::draw_border(frame);

        let [preview_area, selector_area] =
            Layout::horizontal(HORIZONTAL_LAYOUT).areas(content_area);

        // Draw components
        self.selector.draw(frame, selector_area);
        let selected_wallpaper = self.selector.get_selected_wallpaper();
        let _ = self.preview.draw(selected_wallpaper, frame, preview_area);
    }

    fn draw_border(frame: &mut Frame) -> ratatui::layout::Rect {
        let [layout_area] = Layout::vertical(VERTICAL_LAYOUT)
            .margin(1)
            .areas(frame.area());

        let border = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(Line::from("Select Wallpaper").centered());
        let content_area = border.inner(layout_area);

        frame.render_widget(border, layout_area);

        content_area
    }

    fn handle_events(&mut self) -> AppResult<()> {
        if !crossterm::event::poll(Duration::ZERO)? {
            return Ok(());
        }

        if let Event::Key(key) = event::read()? {
            log::debug!("Key '{}' pressed", &key.code);

            if let Some(action) = self.handle_key(key) {
                self.handle_action(action)?;
            }
        }

        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> AppResult<()> {
        match action {
            Action::Quit => self.should_quit = true,
            Action::NextItem => self.selector.list_state.select_next(),
            Action::PreviousItem => self.selector.list_state.select_previous(),
            Action::SelectItem(path) => {
                if let Err(e) = change_wallpaper(&path) {
                    log::error!("Failed to change wallpaper: {}", e);
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            _ => self.selector.handle_key(key),
        }
    }

    fn load_wallpaper(dir_path: &str) -> AppResult<Vec<Wallpaper>> {
        let wallpaper_dir: PathBuf = get_home_dir()?.join(dir_path);
        let dir_str = wallpaper_dir.to_str().ok_or("Invalid wallpaper path")?;
        Loader::load_wallpaper(dir_str)
    }
}
