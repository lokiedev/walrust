use std::{
    collections::HashMap,
    error::Error,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use image::ImageReader;

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};
use ratatui_image::{StatefulImage, picker::Picker, protocol::StatefulProtocol};

use crate::domain::models::Wallpaper;

pub struct Preview {
    path_sender: Sender<ImageRequest>,
    image_receiver: Receiver<ImageResult>,
    pending_request: Option<String>,
    cache: HashMap<String, StatefulProtocol>,
}

type ImageResult = Result<(String, StatefulProtocol), Box<dyn Error + Send>>;
type ImageRequest = String;

impl Preview {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let (path_sender, path_receiver) = mpsc::channel::<ImageRequest>();
        let (image_sender, image_receiver) = mpsc::channel::<ImageResult>();

        Self::spawn_image_processor(path_receiver, image_sender);

        Ok(Preview {
            path_sender,
            image_receiver,
            pending_request: None,
            cache: HashMap::new(),
        })
    }

    fn spawn_image_processor(
        request_receiver: Receiver<ImageRequest>,
        result_sender: Sender<ImageResult>,
    ) {
        thread::spawn(move || {
            let picker = match Picker::from_query_stdio() {
                Ok(picker) => picker,
                Err(_) => return,
            };

            while let Ok(image_path) = request_receiver.recv() {
                log::debug!("Processing image request received");

                let result = Self::process_image_in_thread(&picker, image_path);

                if result_sender.send(result).is_err() {
                    log::error!("Failed to send image result",);

                    break;
                }
            }
        });
    }

    fn process_image_in_thread(picker: &Picker, image_path: String) -> ImageResult {
        log::debug!("Decoding image");

        let dyn_image = ImageReader::open(&image_path)
            .unwrap()
            .decode()
            .map_err(|error| Box::new(error) as Box<dyn Error + Send>)?;

        log::debug!("Creating resize protocol");

        let protocol = picker.new_resize_protocol(dyn_image);

        Ok((image_path, protocol))
    }

    pub fn render(
        &mut self,
        wallpaper: Option<&Wallpaper>,
        frame: &mut Frame,
        section: Rect,
    ) -> Result<(), Box<dyn Error>> {
        let bordered_block = Block::new()
            .borders(Borders::RIGHT)
            .title("Wallpaper Preview");

        let main_layout = bordered_block.inner(section);
        frame.render_widget(bordered_block, section);

        self.process_incoming_results();

        if let Some(wallpaper) = wallpaper {
            self.handle_wallpaper_rendering(wallpaper, frame, main_layout)?;
        }

        Ok(())
    }

    fn process_incoming_results(&mut self) {
        match self.image_receiver.try_recv() {
            Ok(Ok((path, protocol))) => {
                log::debug!("Image protocol received. Inserting cache");
                self.cache.insert(path.clone(), protocol);

                if self.pending_request.as_ref() == Some(&path) {
                    log::debug!("Pending request cleared");
                    self.pending_request = None;
                }
            }
            Ok(Err(e)) => {
                log::error!("{}", e);

                self.pending_request = None;
            }
            Err(mpsc::TryRecvError::Empty) => (),
            Err(e) => {
                log::error!("{}", e);

                self.pending_request = None;
            }
        }
    }

    fn handle_wallpaper_rendering(
        &mut self,
        wallpaper: &Wallpaper,
        frame: &mut Frame,
        layout: Rect,
    ) -> Result<(), Box<dyn Error>> {
        let image_path = &wallpaper.path;

        if let Some(cached) = self.cache.get_mut(image_path) {
            let image = StatefulImage::default();
            frame.render_stateful_widget(image, layout, cached);
            Ok(())
        } else {
            if self.pending_request.as_ref() != Some(&wallpaper.path) {
                log::debug!("Image not cached.");
                log::debug!("Requesting processing for: {}", image_path);

                let _ = self.path_sender.send(image_path.clone()).map_err(|_| {
                    log::error!("Failed to send image processing request.");
                });

                self.pending_request = Some(image_path.clone());
            }

            Ok(())
        }
    }
}
