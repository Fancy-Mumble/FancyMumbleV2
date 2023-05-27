import { createSlice } from '@reduxjs/toolkit'
import type { PayloadAction } from '@reduxjs/toolkit'


export interface UpdateableUserState {
  id?: number
  channel_id?: number,
  comment?: string,
  deaf?: boolean,
  mute?: boolean,
  name?: string,
  priority_speaker?: boolean,
  profile_picture?: string,
  recording?: boolean,
  self_deaf?: boolean,
  self_mute?: boolean,
  suppress?: boolean,
}
export interface UsersState {
  channel_id: number,
  comment: string,
  deaf: boolean,
  id: number
  mute: boolean,
  name: string,
  priority_speaker: boolean,
  profile_picture: string,
  recording: boolean,
  self_deaf: boolean,
  self_mute: boolean,
  suppress: boolean,
  talking: boolean,
  mutedSince: number | undefined,
  deafenedSince: number | undefined,
}

interface UserDataUpdate {
  user_id: number,
  data: any
}

export interface UserInfoState {
  currentUser: UsersState | undefined,
  users: UsersState[],
  connected: boolean,
}

const initialState: UserInfoState = {
  currentUser: undefined,
  users: [],
  connected: false,
};

function updateUserData(state: UsersState[], user_info: UserDataUpdate, field: string) {
  let prevList = state;
  const userIndex = prevList.findIndex(e => e.id === user_info.user_id);
  if (userIndex !== -1) {
    // @ts-ignore (field is one of the above types, but TS doesn't like this)
    state[userIndex][field] = user_info.data;
  }
}

export const userSlice = createSlice({
  name: 'user',
  initialState,
  reducers: {
    updateUserComment: (state, action: PayloadAction<UserDataUpdate>) => {
      updateUserData(state.users, action.payload, "comment");

      if (state.currentUser?.id === action.payload.user_id) {
        state.currentUser.comment = action.payload.data;
      }
    },
    updateUserImage: (state, action: PayloadAction<UserDataUpdate>) => {
      updateUserData(state.users, action.payload, "profile_picture");

      if (state.currentUser?.id === action.payload.user_id) {
        state.currentUser.profile_picture = action.payload.data;
      }
    },
    deleteUser: (state, action: PayloadAction<number>) => {
      let userId = action.payload;
      let userIndex = state.users.findIndex(e => e.id === userId);
      state.users.splice(userIndex, 1);

    },
    updateUser: (state, action: PayloadAction<UsersState>) => {
      let userId = action.payload.id;
      let userIndex = state.users.findIndex(e => e.id === userId);
      if (userIndex !== -1) {
        let muted_since = action.payload.self_mute && state.users[userIndex].self_mute !== action.payload.self_mute ? Date.now() : undefined;
        let deafened_since = action.payload.self_deaf && state.users[userIndex].self_deaf !== action.payload.self_deaf ? Date.now() : undefined;
        let profilePicture = state.users[userIndex].profile_picture;
        let comment = state.users[userIndex].comment;

        state.users[userIndex] = action.payload;
        state.users[userIndex].comment = comment;
        state.users[userIndex].profile_picture = profilePicture;
        state.users[userIndex].mutedSince = muted_since;
        state.users[userIndex].deafenedSince = deafened_since;
      } else {
        action.payload.talking = false;
        state.users.push(action.payload);
      }

      if (state.currentUser?.id === userId) {
        state.currentUser = action.payload;
      }
    },
    updateUserFromUpdateable: (state, action: PayloadAction<UpdateableUserState>) => {
      let currentUser = state.users.find(e => e.id === action.payload.id);
      if (currentUser) {
        if(action.payload.channel_id) currentUser.channel_id = action.payload.channel_id;
        if(action.payload.comment) currentUser.comment = action.payload.comment;
        if(action.payload.deaf) currentUser.deaf = action.payload.deaf;
        if(action.payload.mute) currentUser.mute = action.payload.mute;
        if(action.payload.name) currentUser.name = action.payload.name;
        if(action.payload.priority_speaker) currentUser.priority_speaker = action.payload.priority_speaker;
        if(action.payload.profile_picture) currentUser.profile_picture = action.payload.profile_picture;
        if(action.payload.recording) currentUser.recording = action.payload.recording;
        if(action.payload.self_deaf) currentUser.self_deaf = action.payload.self_deaf;
        if(action.payload.self_mute) currentUser.self_mute = action.payload.self_mute;
        if(action.payload.suppress) currentUser.suppress = action.payload.suppress;
      }
    },
    updateCurrentUserById: (state, action: PayloadAction<number>) => {
      let currentUser = state.users.find(e => e.id === action.payload);
      if (currentUser) {
        state.currentUser = currentUser;
      }
    },
    updateConnected: (state, action: PayloadAction<boolean>) => {
      state.connected = action.payload;
    },
    updateUserTalkingInfo(state, action: PayloadAction<{ user_id: number, talking: boolean }>) {
      let userIndex = state.users.findIndex(e => e.id === action.payload.user_id);
      if (userIndex !== -1) {
        state.users[userIndex].talking = action.payload.talking;
      }
    }
  },
})

// Action creators are generated for each case reducer function
export const { updateUser, deleteUser, updateUserComment, updateUserImage, updateCurrentUserById, updateConnected, updateUserTalkingInfo, updateUserFromUpdateable } = userSlice.actions

export default userSlice.reducer