use std::sync::{atomic::Ordering, Arc};
use std::thread;

use crate::connection::Connection;
use crate::protocol::message_router::MessageRouter;
use crate::protocol::stream_reader::StreamReader;

use super::{ConnectionThread, InputThread, DEADMAN_INTERVAL};

impl InputThread for Connection {
    fn spawn_input_thread(&mut self) {
        let rx_in = self.rx_in.take().unwrap();
        let running = Arc::clone(&self.running);
        let message_channels = self.message_channels.tx_message_channel.clone();
        let back_channel = self.tx_out.clone();
        let reader_copy = Arc::clone(&self.stream_reader);

        let thread_handle = thread::spawn(move || {
            let interval = DEADMAN_INTERVAL;

            {
                let mut reader = reader_copy.lock().unwrap();
                let message_reader = MessageRouter::new(message_channels.clone(), back_channel);

                match message_reader {
                    Ok(message_reader) => {
                        *reader = Some(StreamReader::new(message_reader));
                    }
                    Err(e) => {
                        eprintln!("Failed to create message reader: {e}");
                    }
                }
            }

            while running.load(Ordering::Relaxed) {
                match rx_in.recv() {
                    Ok(mut result) => {
                        let mut reader = reader_copy.lock().unwrap();
                        if let Some(reader) = reader.as_mut() {
                            reader.read_next(&mut result);
                        }
                    }
                    _ => {}
                }

                thread::sleep(interval);
            }
        });

        self.threads.insert(ConnectionThread::Input, thread_handle);
    }
}
