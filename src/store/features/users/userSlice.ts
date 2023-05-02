import { createSlice } from '@reduxjs/toolkit'
import type { PayloadAction } from '@reduxjs/toolkit'

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
}

interface UserDataUpdate {
  user_id: number,
  data: any
}

interface UserInfoState {
  currentUser: UsersState | undefined,
  users: UsersState[]
}

const initialState: UserInfoState = {
  currentUser: undefined,
  users: [],
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
    },
    updateUserImage: (state, action: PayloadAction<UserDataUpdate>) => {
      updateUserData(state.users, action.payload, "profile_picture");
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
        let profilePicture = state.users[userIndex].profile_picture;
        let comment = state.users[userIndex].comment;
        state.users[userIndex] = action.payload;
        state.users[userIndex].comment = comment;
        state.users[userIndex].profile_picture = profilePicture;
      } else {
        state.users.push(action.payload);
      }
    },
    updateCurrentUserById: (state, action: PayloadAction<number>) => {
      let currentUser = state.users.find(e => e.id === action.payload);
      if (currentUser) {
        state.currentUser = currentUser;
      }
    }
  },
})

// Action creators are generated for each case reducer function
export const { updateUser, deleteUser, updateUserComment, updateUserImage, updateCurrentUserById, } = userSlice.actions

export default userSlice.reducer