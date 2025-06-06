use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    widgets::{Block, Widget},
};

use crate::core::Wallpaper;

pub struct App {
    pub items: Vec<Wallpaper>,
}

impl App {
    pub fn new() -> Self {
        App { items: Vec::new() }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame));

            // TODO: Implement key handling
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        // TODO: Implement UI
    }
}
