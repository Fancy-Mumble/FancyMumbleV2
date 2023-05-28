use std::{
    borrow::BorrowMut,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

use cpal::traits::DeviceTrait;
use opus::Channels;
use rodio::cpal::{self, traits::HostTrait, traits::StreamTrait};
use tracing::{error, trace, warn};

use crate::errors::AnyError;

pub struct Recorder {
    audio_thread: Option<thread::JoinHandle<()>>,
    _queue_rx: Receiver<Vec<u8>>,
    _queue_tx: Option<Sender<Vec<u8>>>,
    playing: Arc<AtomicBool>,
    sample_rate: u32,
    channels: opus::Channels,
}

impl Recorder {
    pub fn new(sample_rate: u32, channels: opus::Channels) -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            audio_thread: None,
            _queue_rx: rx,
            _queue_tx: Some(tx),
            playing: Arc::new(AtomicBool::new(false)),
            sample_rate,
            channels,
        }
    }

    pub fn start(&mut self) -> AnyError<()> {
        if self.playing.swap(true, Ordering::Relaxed) || self.audio_thread.is_some() {
            error!("Audio thread already started");
            return Err("Audio thread already started".into());
        }

        //let audio_queue_ref = self.queue_tx.take().unwrap();
        //let playing_clone = self.playing.clone();

        /*let sample_rate = cpal::SampleRate(self.sample_rate);
        let channels = match self.channels {
            opus::Channels::Mono => 1,
            opus::Channels::Stereo => 2,
        };*/
        let playing_clone = self.playing.clone();

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

            const buffer_size_usize: usize = 4096;
            let buffer_size = cpal::BufferSize::Fixed(buffer_size_usize as u32);

            let config = cpal::StreamConfig {
                channels: device_config.channels(),
                sample_rate: device_config.max_sample_rate(),
                buffer_size,
            };

            let (tx, rx) = mpsc::channel();

            let callback = move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // Add the audio samples to the buffer
                tx.send(data.to_vec()).expect("Failed to send audio data");
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
            while playing_clone.load(Ordering::Relaxed) {
                let value = rx.recv().expect("Failed to receive audio data");
                let output = encoder
                    .encode_vec_float(&value, buffer_size_usize)
                    .expect("Failed to encode audio data");

                trace!("Sending audio data to queue: {:?}", output);
            }
        }));

        Ok(())
    }

    /*pub fn read_queue(&mut self) -> AnyError<Vec<u8>> {
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
