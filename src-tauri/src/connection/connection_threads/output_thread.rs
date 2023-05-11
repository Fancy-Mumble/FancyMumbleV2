use std::sync::atomic::Ordering;

use crate::{
    connection::Connection,
    utils::messages::{message_builder, mumble},
};
use tokio::select;
use tokio::time;
use tracing::{debug, error};

use super::{ConnectionThread, OutputThread, DEADMAN_INTERVAL};

impl OutputThread for Connection {
    fn spawn_output_thread(&mut self) {
        if self.threads.get(&ConnectionThread::OutputThread).is_some() {
            error!("OutputThread already running");
            return;
        }

        let tx_out = self.tx_out.clone();
        let running = self.running.clone();
        let mut rx_message_channel = self.tx_message_channel.subscribe();

        self.threads.insert(
            ConnectionThread::OutputThread,
            tokio::spawn(async move {
                let mut interval = time::interval(DEADMAN_INTERVAL);

                while running.load(Ordering::Relaxed) {
                    select! {
                        Ok(result) = rx_message_channel.recv() => {
                            debug!("Sending text message to channel: {:?}", result.channel_id);
                            let message = mumble::proto::TextMessage {
                                actor: None,
                                session: Vec::new(),
                                channel_id: vec![result.channel_id.unwrap_or(0)],
                                tree_id: Vec::new(),
                                message: result.message,
                            };
                            let buffer = message_builder(message);

                            if let Err(error) = tx_out.send(buffer) {
                                error!("Unable to send message: {}", error);
                            }
                        }

                        _ = interval.tick() => {}
                    }
                }
            }),
        );
    }
}
