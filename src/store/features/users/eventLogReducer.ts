import { createSlice } from "@reduxjs/toolkit";
import { UserInfoState, UsersState, deleteUser, updateUser } from "./userSlice";
import dayjs from "dayjs";
import { ChannelState } from "./channelSlice";

interface EventLogState {
    timestamp: number
    logMessage: string
}

// Handle function for updateUser.fulfilled
function handleUpdateUser(action: { payload: UsersState }, storeAPI: any) {
    const userState: UserInfoState = storeAPI.getState().reducer.userInfo;
    const userId = action.payload.id;
    const userInfo = getUserInfo(userState, userId);

    if (!userInfo) {
        storeAPI.dispatch(eventLogSlice.actions.dispatchEventLog({ message: `${action.payload.name} joined the server` }));
        return
    };

    if(userInfo.self_deaf !== action.payload.self_deaf && userInfo.self_mute !== action.payload.self_mute) {
        storeAPI.dispatch(eventLogSlice.actions.dispatchEventLog({ message: `${userInfo.name} ${action.payload.self_deaf ? "deafened" : "undeafened"} and ${action.payload.self_mute ? "muted" : "unmuted"}` }));
    } else if (userInfo.self_deaf !== action.payload.self_deaf) {
        storeAPI.dispatch(eventLogSlice.actions.dispatchEventLog({ message: `${userInfo.name} ${action.payload.self_deaf ? "deafened" : "undeafened"}` }));
    } else if (userInfo.self_mute !== action.payload.self_mute) {
        storeAPI.dispatch(eventLogSlice.actions.dispatchEventLog({ message: `${userInfo.name} ${action.payload.self_mute ? "muted" : "unmuted"}` }));
    }

    if (userInfo.mute !== action.payload.mute) {
        storeAPI.dispatch(eventLogSlice.actions.dispatchEventLog({ message: `${userInfo.name} ${action.payload.mute ? "was muted" : "was unmuted"} by unkonwn` }));
    }

    if (userInfo.deaf !== action.payload.deaf) {
        storeAPI.dispatch(eventLogSlice.actions.dispatchEventLog({ message: `${userInfo.name} ${action.payload.deaf ? "was deafened" : "was undeafened"} by unkonwn` }));
    }

    if (userInfo.channel_id !== action.payload.channel_id) {
        const channelInfo = findChannelById(action.payload.channel_id, storeAPI);
        const oldChannel = findChannelById(userInfo.channel_id, storeAPI);
        storeAPI.dispatch(eventLogSlice.actions.dispatchEventLog({ message: `${userInfo.name} joined ${channelInfo?.name} from ${oldChannel?.name}` }));
    }

    if(userInfo.priority_speaker !== action.payload.priority_speaker) {
        storeAPI.dispatch(eventLogSlice.actions.dispatchEventLog({ message: `${userInfo.name} ${action.payload.priority_speaker ? "is now a priority speaker" : "is no longer a priority speaker"}` }));
    }

    if(userInfo.suppress !== action.payload.suppress) {
        storeAPI.dispatch(eventLogSlice.actions.dispatchEventLog({ message: `${userInfo.name} ${action.payload.suppress ? "is now suppressed" : "is no longer suppressed"}` }));
    }

    if(userInfo.name !== action.payload.name) {
        storeAPI.dispatch(eventLogSlice.actions.dispatchEventLog({ message: `${userInfo.name} changed their name to ${action.payload.name}` }));
    }

    if(userInfo.recording !== action.payload.recording) {
        storeAPI.dispatch(eventLogSlice.actions.dispatchEventLog({ message: `${userInfo.name} ${action.payload.recording ? "started recording" : "stopped recording"}` }));
    }
}

// Handle function for deleteUser
function handleDeleteUser(action: { payload: any; }, storeAPI: any) {
    const userState: UserInfoState = storeAPI.getState().reducer.userInfo;
    const userId = action.payload;
    const userInfo = getUserInfo(userState, userId);

    if (!userInfo) return;

    storeAPI.dispatch(eventLogSlice.actions.dispatchEventLog({ message: `${userInfo.name} left the server` }));
}

// Get user info
function getUserInfo(userState: UserInfoState, userId: number) {
    const userIndex = userState?.users?.findIndex((e: { id: any; }) => e.id === userId);

    if (userIndex === undefined || userIndex === -1) return null;

    return userState.users[userIndex];
}

function findChannelById(channelId: number, storeAPI: any) {
    const channelState: ChannelState[] = storeAPI.getState().reducer.channel;
    console.log(channelState)
    const channelIndex = channelState?.findIndex((e: ChannelState) => e.channel_id === channelId);

    if (channelIndex === undefined || channelIndex === -1) return null;

    return channelState[channelIndex];
}

const actionHandlers = {
    [updateUser.fulfilled.type]: handleUpdateUser,
    [deleteUser.type]: handleDeleteUser,
};

export const checkStatusChangedMiddleware =
    (storeAPI: any) =>
        (next: (arg0: any) => any) =>
            (action: { type: string; payload: any }) => {
                // Call the correct handler based on action type
                const handler = actionHandlers[action.type];
                if (handler) handler(action, storeAPI);

                return next(action);
            };

const initialState: EventLogState[] = [];

export const eventLogSlice = createSlice({
    name: 'eventLog',
    initialState, // Your initial state
    reducers: {
        dispatchEventLog: (state, action) => {
            state.push({ timestamp: Date.now(), logMessage: `${action.payload.message}` });
        },
    },
});

export default eventLogSlice.reducer;