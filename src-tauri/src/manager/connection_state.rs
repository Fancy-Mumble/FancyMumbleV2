use serde::Serialize;
use tracing::error;

use crate::protocol::serialize::message_container::FrontendMessage;

use std::sync::mpsc::Sender as StdSender;

pub struct Manager {
    frontend_channel: StdSender<String>,
    _server_channel: StdSender<Vec<u8>>,
}

impl Manager {
    pub fn new(send_to: StdSender<String>, server_channel: StdSender<Vec<u8>>) -> Self {
        Self {
            frontend_channel: send_to,
            _server_channel: server_channel,
        }
    }

    fn send_to_frontend<T: Serialize + Clone>(&self, msg: &FrontendMessage<T>) {
        match serde_json::to_string(&msg) {
            Ok(json) => {
                if let Err(e) = self.frontend_channel.send(json) {
                    error!("Failed to send user list to frontend: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to serialize user list: {}", e);
            }
        }
    }

    pub fn notify_disconnected(&self, message: &Option<String>) {
        let msg = FrontendMessage::new("disconnected", message);

        self.send_to_frontend(&msg);
    }

    pub fn notify_connected(&self) {
        let msg = FrontendMessage::new("connected", &());

        self.send_to_frontend(&msg);
    }
}
