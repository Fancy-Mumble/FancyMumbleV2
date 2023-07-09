import { createSlice } from "@reduxjs/toolkit";
import { deleteChannel } from "./channelSlice";
import { UserInfoState, deleteUser, updateUser } from "./userSlice";
import { addChatMessage } from "./chatMessageSlice";
import dayjs from "dayjs";


export const checkStatusChangedMiddleware =
    (storeAPI: any) =>
        (next: (arg0: any) => any) =>
            (action: { type: string; payload: any }) => {
                if (action.type === updateUser.fulfilled.type) {
                    const userState: UserInfoState = storeAPI.getState().reducer.userInfo;
                    const userId = action.payload.id;
                    const userIndex = userState?.users?.findIndex((e: { id: any; }) => e.id === userId);

                    if (userIndex === undefined || userIndex === -1)
                        return next(action);

                    const userInfo = userState.users[userIndex];

                    if (userInfo.self_deaf !== action.payload.self_deaf) {
                        storeAPI.dispatch(eventLogSlice.actions.selfDeafStatusChanged({ userId, userInfo, selfDeaf: action.payload.self_deaf }));
                    }

                    if (userInfo.self_mute !== action.payload.self_mute) {
                        storeAPI.dispatch(eventLogSlice.actions.selfMuteStatusChanged({ userId, userInfo, self_mute: action.payload.self_mute }));
                    }
                } else if(action.type === deleteUser.type) {

                }

                return next(action);
            };

interface EventLogState {
    event: string,
    logMessage: string
}

const initialState: EventLogState[] = [];

export const eventLogSlice = createSlice({
    name: 'eventLog',
    initialState, // Your initial state
    reducers: {
        selfDeafStatusChanged: (state, action) => {
            var now = dayjs().format('YYYY-MM-DD HH:mm:ss');
            // Add your event to the event log state here
            state.push({
                event: 'Self Deaf Status Change',
                logMessage: `[${now}] ${action.payload.userInfo.name} ${action.payload.selfDeaf ? 'deafened' : 'undeafened'}`,
            });
        },
        selfMuteStatusChanged: (state, action) => {
            var now = dayjs().format('YYYY-MM-DD HH:mm:ss');
            // Add your event to the event log state here
            state.push({
                event: 'Self Mute Status Change',
                logMessage: `[${now}] ${action.payload.userInfo.name} ${action.payload.self_mute ? 'muted' : 'unmuted'}`,
            });
        }
    },
});

export default eventLogSlice.reducer;