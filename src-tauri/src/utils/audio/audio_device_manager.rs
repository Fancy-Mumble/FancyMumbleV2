use std::collections::HashMap;

use rodio::{
    cpal::{self, traits::HostTrait},
    Device,
};

use crate::errors::AnyError;

struct AudioDeviceManager {
    pub audio_device: Option<Device>,
    pub audio_device_list: HashMap<u32, Device>,
}

impl AudioDeviceManager {
    pub fn new() -> Self {
        Self {
            audio_device: None,
            audio_device_list: HashMap::new(),
        }
    }

    pub fn get_audio_device(&self) -> AnyError<Vec<Device>> {
        let host = cpal::default_host();
        let devices = host.input_devices()?;

        Ok(devices.collect())
    }
}
