use crate::{
    action::Action,
    adapters::WallpaperDiskRepository,
    domain::ports::UIComponent,
    domain::services::WallpaperService,
    error::AppError,
    ui::{Preview, Selector},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::Block,
};
use std::{path::PathBuf, time::Duration};

static VERTICAL_LAYOUT: [Constraint; 1] = [Constraint::Fill(1)];
static HORIZONTAL_LAYOUT: [Constraint; 2] = [
    Constraint::Percentage(PREVIEW_WIDTH),
    Constraint::Percentage(SELECTOR_WIDTH),
];
const PREVIEW_WIDTH: u16 = 40;
const SELECTOR_WIDTH: u16 = 60;
const TARGET_FPS: u64 = 60;
const FRAME_DURATION_MS: u64 = 1000 / TARGET_FPS; // ~60fps

pub type AppResult<T> = Result<T, AppError>;

pub struct App {
    path: PathBuf,
    selector: Selector,
    preview: Preview,
    wallpaper_service: WallpaperService<WallpaperDiskRepository>,
    should_quit: bool,
}

impl App {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let wallpaper_service = WallpaperService::new(WallpaperDiskRepository::new());

        let mut app = App {
            path: path.into(),
            selector: Selector::new(),
            preview: Preview::new(),
            wallpaper_service,
            should_quit: false,
        };

        app.load_wallpaper();
        app.selector.init();

        app
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> AppResult<()> {
        while !self.should_quit {
            self.handle_events()?;

            // Render frame
            let _ = terminal.draw(|frame| self.render(frame));

            // Frame rate control
            std::thread::sleep(Duration::from_millis(FRAME_DURATION_MS));
        }

        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let content_area = Self::render_border(frame);

        let [preview_area, selector_area] =
            Layout::horizontal(HORIZONTAL_LAYOUT).areas(content_area);

        // Draw components
        self.selector.render(frame, selector_area);
        let selected_wallpaper = self.selector.get_selected_wallpaper();
        let _ = self.preview.render(selected_wallpaper, frame, preview_area);
    }

    pub fn render_border(frame: &mut Frame) -> Rect {
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
            Action::NextItem => self.selector.dispatch_action(action),
            Action::PreviousItem => self.selector.dispatch_action(action),
            Action::SelectItem(_) => self.selector.dispatch_action(action),
        }
    }

    #[inline]
    fn dispatch_key(&mut self, key: KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            _ => self.selector.dispatch_key(key),
        }
    }

    fn load_wallpaper(&mut self) {
        if let Ok(wallpapers) = self.wallpaper_service.get_wallpapers(self.path.as_path()) {
            self.selector.update_wallpapers(wallpapers);
        }
    }
}
