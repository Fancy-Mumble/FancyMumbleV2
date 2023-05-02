use serde::Serialize;
use tracing::error;

use crate::protocol::serialize::message_container::FrontendMessage;

use tokio::sync::broadcast::Sender;

pub struct ConnectionManager {
    frontend_channel: Sender<String>,
    _server_channel: Sender<Vec<u8>>,
}

impl ConnectionManager {
    pub fn new(send_to: Sender<String>, server_channel: Sender<Vec<u8>>) -> ConnectionManager {
        ConnectionManager {
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
