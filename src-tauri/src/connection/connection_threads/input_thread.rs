use crate::connection::{Connection};
use crate::protocol::stream_reader::StreamReader;
use tokio::select;
use tokio::time;

use super::{InputThread, DEADMAN_INTERVAL};

impl InputThread for Connection {
    fn spawn_input_thread(&mut self) {
        let mut rx_in = self.tx_in.subscribe();
        let running_clone = self.running.clone();
        let message_channels = self.message_channels.clone();

        self.threads.message_thread = Some(tokio::spawn(async move {
            let mut interval = time::interval(DEADMAN_INTERVAL);
            let mut reader = StreamReader::new(message_channels);

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
