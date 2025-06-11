use crate::{
    core::{Action, Loader},
    ui::{Preview, Selector},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    text::Line,
    widgets::Block,
};
use std::{error::Error, io, path::PathBuf};

pub struct App {
    selector: Selector,
    preview: Preview,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let wallpaper_dir: PathBuf = get_home_dir()?.join("pictures/wallpapers");
        let wallpaper_dir_str = wallpaper_dir
            .to_str()
            .ok_or("Failed to convert home directory path to string")?;
        let wallpapers = Loader::wallpaper(wallpaper_dir_str)?;

        let mut selector = Selector::new(wallpapers);
        let preview = Preview::new();

        selector.init();

        Ok(App { selector, preview })
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        loop {
            let _ = terminal.draw(|frame| self.draw(frame));

            if let Event::Key(key) = event::read()? {
                let action = self.handle_key(key);

                match action {
                    Some(Action::Quit) => break,
                    Some(Action::NextItem) => self.selector.list_state.select_next(),
                    Some(Action::PreviousItem) => self.selector.list_state.select_previous(),
                    None => {}
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [main_layout_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(frame.area());
        let border_block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(Line::from("Select Wallpaper").centered());

        let inner_content_area = border_block.inner(main_layout_area);
        frame.render_widget(border_block, main_layout_area);

        let [preview_area, selector_area] =
            Layout::horizontal(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(inner_content_area);

        self.selector.draw(frame, selector_area);
        self.preview.draw(None, frame, preview_area);
    }

    fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        // Handle global key
        match key.code {
            KeyCode::Char('q') => return Some(Action::Quit),
            _ => {}
        }

        // Handle key for specific section
        if let Some(action) = self.selector.handle_key(key) {
            return Some(action);
        }

        None
    }
}

fn get_home_dir() -> Result<PathBuf, Box<dyn Error>> {
    let home_dir_path = std::env::home_dir().ok_or("Could not determine home directory")?;

    Ok(home_dir_path)
}
