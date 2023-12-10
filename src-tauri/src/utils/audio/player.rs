use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

use rodio::{OutputStreamHandle, Sink};
use tracing::{error, trace};

use crate::errors::AnyError;

use super::decoder::DecodedMessage;

pub struct Player {
    audio_thread: Option<thread::JoinHandle<()>>,
    queue_rx: Option<Receiver<DecodedMessage>>,
    queue_tx: Sender<DecodedMessage>,
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

            let mut sink_map = SinkMap::new(handle);

            while playing_clone.load(Ordering::Relaxed) {
                if let Ok(queue_value) = audio_queue_ref.recv_timeout(Duration::from_millis(2000)) {
                    if let Ok(sink) = sink_map.get_sink(queue_value.user_id) {
                        sink.append(rodio::buffer::SamplesBuffer::<i16>::new(
                            1,
                            48000,
                            queue_value.data,
                        ));
                    }
                }
            }
        }));

        Ok(())
    }

    pub fn add_to_queue(&mut self, data: DecodedMessage) -> AnyError<()> {
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

struct SinkMap {
    handle: OutputStreamHandle,
    sink_map: HashMap<u32, Sink>,
}

impl SinkMap {
    fn new(handle: OutputStreamHandle) -> SinkMap {
        SinkMap {
            handle,
            sink_map: HashMap::new(),
        }
    }

    fn get_sink(&mut self, user_id: u32) -> Result<&Sink, String> {
        let result = match self.sink_map.entry(user_id) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(SinkMap::create_sink(&self.handle)?),
        };

        Ok(result)
    }

    fn create_sink(handle: &OutputStreamHandle) -> Result<Sink, String> {
        let sink = rodio::Sink::try_new(handle);
        if let Err(e) = sink {
            return Err(format!("Failed to create sink: {}", e));
        }
        Ok(match sink {
            Ok(s) => s,
            Err(e) => {
                return Err(format!("Failed to create sink: {}", e));
            }
        })
    }
}
