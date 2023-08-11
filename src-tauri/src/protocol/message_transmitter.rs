use std::sync::Mutex;
use std::sync::{mpsc::Receiver as StdReceiver, Arc, RwLock};
use std::thread;
use tauri::{Manager, Window};
use tracing::{debug, trace};

use crate::connection::threads::DEADMAN_INTERVAL;
use crate::connection::traits::{HandleMessage, Shutdown};
use crate::errors::AnyError;

pub struct MessageTransmitter {
    recv_channel: Option<Arc<Mutex<StdReceiver<String>>>>,
    window: Window,
    transmitter_thread: Option<thread::JoinHandle<()>>,
    running: Arc<RwLock<bool>>,
}

impl MessageTransmitter {
    pub fn new(recv_channel: Arc<Mutex<StdReceiver<String>>>, window: Window) -> Self {
        Self {
            recv_channel: Some(recv_channel),
            window,
            transmitter_thread: None,
            running: Arc::new(RwLock::new(false)),
        }
    }

    pub fn start_message_transmit_handler(&mut self) {
        debug!("Starting MessageTransmitter");

        {
            if let Ok(mut running) = self.running.write() {
                *running = true;
            }
        }

        let channel = self.recv_channel.take().unwrap();
        let window_clone = self.window.clone();
        let running_clone = self.running.clone();

        self.transmitter_thread = Some(thread::spawn(move || {
            let interval = DEADMAN_INTERVAL;

            while *running_clone.read().expect("Failed to get running state") {
                match channel.lock().unwrap().recv() {
                    Ok(result) => {
                        trace!("backend_update received");
                        _ = window_clone.emit_all("backend_update", result);
                    }
                    Err(_) => {}
                }

                thread::sleep(interval);
            }
        }));
    }
}

impl Shutdown for MessageTransmitter {
    fn shutdown(&mut self) -> AnyError<()> {
        trace!("Sending Shutdown Request");
        if let Ok(mut running) = self.running.write() {
            *running = false;
        }

        if let Some(transmitter_thread) = self.transmitter_thread.take() {
            transmitter_thread
                .join()
                .expect("Failed to join transmitter_thread");
            trace!("Joined transmitter_thread");
        }

        Ok(())
    }
}

impl HandleMessage for MessageTransmitter {}
