use std::{
    ops::MulAssign,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver},
        Arc,
    },
    thread,
    time::Duration,
};

use num_traits::{NumCast, Signed};
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
            let sample_rate = microphone.device_config.as_ref().map_or(0, |config| {
                config.sample_rate.0.try_into().unwrap_or_default()
            });

            let mut va = VoiceActivation::new(sample_rate, Duration::from_millis(2000), 0.6, 0.3);

            while playing_clone.load(Ordering::Relaxed) {
                update_settings(&settings_channel, &encoder);

                let mut value = rx.recv().expect("Failed to receive audio data");
                va.process(&mut value);

                let audio_buffer = encoder.encode_audio(&value, &mut sequence_number);

                let result_buffer =
                    raw_message_builder::<UdpTunnel>(&audio_buffer).unwrap_or_default();
                if let Err(e) = audio_queue_ref.send(result_buffer) {
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

pub struct VoiceActivation<T: Signed + Copy + Default + PartialOrd + MulAssign + NumCast> {
    upper_threshold: T,
    lower_threshold: T,
    fade_out_samples: usize,
    fade_out_count: usize,
}

impl<T: Signed + Default + Copy + PartialOrd + MulAssign + NumCast> VoiceActivation<T> {
    // fade_out samples are only whole values
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn new(
        sample_rate: usize,
        fadeout_duration: Duration,
        upper_threshold: T,
        lower_threshold: T,
    ) -> Self {
        let fade_out_samples: usize =
            (fadeout_duration.as_secs_f32() * sample_rate as f32) as usize; // Number of samples for fading out the signal

        Self {
            upper_threshold,
            lower_threshold,
            fade_out_samples,
            fade_out_count: 0,
        }
    }

    pub fn process(&mut self, new_data: &mut [T]) {
        const FRAME_SIZE: usize = 160; // Size of each frame in samples

        let mut vad = Hysteresis::new(self.lower_threshold, self.upper_threshold); // Hysteresis object for the VAD logic

        // Process the input data frame by frame
        for frame in new_data.chunks_mut(FRAME_SIZE) {
            // Calculate the amplitude of the frame
            let amplitude = frame
                .iter()
                .map(Signed::abs)
                .fold(None, |max, x| {
                    max.map_or(Some(x), |max| Some(if x > max { x } else { max }))
                })
                .unwrap_or_else(T::zero); // Apply the VAD logic
            if vad.update(&amplitude) {
                // If the VAD is on, reset the fade out counter
                self.fade_out_count = 0;
            } else {
                // If the VAD is off, increment the fade out counter
                self.fade_out_count += 1;
                if self.fade_out_count > self.fade_out_samples {
                    // If the fade out counter exceeds the fade out samples, clamp it
                    self.fade_out_count = self.fade_out_samples;
                } // Apply the fade out function to the frame
                let fade_out_factor = self.calculate_fadeout(); // Calculate the fade out factor
                for x in frame.iter_mut() {
                    // Multiply each sample by the fade out factor
                    *x *= fade_out_factor;
                }
            } // Append the frame to the output queue
        }
    }

    // precision-loss is intended
    #[allow(clippy::cast_precision_loss)]
    fn calculate_fadeout(&self) -> T {
        let voice_activation_hold_offset = 0.1;
        let inverse_offset = 1.0 - voice_activation_hold_offset;

        let ratio = self.fade_out_count as f32 / self.fade_out_samples as f32;
        let adjusted_ratio = (ratio - voice_activation_hold_offset) * (1.0 / inverse_offset);
        let fade_out_factor = 1.0 - (adjusted_ratio.max(0.0).ln_1p() * inverse_offset).min(1.0);

        T::from(fade_out_factor).unwrap_or_else(T::zero)
    }
}

pub struct Hysteresis<T> {
    threshold_low: T,
    threshold_high: T,
    state: bool,
}

impl<T: PartialOrd> Hysteresis<T> {
    pub const fn new(threshold_low: T, threshold_high: T) -> Self {
        Self {
            threshold_low,
            threshold_high,
            state: false,
        }
    }

    pub fn update(&mut self, value: &T) -> bool {
        if value > &self.threshold_high {
            self.state = true;
        } else if value < &self.threshold_low {
            self.state = false;
        }
        self.state
    }
}
