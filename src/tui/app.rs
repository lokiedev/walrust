use std::{path::Path, time::Duration};

use anyhow::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    text::Line,
    widgets::Block,
};

use crate::{
    adapters::{HyprctlWallpaperService, ImageDiskRepository},
    ports::WallpaperServicePort,
    tui::{
        components::WallpaperListComponent,
        constant::{Action, ActionState},
    },
};

pub struct App<'a> {
    wallpaper_list_component:
        WallpaperListComponent<'a, HyprctlWallpaperService, ImageDiskRepository>,
    monitor: String,
    quit: bool,
}

impl<'a> App<'a> {
    pub fn new(path: &Path, monitor: String) -> Self {
        let wallpaper_list_component = WallpaperListComponent::new(
            monitor.clone(),
            path,
            ImageDiskRepository::default(),
            HyprctlWallpaperService::new(),
        );

        App {
            quit: false,
            wallpaper_list_component,
            monitor,
        }
    }

    fn init(&mut self) -> Result<()> {
        self.execute(Action::Start)
    }

    pub fn run(&mut self) -> Result<()> {
        self.init()?;

        let mut terminal: DefaultTerminal = ratatui::init();

        while !self.quit {
            if let Some(message) = self.handle_event()? {
                self.execute(message)?;
            }

            terminal.draw(|frame| self.render(frame))?;
        }

        ratatui::restore();
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let [bordered_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(frame.area());

        let border_widget = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(Line::from(self.monitor.clone()).centered());

        let inner_area = border_widget.inner(bordered_area);

        let [preview_area, list_area] =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                .areas(inner_area);

        frame.render_widget(Line::from("Select wallpaper"), frame.area());
        frame.render_widget(&border_widget, bordered_area);
        self.wallpaper_list_component.render(frame, list_area);
        // TODO: Preview Component
    }

    fn execute(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Quit => {
                self.quit = true;
            }
            _ => {
                self.wallpaper_list_component.execute(action)?;
            }
        }

        Ok(())
    }

    fn handle_event(&self) -> Result<Option<Action>> {
        if event::poll(Duration::from_millis(250))?
            && let Event::Key(key) = event::read()?
            && key.kind == event::KeyEventKind::Press
        {
            return Ok(self.handle_key(key));
        }

        Ok(None)
    }

    fn handle_key(&self, key: event::KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => Some(Action::Quit),
            _ => self.wallpaper_list_component.handle_key(key),
        }
    }
}
