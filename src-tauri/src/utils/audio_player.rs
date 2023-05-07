use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
};

use tracing::{error, trace};

pub struct AudioPlayer {
    audio_thread: Option<thread::JoinHandle<()>>,
    audio_queue: Arc<Mutex<Vec<i16>>>,
    playing: Arc<RwLock<bool>>,
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        AudioPlayer {
            audio_thread: None,
            audio_queue: Arc::new(Mutex::new(Vec::new())),
            playing: Arc::new(RwLock::new(false)),
        }
    }

    pub fn start(&mut self) {
        if self.audio_thread.is_some() {
            error!("Audio thread already started");
            return;
        }

        let audio_queue_ref = self.audio_queue.clone();
        let playing_clone = self.playing.clone();

        self.audio_thread = Some(thread::spawn(move || {
            trace!("Starting audio thread");

            let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
            let sink = rodio::Sink::try_new(&handle);
            if sink.is_err() {
                error!("Failed to create sink: {}", sink.err().unwrap());
                return;
            }
            let sink = sink.unwrap();

            while *playing_clone.read().unwrap() {
                let current_queue = audio_queue_ref.lock();
                if let Ok(mut current_queue) = current_queue {
                    if current_queue.is_empty() {
                        thread::sleep(std::time::Duration::from_millis(2));
                        continue;
                    }
                    let data = current_queue.drain(..).collect::<Vec<_>>();
                    trace!("Playing audio: {:?}", data.len());
                    sink.append(rodio::buffer::SamplesBuffer::<i16>::new(2, 48000, data));
                    sink.sleep_until_end();
                }
            }
        }));
    }

    pub fn add_to_queue(&mut self, data: &Vec<i16>) {
        if let Ok(mut audio_queue) = self.audio_queue.lock() {
            audio_queue.extend(data);
        }
    }

    pub fn stop(&mut self) {
        trace!("Stopping audio thread");
        if let Ok(mut playing) = self.playing.write() {
            *playing = false;
        }

        if let Some(thread) = self.audio_thread.take() {
            if let Err(e) = thread.join() {
                error!("Failed to join audio thread: {:?}", e);
            }
        }
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}
