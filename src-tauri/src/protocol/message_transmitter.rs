use std::error::Error;
use std::sync::{Arc, RwLock};

use crate::connection::connection_traits::{HandleMessage, Shutdown};
use async_trait::async_trait;
use tauri::Manager;
use tokio::task::JoinHandle;
use tokio::{select, sync::broadcast::Receiver};
use tracing::{debug, trace};

pub struct MessageTransmitter {
    recv_channel: Receiver<String>,
    window: tauri::Window,
    transmitter_thread: Option<JoinHandle<()>>,
    running: Arc<RwLock<bool>>,
}

impl MessageTransmitter {
    pub fn new(recv_channel: Receiver<String>, window: tauri::Window) -> MessageTransmitter {
        MessageTransmitter {
            recv_channel,
            window,
            transmitter_thread: None,
            running: Arc::new(RwLock::new(false)),
        }
    }

    //TODO: This is missing a shutdown for this channel and will cause a crash on shutdown!
    pub async fn start_message_transmit_handler(&mut self) {
        debug!("Starting MessageTransmitter");

        {
            if let Ok(mut running) = self.running.write() {
                *running = true;
            }
        }

        let mut channel = self.recv_channel.resubscribe();
        let window_clone = self.window.clone();
        let running_clone = self.running.clone();

        self.transmitter_thread = Some(tokio::spawn(async move {
            while *running_clone.read().unwrap() {
                select! {
                    Ok(result) = channel.recv() => {
                        trace!("text_message: {result}");
                        _ = window_clone.emit_all("text_message", result);
                    }
                }
            }
        }));
    }
}

#[async_trait]
impl Shutdown for MessageTransmitter {
    async fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        trace!("Sending Shutdown Request");
        if let Ok(mut running) = self.running.write() {
            *running = false;
        }

        if let Some(transmitter_thread) = self.transmitter_thread.as_mut() {
            transmitter_thread.await?;
            trace!("Joined transmitter_thread");
        }

        Ok(())
    }
}

#[async_trait]
impl HandleMessage for MessageTransmitter {}
