use crate::{
    core::{Action, Loader, change_wallpaper},
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

const POLL_TIMEOUT_MS: u64 = 33;
const PREVIEW_WIDTH_PERCENT: u16 = 40;
const SELECTOR_WIDTH_PERCENT: u16 = 60;

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
        log::info!("App object created");

        let wallpaper_dir: PathBuf = get_home_dir()?.join("pictures/wallpapers");
        let wallpaper_dir_str = wallpaper_dir
            .to_str()
            .ok_or("Failed to convert home directory path to string")?;
        let wallpapers = Loader::load_wallpaper(wallpaper_dir_str)?;

        let mut selector = Selector::new(wallpapers);

        selector.init();

        Ok(App {
            selector,
            preview: Preview::new()?,
            should_quit: false,
        })
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> AppResult<()> {
        while !self.should_quit {
            let _ = terminal.draw(|frame| self.draw(frame));

            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [main_layout_area] = Layout::vertical(VERTICAL_CONSTRAINT)
            .margin(1)
            .areas(frame.area());
        let border_block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(Line::from("Select Wallpaper").centered());

        let inner_content_area = border_block.inner(main_layout_area);
        frame.render_widget(border_block, main_layout_area);

        let [preview_area, selector_area] =
            Layout::horizontal(HORIZONTAL_CONTRAINT).areas(inner_content_area);

        self.selector.draw(frame, selector_area);

        let selected_wallpaper = self.selector.get_selected_wallpaper();
        let _ = self.preview.draw(selected_wallpaper, frame, preview_area);
    }

    fn handle_events(&mut self) -> AppResult<()> {
        if let Ok(available) = crossterm::event::poll(Duration::from_millis(POLL_TIMEOUT_MS)) {
            if !available {
                return Ok(());
            }

            if let Event::Key(key) = event::read()? {
                if let Some(action) = self.handle_key(key) {
                    self.handle_action(action)?;
                }
            }
        };

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
}

#[inline]
fn get_home_dir() -> AppResult<PathBuf> {
    std::env::home_dir().ok_or("Could not determine home directory".into())
}
