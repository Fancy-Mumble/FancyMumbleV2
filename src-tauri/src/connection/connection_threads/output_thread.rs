use crate::{connection::{Connection}, utils::messages::{mumble, message_builder}};
use tokio::select;
use tokio::time;

use super::{OutputThread, DEADMAN_INTERVAL};

impl OutputThread for Connection {
    fn spawn_output_thread(&mut self) {
        let tx_b = self.tx_out.clone();
        let running_clone = self.running.clone();
        let mut rx_message_channel = self.tx_message_channel.subscribe();

        self.threads.output_thread = Some(tokio::spawn(async move {
            let mut interval = time::interval(DEADMAN_INTERVAL);

            while *running_clone.read().unwrap() {
                select! {
                    Ok(result) = rx_message_channel.recv() => {
                        let message = mumble::proto::TextMessage {
                            actor: None,
                            session: Vec::new(),
                            channel_id: vec![60u32],
                            tree_id: Vec::new(),
                            message: result,
                        };
                        let buffer = message_builder(message);
                        _ = tx_b.send(buffer);
                    }

                    _ = interval.tick() => {}
                }
            }
        }));
    }
}