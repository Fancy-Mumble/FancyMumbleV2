use prost::Message;
use std::any::Any;
use tokio::sync::broadcast::Sender;

use crate::utils::messages::mumble::proto::{TextMessage};

pub struct MessageHandler {
    sender: Sender<String>,
}

impl MessageHandler {
    pub fn new(sender: Sender<String>) -> MessageHandler {
        MessageHandler { sender }
    }

    pub fn recv_message(&self, message: Box<dyn Any>) {
        println!("Incomming: {message:?}");
        match message.downcast::<TextMessage>() {
            Ok(mut b) => {
                if let Ok(v) = serde_json::to_string(b.as_mut()) {
                    _ = self.sender.send(v);
                }
            }
            Err(e) => {
                println!("Type not yet implemented: {e:?}");
            }
        };
    }
}
