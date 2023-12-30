use std::{
    ops::MulAssign,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self},
        Arc,
    },
    thread,
    time::Duration,
};

use num_traits::{NumCast, Signed};
use tokio::sync::broadcast::{self, Receiver};
use tracing::{error, info, trace, warn};

use crate::{
    commands::utils::settings::{AudioOptions, AudioPreviewContainer, GlobalSettings, InputMode},
    errors::AnyError,
    mumble::proto::UdpTunnel,
    utils::{audio::microphone::Microphone, messages::raw_message_builder},
};

use super::encoder::Encoder;

pub struct Recorder {
    audio_thread: Option<thread::JoinHandle<()>>,
    playing: Arc<AtomicBool>,
    server_channel: Option<broadcast::Sender<Vec<u8>>>,
    settings_channel: Option<broadcast::Receiver<GlobalSettings>>,
}

impl Recorder {
    pub fn new(
        server_channel: broadcast::Sender<Vec<u8>>,
        settings_channel: broadcast::Receiver<GlobalSettings>,
    ) -> Self {
        Self {
            audio_thread: None,
            playing: Arc::new(AtomicBool::new(false)),
            server_channel: Some(server_channel),
            settings_channel: Some(settings_channel),
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

        let mut settings_channel = self
            .settings_channel
            .take()
            .ok_or("Failed to get Settings Channel, audio thread is possibly already started")?;

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

            let mut va: Option<VoiceActivation<f32>> = Some(VoiceActivation::new(
                sample_rate,
                Duration::from_millis(100),
                Duration::from_secs(1),
                0.6,
                0.3,
            ));

            let mut audio_preview: Option<AudioPreviewContainer> = None;

            while playing_clone.load(Ordering::Relaxed) {
                update_settings(
                    &mut settings_channel,
                    &mut va,
                    &microphone,
                    &mut audio_preview,
                );
                let mut max_amplitude = 0.0;

                let mut value = rx.recv().expect("Failed to receive audio data");
                if let Some(va) = va.as_mut() {
                    max_amplitude = va.process(&mut value);
                }

                if let Some(audio_preview) = audio_preview.as_mut() {
                    let _ = audio_preview.window.try_lock().map(|window| {
                        let _ = window.emit("audio_preview", max_amplitude);
                    });
                }

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

fn update_settings<T: VoiceActivationType>(
    settings_channel: &mut Receiver<GlobalSettings>,
    va: &mut Option<VoiceActivation<T>>,
    microphone: &Microphone,
    audio_settings: &mut Option<AudioPreviewContainer>,
) {
    match settings_channel.try_recv() {
        Ok(GlobalSettings::AudioInputSettings(audio_settings)) => {
            trace!("Received settings: {:?}", audio_settings);
            update_voice_activation_options(&audio_settings, va);

            let _ = microphone.volume_adjustment(audio_settings.amplification);
        }
        Ok(GlobalSettings::AudioPreview(audio_preview)) => {
            info!("Received audio preview: {:?}", audio_preview.enabled);
            if audio_preview.enabled {
                *audio_settings = Some(audio_preview);
            } else {
                *audio_settings = None;
            }
        }
        _ => {}
    }
}

// f32 to u64
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn update_voice_activation_options<T: VoiceActivationType>(
    audio_settings: &AudioOptions,
    va: &mut Option<VoiceActivation<T>>,
) {
    if audio_settings.input_mode != InputMode::VoiceActivation {
        va.take();
        return;
    }

    if let Some(va) = va.as_mut() {
        if let Some(va_options) = &audio_settings.voice_activation_options {
            va.set_durations(
                Duration::from_millis(va_options.fade_out_duration as u64),
                Duration::from_millis(va_options.voice_hold as u64),
            );
            va.set_thresholds(
                T::from(va_options.voice_hysteresis_upper_threshold).unwrap_or_else(T::zero),
                T::from(va_options.voice_hysteresis_lower_threshold).unwrap_or_else(T::zero),
            );
        }
    };
}

impl Drop for Recorder {
    fn drop(&mut self) {
        self.stop();
    }
}

pub trait VoiceActivationType:
    Signed + Copy + Default + PartialOrd + MulAssign + NumCast + core::fmt::Debug
{
}
impl<T: Signed + Copy + Default + PartialOrd + MulAssign + NumCast + core::fmt::Debug>
    VoiceActivationType for T
{
}

#[allow(clippy::struct_field_names)]
pub struct VoiceActivation<T: VoiceActivationType> {
    upper_threshold: T,
    lower_threshold: T,
    fade_out_samples: usize,
    fade_out_count: usize,
    voice_activation_hold_offset: f32,
    sample_rate: usize,
}

impl<T: VoiceActivationType> VoiceActivation<T> {
    // fade_out samples are only whole values
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn new(
        sample_rate: usize,
        fadeout_duration: Duration,
        voice_activation_hold: Duration,
        upper_threshold: T,
        lower_threshold: T,
    ) -> Self {
        let (voice_activation_hold_offset, fade_out_samples) =
            Self::calculate_voice_activation_hold_offset(
                fadeout_duration,
                voice_activation_hold,
                sample_rate,
            );

        Self {
            upper_threshold,
            lower_threshold,
            fade_out_samples,
            fade_out_count: 0,
            voice_activation_hold_offset,
            sample_rate,
        }
    }

    // truncation is needed
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    fn calculate_voice_activation_hold_offset(
        fadeout_duration: Duration,
        voice_activation_hold: Duration,
        sample_rate: usize,
    ) -> (f32, usize) {
        let total_duration = voice_activation_hold + fadeout_duration;
        let fade_out_samples: usize = (total_duration.as_secs_f32() * sample_rate as f32) as usize; // Number of samples for fading out the signal
        let voice_hold_samples: usize =
            (voice_activation_hold.as_secs_f32() * sample_rate as f32) as usize;
        (
            voice_hold_samples as f32 / fade_out_samples as f32,
            fade_out_samples,
        )
    }

    pub fn set_thresholds(&mut self, upper_threshold: T, lower_threshold: T) {
        self.upper_threshold = upper_threshold;
        self.lower_threshold = lower_threshold;
    }

    pub fn set_durations(&mut self, fadeout_duration: Duration, voice_activation_hold: Duration) {
        let (voice_activation_hold_offset, fade_out_samples) =
            Self::calculate_voice_activation_hold_offset(
                fadeout_duration,
                voice_activation_hold,
                self.sample_rate,
            );
        self.voice_activation_hold_offset = voice_activation_hold_offset;
        self.fade_out_samples = fade_out_samples;
    }

    pub fn process(&mut self, new_data: &mut [T]) -> T {
        const FRAME_SIZE: usize = 160; // Size of each frame in samples
        let mut max_amplitude: T = T::zero();

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
            max_amplitude = if amplitude > max_amplitude {
                amplitude
            } else {
                max_amplitude
            };
            if vad.update(&amplitude) {
                // If the VAD is on, reset the fade out counter
                self.fade_out_count = 0;
            } else if self.fade_out_count < self.fade_out_samples {
                // If the VAD is off, increment the fade out counter
                self.fade_out_count += FRAME_SIZE;
                if self.fade_out_count > self.fade_out_samples {
                    // If the fade out counter exceeds the fade out samples, clamp it
                    self.fade_out_count = self.fade_out_samples;
                } // Apply the fade out function to the frame
                let fade_out_factor = self.calculate_fadeout(); // Calculate the fade out factor
                for x in frame.iter_mut() {
                    // Multiply each sample by the fade out factor
                    *x *= fade_out_factor;
                }
            } else {
                frame.fill(T::zero());
            }
        }

        max_amplitude
    }

    // precision-loss is intended
    #[allow(clippy::cast_precision_loss)]
    fn calculate_fadeout(&self) -> T {
        let voice_activation_hold_offset = self.voice_activation_hold_offset;
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
