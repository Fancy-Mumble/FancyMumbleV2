use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver},
        Arc,
    },
    thread,
};

use tokio::sync::broadcast;
use tracing::{error, trace, warn};

use crate::{
    errors::AnyError,
    mumble::proto::UdpTunnel,
    utils::{audio::microphone::Microphone, messages::raw_message_builder},
};

use super::encoder::Encoder;

pub struct Settings {
    pub sample_rate: u32,
    pub channels: u8,
    pub frame_size: usize,
}

struct SettingsChannel {
    rx_channel: Option<mpsc::Receiver<Settings>>,
    _tx_channel: mpsc::Sender<Settings>,
}

pub struct Recorder {
    audio_thread: Option<thread::JoinHandle<()>>,
    playing: Arc<AtomicBool>,
    server_channel: Option<broadcast::Sender<Vec<u8>>>,
    settings_channel: SettingsChannel,
}

impl Recorder {
    pub fn new(server_channel: broadcast::Sender<Vec<u8>>) -> Self {
        let (tx, rx) = mpsc::channel();
        let settings_channel = SettingsChannel {
            rx_channel: Some(rx),
            _tx_channel: tx,
        };

        Self {
            audio_thread: None,
            playing: Arc::new(AtomicBool::new(false)),
            server_channel: Some(server_channel),
            settings_channel,
        }
    }

    pub fn start(&mut self) -> AnyError<()> {
        if self.audio_thread.is_some() || self.playing.swap(true, Ordering::Relaxed) {
            error!("Audio thread already started");
            return Err("Audio thread already started".into());
        }

        let playing_clone = self.playing.clone();
        let audio_queue_ref = self
            .server_channel
            .take()
            .ok_or("failed to get audio queue")
            .expect("failed to get audio queue");
        let settings_channel = self
            .settings_channel
            .rx_channel
            .take()
            .ok_or("failed to get settings channel")?;

        self.audio_thread = Some(thread::spawn(move || {
            trace!("Starting audio thread");

            let (tx, rx) = mpsc::channel();
            let mut microphone = Microphone::new(tx).expect("Failed to create microphone");
            let mut encoder = Encoder::new(microphone.config());
            match microphone.start() {
                Ok(()) => {}
                Err(e) => {
                    error!("Failed to start microphone: {}", e);
                    return;
                }
            }

            trace!("Audio thread started");
            trace!("Playing: {:?}", playing_clone.load(Ordering::Relaxed));

            let mut sequence_number = 0u64;
            while playing_clone.load(Ordering::Relaxed) {
                update_settings(&settings_channel, &encoder);

                let value = rx.recv().expect("Failed to receive audio data");

                let audio_buffer = encoder.encode_audio(&value, &mut sequence_number);

                let result_buffer =
                    raw_message_builder::<UdpTunnel>(&audio_buffer).unwrap_or_default();
                if let Err(e) = audio_queue_ref
                    .send(result_buffer) {
                        warn!("Failed to send audio data: {e}");
                    }
            }
            microphone.stop().expect("Failed to stop microphone");
        }));
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

fn update_settings(settings_channel: &Receiver<Settings>, encoder: &Encoder) {
    match settings_channel.try_recv() {
        Ok(settings) => {
            encoder.update_settings(&settings);
        }
        Err(mpsc::TryRecvError::Empty) => {}
        Err(mpsc::TryRecvError::Disconnected) => {
            error!("Failed to receive settings");
        }
    };
}

impl Drop for Recorder {
    fn drop(&mut self) {
        self.stop();
    }
}
