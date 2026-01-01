use std::{path::PathBuf, sync::mpsc, thread, time::Duration};

use ratatui::crossterm::event::{self, KeyEvent};
use ratatui_image::protocol::StatefulProtocol;

/*
 * Messages is used to wrap the communication channel that facilitates
 * this app flow and keyboard events.
 *
 * The name Message or Messages is chosen because I couldn't find any other name.
 * (It may get confused with crossterm::event::Event if it uses the name Events)
 */

pub enum Message {
    Key(KeyEvent),

    // This will be sent when a PreviewComponent worker finished generating an image protocol.
    // The StatefulProtocol is wrapped in a Box based on clippy suggestion
    ImagePreviewFinished(PathBuf, Box<StatefulProtocol>),
}

#[derive(PartialEq, Eq)]
pub enum MessageState {
    Consumed,
    NotConsumed,
}

impl MessageState {
    pub fn is_consumed(&self) -> bool {
        *self == MessageState::Consumed
    }
}

pub struct Messages {
    tick_rate: Duration,
    pub rx: mpsc::Receiver<Message>,
    pub tx: mpsc::Sender<Message>,
}

impl Messages {
    pub fn new(tick_rate: u64) -> Self {
        let messages_channel = mpsc::channel::<Message>();

        Messages {
            tick_rate: Duration::from_millis(tick_rate),
            rx: messages_channel.1,
            tx: messages_channel.0,
        }
    }

    pub fn start_event_listener(&mut self) {
        let tick_rate = self.tick_rate;
        let tx = self.tx.clone();

        thread::spawn(move || {
            loop {
                // I'm planning to add a support for control modifier if needed.
                if event::poll(tick_rate).unwrap()
                    && let event::Event::Key(event) = event::read().unwrap()
                {
                    tx.send(Message::Key(event)).unwrap();
                }
            }
        });
    }
}
