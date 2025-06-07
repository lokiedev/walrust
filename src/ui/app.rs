use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    text::Line,
    widgets::Block,
};

use crate::ui::{Preview, Selector};

pub struct App {
    selector: Selector,
    preview: Preview,
}

impl App {
    pub fn new() -> Self {
        let preview = Preview::new();
        let selector = Selector::new();

        App { selector, preview }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame));
            if let Event::Key(key) = event::read()? {
                match key.code {
                    _ => break,
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
        self.preview.draw(frame, preview_area);
    }
}
