import { createSlice } from "@reduxjs/toolkit";
import { invoke } from "@tauri-apps/api";

export interface LinkPreviewSettings {
  enabled: boolean,
  allow_all: boolean,
  urls: string[]
}

export interface FrontendSettings {
  linkPreview: LinkPreviewSettings
}


const initialState: FrontendSettings = {
  linkPreview: {
    enabled: true,
    allow_all: false,
    urls: []
  }
};

export const frontendSettings = createSlice({
  name: 'channel',
  initialState,
  reducers: {
    updateLinkPreview: (state, action) => {
      state.linkPreview = action.payload;
    }
  },
})

export const { updateLinkPreview } = frontendSettings.actions

export default frontendSettings.reducer