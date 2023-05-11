use std::sync::atomic::Ordering;

use crate::connection::Connection;
use crate::protocol::message_router::MessageRouter;
use crate::protocol::stream_reader::StreamReader;
use tokio::select;
use tokio::time;

use super::{ConnectionThread, InputThread, DEADMAN_INTERVAL};

impl InputThread for Connection {
    fn spawn_input_thread(&mut self) {
        let mut rx_in = self.tx_in.subscribe();
        let running = self.running.clone();
        let message_channels = self.message_channels.clone();
        let back_channel = self.tx_out.clone();

        let reader_copy = self.stream_reader.clone();
        self.threads.insert(
            ConnectionThread::Input,
            tokio::spawn(async move {
                let mut interval = time::interval(DEADMAN_INTERVAL);
                {
                    let mut reader = reader_copy.lock().await;
                    *reader = Some(StreamReader::new(MessageRouter::new(
                        message_channels,
                        back_channel,
                    )));
                }

                while running.load(Ordering::Relaxed) {
                    select! {
                        Ok(mut result) = rx_in.recv() => {
                            let mut reader = reader_copy.lock().await;
                            if let Some(reader) = reader.as_mut() {
                                reader.read_next(&mut result);
                            }
                        }

                        _ = interval.tick() => {}
                    }
                }
            }),
        );
    }
}
