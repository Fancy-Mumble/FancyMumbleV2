use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self},
        Arc,
    },
    thread,
    time::Duration,
};

use cpal::traits::DeviceTrait;
use opus::Channels;
use rodio::cpal::{self, traits::HostTrait, traits::StreamTrait};
use tokio::sync::broadcast::Sender;
use tracing::{error, trace};

use crate::{
    errors::AnyError,
    mumble::proto::UdpTunnel,
    utils::{messages::raw_message_builder, varint},
};

pub struct Recorder {
    audio_thread: Option<thread::JoinHandle<()>>,
    playing: Arc<AtomicBool>,
    server_channel: Option<Sender<Vec<u8>>>,
}

const VOLUME_ADJUSTMENT: f32 = 12.0;
const BUFFER_SIZE_USIZE: usize = 4096;
const MAXIMUM_SAMPLES_PER_TALK: u64 = 600;

impl Recorder {
    pub fn new(server_channel: Sender<Vec<u8>>) -> Self {
        Self {
            audio_thread: None,
            playing: Arc::new(AtomicBool::new(false)),
            server_channel: Some(server_channel),
        }
    }

    pub fn start(&mut self) -> AnyError<()> {
        if self.playing.swap(true, Ordering::Relaxed) || self.audio_thread.is_some() {
            error!("Audio thread already started");
            return Err("Audio thread already started".into());
        }

        let playing_clone = self.playing.clone();
        let audio_queue_ref = self
            .server_channel
            .take()
            .ok_or("failed to get audio queue")
            .expect("failed to get audio queue");

        self.audio_thread = Some(thread::spawn(move || {
            trace!("Starting audio thread");
            //let host = cpal::default_host();

            let host = cpal::default_host();
            let input_device = host
                .default_input_device()
                .expect("Failed to get default input device");

            trace!("Default input device: {:?}", input_device.name());

            let device_config = input_device
                .supported_input_configs()
                .expect("Failed to get supported input formats")
                .filter(|c| c.channels() <= 2)
                .max_by(|a, b| a.max_sample_rate().cmp(&b.max_sample_rate()))
                .expect("Failed to get max sample rate");

            let buffer_size = cpal::BufferSize::Fixed(BUFFER_SIZE_USIZE as u32);

            let config = cpal::StreamConfig {
                channels: device_config.channels(),
                sample_rate: device_config.max_sample_rate(),
                buffer_size,
            };

            trace!("Using config: {:?}", config);

            let (tx, rx) = mpsc::channel();

            let callback = move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut processed_buffer = data.to_vec();

                // Convert stereo samples to mono by averaging the left and right channels
                for (_, processed_sample) in processed_buffer.iter_mut().enumerate() {
                    *processed_sample *= VOLUME_ADJUSTMENT;
                }

                // Add the audio samples to the buffer
                tx.send(processed_buffer)
                    .expect("Failed to send audio data");
            };

            let err_fn = |err| error!("an error occurred on stream: {}", err);

            let stream = input_device
                .build_input_stream(&config, callback, err_fn, Some(Duration::from_secs(10)))
                .unwrap();
            stream.play().expect("Failed to play audio stream");

            let opus_channels = match config.channels {
                1 => Channels::Mono,
                2 => Channels::Stereo,
                _ => panic!("Unsupported channel count"),
            };

            let mut encoder = opus::Encoder::new(
                device_config.max_sample_rate().0,
                opus_channels,
                opus::Application::Voip,
            )
            .expect("Failed to create opus encoder");

            trace!("Audio thread started");
            trace!("Playing: {:?}", playing_clone.load(Ordering::Relaxed));

            let mut sequence_number = 0u64;
            while playing_clone.load(Ordering::Relaxed) {
                let value = rx.recv().expect("Failed to receive audio data");
                let output = encoder
                    .encode_vec_float(&value, BUFFER_SIZE_USIZE)
                    .expect("Failed to encode audio data");

                let mut audio_buffer = Vec::new();

                let opus_audio_codec = 4u8 << 5;
                let target = 0b0000_0000u8;
                let first_byte = opus_audio_codec | target;
                audio_buffer.push(first_byte);

                let sequence_number_bytes = varint::Builder::from(sequence_number as i128)
                    .build()
                    .expect("Failed to build sequence number");
                audio_buffer.extend(sequence_number_bytes.parsed_vec());
                sequence_number += 1;

                let mut size_pre = output.len() as i128;
                if sequence_number > MAXIMUM_SAMPLES_PER_TALK {
                    size_pre |= 1 << 14;
                    sequence_number = 0;
                }
                let size = varint::Builder::new(size_pre)
                    .minimum_bytes(2)
                    .encode_build()
                    .expect("Failed to build size");

                audio_buffer.extend(size.parsed_vec());

                audio_buffer.extend(output);

                let result_buffer = raw_message_builder::<UdpTunnel>(&audio_buffer);
                audio_queue_ref
                    .send(result_buffer)
                    .expect("Failed to send audio data");
            }
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

impl Drop for Recorder {
    fn drop(&mut self) {
        self.stop();
    }
}
