import { deleteUser, updateConnected, updateCurrentUserById, updateUser, updateUserComment, updateUserImage, updateUserTalkingInfo } from '../store/features/users/userSlice';
import { updateChannel, updateChannelDescription } from '../store/features/users/channelSlice';
import { Event } from '@tauri-apps/api/event';
import { addChatMessage } from '../store/features/users/chatMessageSlice';
import { Dispatch } from 'react';
import { AnyAction } from '@reduxjs/toolkit';

enum MessageTypes {
    Connected = "connected",
    Disconnected = "disconnected",
    Ping = "Ping",
    TextMessage = "text_message",
    UserList = "user_list",
    UserImage = "user_image",
    UserComment = "user_comment",
    UserUpdate = "user_update",
    UserRemove = "user_remove",
    ChannelUpdate = "channel_update",
    ChannelDescription = "channel_description",
    NotifyCurrentUser = "current_user_id",
    AudioInfo = "audio_info"
}

interface BackendMessage {
    message_type: MessageTypes,
    data: any
}

export function handleBackendMessage<T>(event: Event<T>, dispatch: Dispatch<AnyAction>) {
    let message: BackendMessage = JSON.parse(event.payload as any);
    console.log("msg: ", message);

    switch (message.message_type) {
        case MessageTypes.Connected: {
            dispatch(updateConnected(true));
            break;
        }
        case MessageTypes.Disconnected: {
            dispatch(updateConnected(false));
            dispatch({ type: "logout" });
            break;
        }
        case MessageTypes.TextMessage: {
            dispatch(addChatMessage(message.data));
            break;
        }
        case MessageTypes.UserImage: {
            dispatch(updateUserImage(message.data));
            break;
        }
        case MessageTypes.UserComment: {
            // @ts-ignore (whatever is going on here)
            dispatch(updateUserComment(message.data));
            break;
        }
        case MessageTypes.UserUpdate: {
            dispatch(updateUser(message.data));
            break;
        }
        case MessageTypes.UserRemove: {
            dispatch(deleteUser(message.data));
            break;
        }
        case MessageTypes.ChannelUpdate: {
            dispatch(updateChannel(message.data));
            break;
        }
        case MessageTypes.ChannelDescription: {
            dispatch(updateChannelDescription(message.data));
            break;
        }
        case MessageTypes.NotifyCurrentUser: {
            dispatch(updateCurrentUserById(message.data));
            break;
        }
        case MessageTypes.AudioInfo: {
            dispatch(updateUserTalkingInfo(message.data));
            break;
        }
    }
}