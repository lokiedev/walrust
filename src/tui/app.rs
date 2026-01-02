use std::path::PathBuf;

use anyhow::{Context, Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    text::Line,
    widgets::Block,
};

use crate::{
    adapters::{ImageDiskRepository, ImageService},
    ports::WallpaperServicePort,
    tui::{
        PreviewComponent, WallpaperListComponent,
        messages::{Message, MessageState, Messages},
    },
};

pub struct App<S> {
    // Dependencies
    messages: Messages,
    wallpaper_service: S,

    // Components
    wallpaper_list_component: WallpaperListComponent,
    preview_component: PreviewComponent<ImageService>,

    // Data or states
    monitor: String,
    quit: bool,
}

impl<S> App<S>
where
    S: WallpaperServicePort,
{
    pub fn new(
        messages: Messages,
        dir_path: PathBuf,
        monitor: String,
        wallpaper_service: S,
    ) -> Result<Self> {
        let wallpaper_list_component =
            WallpaperListComponent::new(ImageDiskRepository::default(), dir_path)
                .with_context(|| "Failed to create wallpaper list component")?;
        let preview_component = PreviewComponent::new(&messages, ImageService {});

        Ok(App {
            quit: false,
            messages,
            monitor,
            wallpaper_list_component,
            preview_component,
            wallpaper_service,
        })
    }

    fn init(&mut self) -> Result<()> {
        if let Some(selected_image_path) = self.wallpaper_list_component.get_selected() {
            self.preview_component
                .init(selected_image_path.to_path_buf())
                .with_context(|| "Failed to initialize preview component")?;
        }

        Ok(())
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.init()?;

        while !self.quit {
            terminal.draw(|frame| self.render(frame))?;

            if let std::result::Result::Ok(message) = self.messages.rx.recv() {
                self.message(message)?;
            }
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

        self.preview_component.render(frame, preview_area);
    }

    fn message(&mut self, message: Message) -> Result<MessageState> {
        match message {
            Message::ImagePreviewFinished(image_path, protocol) => {
                self.preview_component
                    .insert_protocol(image_path, *protocol)?;
                Ok(MessageState::Consumed)
            }
            Message::Key(key) => self.event(key),
            Message::Resize => Ok(MessageState::Consumed),
        }
    }

    fn event(&mut self, key: KeyEvent) -> Result<MessageState> {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.quit = true;
                Ok(MessageState::Consumed)
            }
            KeyCode::Enter => {
                self.change_wallpaper()?;
                Ok(MessageState::Consumed)
            }
            _ => self.components_event(key),
        }?;

        Ok(MessageState::NotConsumed)
    }

    fn components_event(&mut self, key: event::KeyEvent) -> Result<MessageState> {
        if self.wallpaper_list_component.event(key)?.is_consumed()
            && let Some(image_path) = self.wallpaper_list_component.get_selected()
        {
            self.preview_component
                .update_image_path(image_path.to_path_buf())?;
        }

        Ok(MessageState::Consumed)
    }

    fn change_wallpaper(&self) -> Result<()> {
        if let Some(image_path) = self.wallpaper_list_component.get_selected() {
            self.wallpaper_service
                .set_wallpaper(&self.monitor, image_path)?
        }

        Ok(())
    }
}
