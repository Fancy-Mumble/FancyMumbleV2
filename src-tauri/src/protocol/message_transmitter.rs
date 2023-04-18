use std::error::Error;

use async_trait::async_trait;
use tauri::Manager;
use tokio::{select, sync::broadcast::Receiver};
use crate::connection::connection_traits::{Shutdown, HandleMessage};

pub struct MessageTransmitter {
    window: tauri::Window,
}

impl MessageTransmitter {
    pub fn new(window: tauri::Window) -> MessageTransmitter {
        MessageTransmitter {
            window,
        }
    }

    //TODO: This is missing a shutdown for this channel and will cause a crash on shutdown!
    pub async fn message_transmit_handler(&self) {
        //let mut channel = self.recv_channel.resubscribe();
        let window_clone = self.window.clone();

        /*tokio::spawn(async move {
            loop {
                select! {
                    Ok(result) = channel.recv() => {
                        _ = window_clone.emit_all("text_message", result);
                    }
                }
            }
        });*/
    }
}

#[async_trait]
impl Shutdown for MessageTransmitter {
    async fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[async_trait]
impl HandleMessage for MessageTransmitter {

}