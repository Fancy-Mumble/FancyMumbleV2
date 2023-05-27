use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

use tracing::{error, trace};

use crate::errors::AnyError;

pub struct Player {
    audio_thread: Option<thread::JoinHandle<()>>,
    queue_rx: Option<Receiver<Vec<i16>>>,
    queue_tx: Sender<Vec<i16>>,
    playing: Arc<AtomicBool>,
}

impl Player {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            audio_thread: None,
            queue_rx: Some(rx),
            queue_tx: tx,
            playing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self) -> AnyError<()> {
        if self.playing.swap(true, Ordering::Relaxed) || self.audio_thread.is_some() {
            return Err("Audio thread already started".into());
        }

        let audio_queue_ref = self.queue_rx.take().ok_or("failed to get audio queue")?;
        let playing_clone = self.playing.clone();

        self.audio_thread = Some(thread::spawn(move || {
            trace!("Starting audio thread");

            let stream = rodio::OutputStream::try_default();
            if let Err(e) = stream {
                error!("Failed to create audio stream: {}", e);
                return;
            }

            let (_stream, handle) = match stream {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to create audio stream: {}", e);
                    return;
                }
            };
            let sink = rodio::Sink::try_new(&handle);
            if let Err(e) = sink {
                error!("Failed to create sink: {}", e);
                return;
            }
            let sink = match sink {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to create sink: {}", e);
                    return;
                }
            };

            while playing_clone.load(Ordering::Relaxed) {
                if let Ok(queue_value) = audio_queue_ref.recv_timeout(Duration::from_millis(2000)) {
                    sink.append(rodio::buffer::SamplesBuffer::<i16>::new(
                        1,
                        48000,
                        queue_value,
                    ));
                }
            }
        }));

        Ok(())
    }

    pub fn add_to_queue(&mut self, data: Vec<i16>, _user_id: u32) -> AnyError<()> {
        if self.playing.load(Ordering::Relaxed) {
            //todo add user id to audio data
            self.queue_tx.send(data)?;
        }

        Ok(())
    }

    pub fn stop(&mut self) {
        if self.playing.swap(false, Ordering::Relaxed) {
            trace!("Stopping audio thread");

            if let Some(thread) = self.audio_thread.take() {
                if let Err(e) = thread.join() {
                    error!("Failed to join audio thread: {:?}", e);
                }
            }
        }
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        self.stop();
    }
}
