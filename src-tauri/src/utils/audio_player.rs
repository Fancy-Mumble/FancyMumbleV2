use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

use tracing::{error, trace};

pub struct AudioPlayer {
    audio_thread: Option<thread::JoinHandle<()>>,
    queue_rx: Option<Receiver<Vec<i16>>>,
    queue_tx: Sender<Vec<i16>>,
    playing: Arc<AtomicBool>,
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        let (tx, rx) = mpsc::channel();

        AudioPlayer {
            audio_thread: None,
            queue_rx: Some(rx),
            queue_tx: tx,
            playing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        if self.playing.swap(true, Ordering::Relaxed) || self.audio_thread.is_some() {
            error!("Audio thread already started");
            return Err("Audio thread already started".into());
        }

        let audio_queue_ref = self.queue_rx.take().unwrap();
        let playing_clone = self.playing.clone();

        self.audio_thread = Some(thread::spawn(move || {
            trace!("Starting audio thread");

            let stream = rodio::OutputStream::try_default();
            if let Err(e) = stream {
                error!("Failed to create audio stream: {}", e);
                return;
            }

            let (_stream, handle) = stream.unwrap();
            let sink = rodio::Sink::try_new(&handle);
            if let Err(e) = sink {
                error!("Failed to create sink: {}", e);
                return;
            }
            let sink = sink.unwrap();

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

    pub fn add_to_queue(&mut self, data: Vec<i16>, user_id: u32) -> Result<(), Box<dyn Error>> {
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

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}
