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

const POLL_TIMEOUT_MS: u64 = 0;
const THREAD_SLEEP_DURATION_MS: u64 = 16; // ~60fps

const PREVIEW_WIDTH_PERCENT: u16 = 40;
const SELECTOR_WIDTH_PERCENT: u16 = 60;
const DEFAULT_WALLPAPER_PATH: &str = "pictures/wallpapers";

static VERTICAL_CONSTRAINT: [Constraint; 1] = [Constraint::Fill(1)];
static HORIZONTAL_CONTRAINT: [Constraint; 2] = [
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
        // Initializing Selector component
        let wallpapers = Self::load_wallpaper(DEFAULT_WALLPAPER_PATH)?;
        let mut selector = Selector::new(wallpapers);
        selector.init();

        // Initializing Preview component
        let preview = Preview::new()?;

        log::info!("App object created");
        Ok(App {
            selector,
            preview,
            should_quit: false,
        })
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> AppResult<()> {
        // Main loop
        while !self.should_quit {
            self.handle_events()?;

            let _ = terminal.draw(|frame| self.draw(frame));

            std::thread::sleep(Duration::from_millis(THREAD_SLEEP_DURATION_MS));
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let inner_content_area = Self::draw_main_border(frame);

        let [preview_area, selector_area] =
            Layout::horizontal(HORIZONTAL_CONTRAINT).areas(inner_content_area);

        // Draw selector components
        self.selector.draw(frame, selector_area);

        // Draw preview components
        let selected_wallpaper = self.selector.get_selected_wallpaper();
        let _ = self.preview.draw(selected_wallpaper, frame, preview_area);
    }

    fn draw_main_border(frame: &mut Frame) -> ratatui::layout::Rect {
        let [main_layout_area] = Layout::vertical(VERTICAL_CONSTRAINT)
            .margin(1)
            .areas(frame.area());
        let border_block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(Line::from("Select Wallpaper").centered());
        let inner_content_area = border_block.inner(main_layout_area);

        frame.render_widget(border_block, main_layout_area);

        inner_content_area
    }

    fn handle_events(&mut self) -> AppResult<()> {
        log::debug!("Checking event poll");
        if !crossterm::event::poll(Duration::from_millis(POLL_TIMEOUT_MS))? {
            log::debug!("Event not available");
            return Ok(());
        }

        if let Event::Key(key) = event::read()? {
            log::debug!("Key {} clicked by user", &key.code);

            if let Some(action) = self.handle_key(key) {
                self.handle_action(action)?;
            }
        }

        Ok(())
    }

    fn handle_action(&mut self, action: Action) -> AppResult<()> {
        match action {
            Action::Quit => self.should_quit = true,
            Action::NextItem => {
                log::debug!("Next item action");
                self.selector.list_state.select_next();
            }
            Action::PreviousItem => {
                log::debug!("Previous item action");
                self.selector.list_state.select_previous();
            }
            Action::SelectItem(wallpaper_path) => {
                log::debug!("Select item action");
                if let Err(e) = change_wallpaper(&wallpaper_path) {
                    log::error!("{}", e);
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        // Handle global key
        match key.code {
            KeyCode::Char('q') => return Some(Action::Quit),
            // Handle other global key here
            _ => {}
        }

        // Handle specific area key
        if let Some(action) = self.selector.handle_key(key) {
            return Some(action);
        }

        None
    }

    fn load_wallpaper(dir_path: &str) -> AppResult<Vec<Wallpaper>> {
        let wallpaper_dir: PathBuf = get_home_dir()?.join(dir_path);
        let wallpaper_dir_str = wallpaper_dir
            .to_str()
            .ok_or("Failed to convert home directory path to string")?;
        Loader::load_wallpaper(wallpaper_dir_str)
    }
}
