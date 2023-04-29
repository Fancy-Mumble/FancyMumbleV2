use crate::connection::{Connection};
use crate::protocol::message_router::MessageRouter;
use crate::protocol::stream_reader::StreamReader;
use tokio::select;
use tokio::time;

use super::{InputThread, DEADMAN_INTERVAL, ConnectionThread};

impl InputThread for Connection {
    fn spawn_input_thread(&mut self) {
        let mut rx_in = self.tx_in.subscribe();
        let running_clone = self.running.clone();
        let message_channels = self.message_channels.clone();
        let back_channel = self.tx_out.clone();

        self.threads.insert(ConnectionThread::InputThread, tokio::spawn(async move {
            let mut interval = time::interval(DEADMAN_INTERVAL);
            let mut reader = StreamReader::new(MessageRouter::new(message_channels, back_channel));

            while *running_clone.read().unwrap() {
                select! {
                    Ok(mut result) = rx_in.recv() => {
                        reader.read_next(&mut result);
                    }

                    _ = interval.tick() => {}
                }
            }
        }));
    }
}
