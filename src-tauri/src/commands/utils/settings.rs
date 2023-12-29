use serde::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)]
#[derive(serde::Deserialize, Serialize, Debug)]
pub enum FrontendSettings {
    LinkPreview(LinkPreview),
    ApiKeys(ApiKeys),
    AudioInput(AudioOptions),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LinkPreview {
    enabled: Option<bool>,
    allow_all: Option<bool>,
    urls: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiKeys {
    tenor: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VoiceActivationOptions {
    pub voice_hold: f32,
    pub fade_out_duration: usize,
    pub voice_hysteresis_lower_threshold: f32,
    pub voice_hysteresis_upper_threshold: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum InputMode {
    VoiceActivation = 0,
    PushToTalk = 1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioOptions {
    pub amplification: f32,
    pub input_mode: InputMode,
    pub voice_activation_options: Option<VoiceActivationOptions>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserVoiceAdjustment {
    pub volume: f32,
    pub user_id: u32,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioOutputSettings {
    pub voice_adjustment: Vec<UserVoiceAdjustment>
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub enum GlobalSettings {
    AudioInputSettings(AudioOptions),
    AudioOutputSettings(AudioOutputSettings),
}
