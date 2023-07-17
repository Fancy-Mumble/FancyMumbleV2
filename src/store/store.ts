import { combineReducers, configureStore } from '@reduxjs/toolkit'
import userReducer from './features/users/userSlice';
import channelReducer from './features/users/channelSlice';
import chatMessageReducer from './features/users/chatMessageSlice';
import eventLogReducer, { checkStatusChangedMiddleware } from './features/users/eventLogReducer';
import frontendSettingsReducer from './features/users/frontendSettings';

const combinedReducer = combineReducers({
  channel: channelReducer,
  userInfo: userReducer,
  chatMessage: chatMessageReducer,
  frontendSettings: frontendSettingsReducer
})

const rootReducer = (state: any, action: any) => {
  if (action.type === 'logout') {
    console.log("clearing state");
    state = undefined
  }
  return combinedReducer(state, action)
}

export const store = configureStore({
  reducer: {
    reducer: rootReducer,
    eventLog: eventLogReducer
  },
  middleware: (getDefaultMiddleware) => getDefaultMiddleware().concat(checkStatusChangedMiddleware),
})

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch