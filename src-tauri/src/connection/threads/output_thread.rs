use std::sync::{atomic::Ordering, Arc, Mutex};
use std::thread;

use crate::{connection::Connection, mumble, utils::messages::message_builder};
use tracing::{debug, error, trace};

use super::{ConnectionThread, OutputThread, DEADMAN_INTERVAL};

impl OutputThread for Connection {
    fn spawn_output_thread(&mut self) {
        if self.threads.get(&ConnectionThread::Output).is_some() {
            error!("OutputThread already running");
            return;
        }

        let tx_out = self.tx_out.clone();
        let running = Arc::clone(&self.running);
        let rx_message_channel = Arc::new(Mutex::new(self.rx_message_channel.take().unwrap()));

        let thread_handle = thread::spawn(move || {
            let interval = DEADMAN_INTERVAL;

            while running.load(Ordering::Relaxed) {
                if let Ok(result) = rx_message_channel.lock().unwrap().recv() {
                    debug!("Sending text message to channel: {:?}", result.channel_id);
                    let message = mumble::proto::TextMessage {
                        actor: None,
                        session: result.reciever.iter().copied().collect(),
                        channel_id: result.channel_id.iter().copied().collect(),
                        tree_id: Vec::new(),
                        message: result.message,
                    };
                    trace!("Sending message: {:?}", message);
                    let buffer = message_builder(&message).unwrap_or_default();

                    if let Err(error) = tx_out.send(buffer) {
                        error!("Unable to send message: {}", error);
                    }
                }

                thread::sleep(interval);
            }
        });

        self.threads.insert(ConnectionThread::Output, thread_handle);
    }
}
