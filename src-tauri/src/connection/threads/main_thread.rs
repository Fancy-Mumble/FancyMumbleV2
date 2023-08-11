use crate::connection::{Connection, BUFFER_SIZE};
use crate::errors::application_error::ApplicationError;
use crate::errors::AnyError;
use std::cmp;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{atomic::Ordering, Arc, Mutex};
use std::thread;
use tracing::{error, trace};

use super::{ConnectionThread, MainThread, DEADMAN_INTERVAL};

pub const MAX_SEND_SIZE: usize = 8192;

impl MainThread for Connection {
    fn init_main_thread(
        &mut self,
        stream: Option<native_tls::TlsStream<TcpStream>>,
    ) -> AnyError<()> {
        if self.threads.get(&ConnectionThread::Main).is_some() {
            return Err(Box::new(ApplicationError::new(
                "MainThread already running",
            )));
        }

        let mut buffer = [0; BUFFER_SIZE];
        let mut stream = stream.ok_or("No stream found")?;
        //let mut reader = BufReader::new(writer);

        let tx_in = self.tx_in.clone();
        let rx_out = Arc::new(Mutex::new(self.rx_out.take().unwrap()));
        let running = Arc::clone(&self.running);

        let thread_handle = thread::spawn(move || {
            let interval = DEADMAN_INTERVAL;

            while running.load(Ordering::Relaxed) {
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        if size == 0 {
                            return;
                        }
                        if let Err(e) = tx_in.send(buffer[0..size].to_vec()) {
                            error!("Error while channeling incoming data: {e:?}");
                        }
                    }
                    Err(_) => {}
                }

                if let Ok(result) = rx_out.lock().unwrap().recv() {
                    if result.len() < MAX_SEND_SIZE && result[1] != 0x01 {
                        trace!("Sending to server: {result:?}");
                    }

                    let chunks = result.chunks(cmp::max(1, result.len() / MAX_SEND_SIZE));

                    for chunk in chunks {
                        if let Err(e) = stream.write_all(chunk) {
                            error!("Error while writing to socket: {:?}", e);
                            return;
                        }
                    }
                }

                thread::sleep(interval);
            }
        });

        self.threads.insert(ConnectionThread::Main, thread_handle);

        Ok(())
    }
}
