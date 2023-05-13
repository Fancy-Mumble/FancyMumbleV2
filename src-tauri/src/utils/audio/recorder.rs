use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread,
};

use tracing::{error, trace};

pub struct Recorder {
    audio_thread: Option<thread::JoinHandle<()>>,
    _queue_rx: Receiver<Vec<u8>>,
    _queue_tx: Option<Sender<Vec<u8>>>,
    playing: Arc<AtomicBool>,
}

impl Recorder {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            audio_thread: None,
            _queue_rx: rx,
            _queue_tx: Some(tx),
            playing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        if self.playing.swap(true, Ordering::Relaxed) || self.audio_thread.is_some() {
            error!("Audio thread already started");
            return Err("Audio thread already started".into());
        }

        //let audio_queue_ref = self.queue_tx.take().unwrap();
        //let playing_clone = self.playing.clone();

        self.audio_thread = Some(thread::spawn(move || {
            trace!("Starting audio thread");
            //let host = cpal::default_host();
        }));

        Ok(())
    }

    /*pub fn read_queue(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        if self.playing.load(Ordering::Relaxed) {
            //todo add user id to audio data
            return Ok(self.queue_rx.recv_timeout(Duration::from_millis(2000))?);
        }

        Err("Audio thread not started".into())
    }*/

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

impl Drop for Recorder {
    fn drop(&mut self) {
        self.stop();
    }
}
