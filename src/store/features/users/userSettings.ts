import { createSlice } from "@reduxjs/toolkit";


export interface UserSettings {
}


const initialState: UserSettings[] = [];

export const userSettings = createSlice({
  name: 'channel',
  initialState,
  reducers: {

  },
})

export const { } = userSettings.actions

export default userSettings.reducer