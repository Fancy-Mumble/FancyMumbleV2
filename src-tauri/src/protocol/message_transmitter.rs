use tauri::Manager;
use tokio::{select, sync::broadcast::Receiver};

pub struct MessageTransmitter {
    recv_channel: Receiver<String>,
    window: tauri::Window,
}

impl MessageTransmitter {
    pub fn new(recv_channel: Receiver<String>, window: tauri::Window) -> MessageTransmitter {
        MessageTransmitter {
            recv_channel,
            window,
        }
    }

    pub async fn shutdown() {
        todo!();
    }

    //TODO: This is missing a shutdown for this channel and will cause a crash on shutdown!
    pub async fn message_transmit_handler(&self) {
        let mut channel = self.recv_channel.resubscribe();
        let window_clone = self.window.clone();

        tokio::spawn(async move {
            loop {
                select! {
                    Ok(result) = channel.recv() => {
                        _ = window_clone.emit_all("text_message", result);
                    }
                }
            }
        });
    }
}
