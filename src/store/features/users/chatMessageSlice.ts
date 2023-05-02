import { PayloadAction, createSlice } from "@reduxjs/toolkit";

export interface SenderInfo {
    user_id: number,
    user_name: string,
}
export interface TextMessage {
    // The message sender, identified by its session.
    actor: number,
    sender: SenderInfo,
    // The channels to which the message is sent, identified by their
    // channel_ids.
    channel_id: number[]
    // The root channels when sending message recursively to several channels,
    // identified by their channel_ids.
    tree_id: number[]
    // The UTF-8 encoded message. May be HTML if the server allows.
    message: string,
    // custom property to keep track of time
    timestamp: number
}


const initialState: TextMessage[] = [];

export const chatMessageSlice = createSlice({
    name: 'channel',
    initialState,
    reducers: {
        addChatMessage: (state, action: PayloadAction<TextMessage>) => {
            console.log("addChatMessage: ", action.payload);
            state.push(action.payload);
        },
    },
})

export const { addChatMessage } = chatMessageSlice.actions

export default chatMessageSlice.reducer