use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::app::Action;

pub trait UIComponent {
    fn render(&mut self, frame: &mut Frame, area: Rect);

    fn dispatch_action(&mut self, action: &Action);
    fn dispatch_key(&mut self, key: KeyEvent) -> Option<Action>;
}
