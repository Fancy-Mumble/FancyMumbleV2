import { createSlice } from "@reduxjs/toolkit";

export interface LinkPreviewSettings {
  enabled: boolean,
  allow_all: boolean,
  urls: string[]
}

export interface ApiKeys {
  tenor: string
}

export interface AdvancedSettings {
  disableAutoscroll: boolean;
  alwaysScrollDown: boolean;
}

export interface FrontendSettings {
  api_keys: ApiKeys;
  link_preview: LinkPreviewSettings;
  advancedSettings: AdvancedSettings
}


const initialState: FrontendSettings = {
  link_preview: {
    enabled: true,
    allow_all: false,
    urls: []
  },
  api_keys: {
    tenor: ''
  },
  advancedSettings: {
    disableAutoscroll: false,
    alwaysScrollDown: false,
  }
};

export const frontendSettings = createSlice({
  name: 'channel',
  initialState,
  reducers: {
    updateLinkPreview: (state, action) => {
      state.link_preview = action.payload;
    },
    updateApiKey: (state, action) => {
      state.api_keys = action.payload;
    },
    updateFrontendSettings: (state, action) => {
      state = action.payload;
    },
    updateAdvancedSettings: (state, action) => {
      console.log("updateAdvancedSettings: ", action.payload);
      state.advancedSettings = action.payload;
    }
  },
})

export const { updateFrontendSettings, updateLinkPreview, updateApiKey, updateAdvancedSettings } = frontendSettings.actions

export default frontendSettings.reducer