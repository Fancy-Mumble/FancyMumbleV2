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

const initialState: UsersState[] = [];

function updateUserData(state: UsersState[], user_info: UserDataUpdate, field: string) {
  let prevList = state;

  console.log("updateUserData: ", user_info, field, prevList);
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
      updateUserData(state, action.payload, "comment");
    },
    updateUserImage: (state, action: PayloadAction<UserDataUpdate>) => {
      updateUserData(state, action.payload, "profile_picture");
    },
    deleteUser: (state, action: PayloadAction<number>) => {
      //state.value += action.payload
    },
    updateUser: (state, action: PayloadAction<UsersState>) => {
      let userId = action.payload.id;
      let userIndex = state.findIndex(e => e.id === userId);
      if (userIndex !== -1) {
        let profilePicture = state[userIndex].profile_picture;
        let comment = state[userIndex].comment;
        state[userIndex] = action.payload;
        state[userIndex].comment = comment;
        state[userIndex].profile_picture = profilePicture;
      } else {
        state.push(action.payload);
      }
    }
  },
})

// Action creators are generated for each case reducer function
export const { updateUser, deleteUser, updateUserComment, updateUserImage } = userSlice.actions

export default userSlice.reducer