use async_trait::async_trait;

use crate::connection::{Connection, BUFFER_SIZE};
use crate::errors::application_error::ApplicationError;

use super::{ConnectionThread, MainThread};
use std::cmp;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::select;
use tracing::{error, trace};

const MAX_SEND_SIZE: usize = 1024;

#[async_trait]
impl MainThread for Connection {
    async fn init_main_thread(
        &mut self,
        stream: Option<tokio_native_tls::TlsStream<TcpStream>>,
    ) -> Result<(), Box<dyn Error>> {
        if self.threads.get(&ConnectionThread::MainThread).is_some() {
            return Err(Box::new(ApplicationError::new("MainThread already running")));
        }

        let mut buffer = [0; BUFFER_SIZE];
        let (reader, mut writer) = tokio::io::split(stream.unwrap());
        let mut reader = BufReader::new(reader);

        let tx_in = self.tx_in.clone();
        let mut rx_out = self.tx_out.subscribe();
        let running_clone = self.running.clone();

        self.threads.insert(
            ConnectionThread::MainThread,
            tokio::spawn(async move {
                while *running_clone.read().unwrap() {
                    select! {
                        Ok(size) = reader.read(&mut buffer) => {
                            if size == 0 {
                                return;
                            }

                            if let Err(e) = tx_in.send((&buffer[0..size]).to_vec()) {
                                error!("Error while channeling incomming data: {e:?}");
                            }
                        }
                        Ok(result) = rx_out.recv() => {
                            if result.len() < MAX_SEND_SIZE {
                                trace!("Sending to server: {result:?}");
                            }

                            let chunks = result.chunks(cmp::max(1, result.len() / MAX_SEND_SIZE));
                            for chunk in chunks {
                                if let Err(e) = writer.write(&chunk).await {
                                    error!("Error while writing to socket: {:?}", e);
                                    return;
                                }
                            }
                        }
                    }
                }
            }),
        );

        Ok(())
    }
}
