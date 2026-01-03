use std::{
    num::NonZeroUsize,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use anyhow::Result;
use lru::LruCache;
use ratatui::{Frame, layout::Rect, widgets::Block};
use ratatui_image::{StatefulImage, picker::Picker, protocol::StatefulProtocol};

use crate::{
    ports::ImageServicePort,
    tui::messages::{Message, Messages},
};

/*
 * PreviewComponent render the image preview using ratatui-image crate.
 * Decoding the image and making the StatefulProtocol is slow,
 * so Walrust do it in another thread.
 */

pub struct PreviewComponent<A> {
    // Dependencies
    image_service: A, // I'm keeping this because it might be useful later

    // Data or states
    image_path: PathBuf,
    protocols: LruCache<PathBuf, StatefulProtocol>, // Protocol cache

    // Concurrency
    image_path_tx: Sender<PathBuf>,
    pending_image_preview: Option<PathBuf>,
}

impl<A> PreviewComponent<A>
where
    A: ImageServicePort + Clone + Send + 'static,
{
    pub fn new(picker: Picker, messages: &Messages, image_service: A) -> Result<Self> {
        let image_path_channel = mpsc::channel::<PathBuf>();

        Self::spawn_path_request_listener(
            image_service.clone(),
            image_path_channel.1,
            messages.tx.clone(),
            picker,
        )?;

        Ok(PreviewComponent {
            image_service,
            image_path: PathBuf::new(),
            protocols: LruCache::new(NonZeroUsize::new(8).unwrap()),
            image_path_tx: image_path_channel.0,
            pending_image_preview: None,
        })
    }

    pub fn init(&mut self, selected_image_path: PathBuf) -> Result<()> {
        self.update_image_path(selected_image_path)?;

        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block_widget = Block::new().title("Preview");
        let block_inner_area = block_widget.inner(area);

        frame.render_widget(block_widget, area);

        if let Some(protocol) = self.protocols.get_mut(&self.image_path) {
            frame.render_stateful_widget(StatefulImage::new(), block_inner_area, protocol);
        }
    }

    pub fn insert_protocol(
        &mut self,
        image: PathBuf,
        value: StatefulProtocol,
    ) -> anyhow::Result<()> {
        self.protocols.put(image, value);

        Ok(())
    }

    pub fn update_image_path(&mut self, new_image_path: PathBuf) -> Result<()> {
        if self.image_path == new_image_path {
            return Ok(());
        }

        if !self.protocols.contains(&new_image_path)
            && self.pending_image_preview.as_ref() != Some(&new_image_path)
        {
            self.image_path_tx.send(new_image_path.clone())?;
        }

        self.image_path = new_image_path;
        Ok(())
    }

    // TODO:
    // I'm planning to add a compression mechanism.
    // Currently this method decodes the actual image (not a compressed one)
    // which I think takes a lot of memory
    // especially for image with high resolution.
    fn spawn_path_request_listener(
        image_service: A,
        image_path_rx: Receiver<PathBuf>,
        image_preview_tx: Sender<Message>,
        picker: Picker,
    ) -> anyhow::Result<()> {
        thread::spawn(move || {
            while let Ok(image_path) = image_path_rx.recv() {
                let dyn_image = image_service.decode(&image_path);
                if dyn_image.is_err() {
                    break;
                }

                let protocol = picker.new_resize_protocol(dyn_image.unwrap());

                image_preview_tx
                    .send(Message::ImagePreviewFinished(
                        image_path,
                        Box::new(protocol),
                    ))
                    .unwrap();
            }
        });

        Ok(())
    }
}
