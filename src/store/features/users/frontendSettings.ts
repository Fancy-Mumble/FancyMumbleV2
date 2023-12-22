import { createSlice } from "@reduxjs/toolkit";
import { invoke } from "@tauri-apps/api";

export interface LinkPreviewSettings {
  enabled: boolean,
  allow_all: boolean,
  urls: string[]
}

export interface ApiKeys {
  tenor: string
}

export interface FrontendSettings {
  api_keys: ApiKeys;
  link_preview: LinkPreviewSettings
}


const initialState: FrontendSettings = {
  link_preview: {
    enabled: true,
    allow_all: false,
    urls: []
  },
  api_keys: {
    tenor: ''
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
  },
})

export const { updateFrontendSettings, updateLinkPreview, updateApiKey } = frontendSettings.actions

export default frontendSettings.reducer