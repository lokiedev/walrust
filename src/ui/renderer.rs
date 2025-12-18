use crate::action::Action;
use crate::domain::ports::UIComponent;
use crate::ui::{Preview, Selector};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Rect,
    text::Line,
    widgets::Block,
};

const PREVIEW_WIDTH: u16 = 40;
const SELECTOR_WIDTH: u16 = 60;

static VERTICAL_LAYOUT: [Constraint; 1] = [Constraint::Fill(1)];
static HORIZONTAL_LAYOUT: [Constraint; 2] = [
    Constraint::Percentage(PREVIEW_WIDTH),
    Constraint::Percentage(SELECTOR_WIDTH),
];

pub struct Renderer {
    pub preview: Preview,
    pub selector: Selector,
}

impl Renderer {
    pub fn new(preview: Preview, selector: Selector) -> Self {
        Renderer { preview, selector }
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
    pub fn dispatch_action(&mut self, action: &Action) {
        match action {
            Action::NextItem => self.selector.dispatch_action(action),
            Action::PreviousItem => self.selector.dispatch_action(action),
            Action::SelectItem(_) => self.selector.dispatch_action(action),
            _ => {}
        }
    }
}
