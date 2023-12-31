import { createSlice } from "@reduxjs/toolkit";

export enum InputMode {
    VoiceActivation = 0,
    PushToTalk = 1,
}

interface VoiceActivationOptions {
    voice_hold: number;
    fade_out_duration: number;
    voice_hysteresis_lower_threshold: number;
    voice_hysteresis_upper_threshold: number;
}

interface AudioInputSettings {
    amplification: number;
    input_mode: InputMode;
    voice_activation_options: VoiceActivationOptions;
}

const initialState: AudioInputSettings = {
    amplification: 13.0,
    input_mode: InputMode.VoiceActivation,
    voice_activation_options: {
        voice_hold: 66.0,
        fade_out_duration: 850,
        voice_hysteresis_lower_threshold: 0.03,
        voice_hysteresis_upper_threshold: 0.07

    }
};

export const frontendSettings = createSlice({
    name: 'channel',
    initialState,
    reducers: {
        updateAudioSettings: (state, action) => {
            state = action.payload;
        },
        setAmplification: (state, action) => {
            state.amplification = action.payload;
        },
        setInputMode: (state, action) => {
            state.input_mode = action.payload;
        },
        setVoiceHold(state, action) {
            state.voice_activation_options.voice_hold = action.payload
        },
        setFadeOutDuration(state, action) {
            state.voice_activation_options.fade_out_duration = action.payload
        },
        setVoiceHysteresis(state, action) {
            state.voice_activation_options.voice_hysteresis_lower_threshold = action.payload[0];
            state.voice_activation_options.voice_hysteresis_upper_threshold = action.payload[1];
        }
    },
})

export const { updateAudioSettings, setAmplification, setInputMode, setVoiceHold, setFadeOutDuration, setVoiceHysteresis } = frontendSettings.actions

export default frontendSettings.reducer