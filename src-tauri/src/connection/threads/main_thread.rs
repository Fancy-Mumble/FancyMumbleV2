use crate::connection::{Connection, BUFFER_SIZE};
use crate::errors::application_error::ApplicationError;
use crate::errors::AnyError;
use std::cmp;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::{atomic::Ordering, Arc, Mutex};
use std::thread;
use std::time::Duration;
use tracing::{error, trace};

use super::{ConnectionThread, MainThread};

pub const MAX_SEND_SIZE: usize = 8192;

impl MainThread for Connection {
    fn init_main_thread(&mut self, mut stream: native_tls::TlsStream<TcpStream>) -> AnyError<()> {
        if self.threads.get(&ConnectionThread::MainRead).is_some()
            || self.threads.get(&ConnectionThread::MainWrite).is_some()
        {
            return Err(Box::new(ApplicationError::new(
                "MainThread already running",
            )));
        }

        let mut buffer = [0; BUFFER_SIZE];

        let stream_arc = Arc::new(Mutex::new(stream));
        //let mut reader = BufReader::new();
        //let mut writer = stream.get_mut().try_clone()?;
        //let mut reader = BufReader::new(writer);

        let tx_in = self.tx_in.clone();
        let rx_out = Arc::new(Mutex::new(self.rx_out.take().unwrap()));
        let reader_running = Arc::clone(&self.running);
        let writer_running = Arc::clone(&self.running);

        let writer = stream_arc.clone();
        let writer_handle = thread::spawn(move || {
            while writer_running.load(Ordering::Relaxed) {
                if let Ok(result) = rx_out
                    .lock()
                    .unwrap()
                    .recv_timeout(Duration::from_millis(50))
                {
                    if result.len() < MAX_SEND_SIZE && result[1] != 0x01 {
                        trace!("Sending to server: {result:?}");
                    }

                    let chunks = result.chunks(cmp::max(1, result.len() / MAX_SEND_SIZE));

                    for chunk in chunks {
                        if let Err(e) = writer.lock().expect("error").write_all(chunk) {
                            error!("Error while writing to socket: {:?}", e);
                            return;
                        }
                    }
                }
            }
        });

        let reader = stream_arc.clone();
        let reader_handle = thread::spawn(move || {
            {
                if let Err(e) = reader
                    .lock()
                    .expect("error")
                    .get_mut()
                    .set_read_timeout(Some(Duration::from_millis(50)))
                {
                    error!("Error while setting read timeout: {:?}", e);
                    return;
                }
            }
            while reader_running.load(Ordering::Relaxed) {
                if let Ok(size) = reader.lock().expect("Failed to get").read(&mut buffer) {
                    trace!("Message from Server: {:?}", buffer[0..size].to_vec());
                    if size == 0 {
                        return;
                    }
                    if let Err(e) = tx_in.send(buffer[0..size].to_vec()) {
                        error!("Error while channeling incoming data: {e:?}");
                    }
                }
            }
        });

        self.threads
            .insert(ConnectionThread::MainRead, reader_handle);
        self.threads
            .insert(ConnectionThread::MainWrite, writer_handle);

        Ok(())
    }
}
