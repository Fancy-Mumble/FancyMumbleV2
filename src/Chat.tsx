import { Box, Divider, IconButton, InputBase, Paper } from '@mui/material';
import { invoke } from '@tauri-apps/api';
import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event'
import SendIcon from '@mui/icons-material/Send';
import AddToPhotosIcon from '@mui/icons-material/AddToPhotos';
import ChatMessageContainer from './components/ChatMessageContainer';
import { TextMessage } from './components/ChatMessage';
import GifIcon from '@mui/icons-material/Gif';
import GifSearch from './components/GifSearch';
import React from 'react';
import Sidebar from './Sidebar';
import { useDispatch, useSelector } from 'react-redux';
import { RootState } from './store/store';
import { updateCurrentUserById, updateUser, updateUserComment, updateUserImage } from './store/features/users/userSlice';
import { updateChannel } from './store/features/users/channelSlice';

enum MessageTypes {
    Ping = "Ping",
    TextMessage = "text_message",
    UserList = "user_list",
    UserImage = "user_image",
    UserComment = "user_comment",
    UserUpdate = "user_update",
    ChannelUpdate = "channel_update",
    NotifyCurrentUser = "current_user_id"
}

interface BackendMessage {
    message_type: MessageTypes,
    data: any
}

function Chat() {
    const [chatMessage, setChatMessage] = useState("");
    const [messageLog, setMessageLog] = useState<TextMessage[]>([]);
    const [showGifSearch, setShowGifSearch] = useState(false);
    const [gifSearchAnchor, setGifSearchAnchor] = useState<HTMLElement>();

    const dispatch = useDispatch();
    const userInfo = useSelector((state: RootState) => state.reducer.userInfo);

    useEffect(() => {

        //listen to a event
        const unlisten = listen("backend_update", (e) => {
            let message: BackendMessage = JSON.parse(e.payload as any);
            console.log("msg: ", message);

            switch (message.message_type) {
                case MessageTypes.TextMessage: {
                    addChatMessage(message.data);
                    break;
                }
                case MessageTypes.UserImage: {
                    dispatch(updateUserImage(message.data));
                    break;
                }
                case MessageTypes.UserComment: {
                    dispatch(updateUserComment(message.data));
                    break;
                }
                case MessageTypes.UserUpdate: {
                    dispatch(updateUser(message.data));
                    break;
                }
                case MessageTypes.ChannelUpdate: {
                    dispatch(updateChannel(message.data));
                    break;
                }
                case MessageTypes.NotifyCurrentUser: {
                    dispatch(updateCurrentUserById(message.data));
                    break;
                }
            }
        });

        return () => {
            unlisten.then(f => f());
        }
    });

    function addChatMessage(message: TextMessage) {
        setMessageLog(messageLog => [...messageLog, message]);
    }

    function customChatMessage(data: string) {
        invoke('send_message', { chatMessage: data, channelId: userInfo.currentUser?.channel_id });
        addChatMessage({
            actor: 0,
            sender: { user_id: 0, user_name: "test" },
            channel_id: [0],
            tree_id: [0],
            message: data,
            timestamp: Date.now()
        })
        setChatMessage("");
    }

    function sendChatMessage(e: any) {
        customChatMessage(chatMessage)
    }



    function keyDownHandler(e: React.KeyboardEvent<HTMLInputElement | HTMLTextAreaElement>) {
        if (e && e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            sendChatMessage({});
        }
    }

    function uploadFile(e: any) {
        //TODO
    }

    function showGifPreview(e: any) {
        setShowGifSearch(!showGifSearch);
        setGifSearchAnchor(e.currentTarget)
    }

    function pasteEvent(event: any) {
        let items = event.clipboardData.items;
        for (const item of items) {
            if (item.type.indexOf('image') !== -1) {
                const file = item.getAsFile();
                const reader = new FileReader();
                reader.readAsDataURL(file);
                reader.onload = function () {
                    let img = '<img src="' + reader.result + '" />';
                    customChatMessage(img);
                };
            }
        }

    }

    return (
        <Box sx={{ height: '100%', display: 'flex', flexDirection: 'row' }}>
            <Sidebar />
            <Box sx={{ flex: 1, overflowY: 'auto' }}>
                <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
                    <ChatMessageContainer messages={messageLog}></ChatMessageContainer>
                    <Box m={2} sx={{ display: 'flex' }}>
                        <Paper
                            component="form"
                            sx={{ p: '2px 4px', display: 'flex', alignItems: 'center', width: 400, flexGrow: 1 }}
                        >
                            <IconButton sx={{ p: '10px' }} aria-label="menu" onClick={uploadFile}>
                                <AddToPhotosIcon />
                            </IconButton>
                            <InputBase
                                sx={{ ml: 1, flex: 1 }}
                                placeholder="Send Message to {TO_DO}"
                                inputProps={{ 'aria-label': 'Send Message to {TO_DO}' }}
                                onChange={e => setChatMessage(e.target.value)}
                                onKeyDown={keyDownHandler}
                                value={chatMessage}
                                onPaste={pasteEvent}
                            />
                            <IconButton onClick={showGifPreview}>
                                <GifIcon />
                            </IconButton>
                            <Divider sx={{ height: 28, m: 0.5 }} orientation="vertical" />
                            <IconButton sx={{ p: '10px' }} aria-label="Send Message" onClick={sendChatMessage}>
                                <SendIcon />
                            </IconButton>
                        </Paper>
                    </Box>
                </Box>
                <GifSearch open={showGifSearch} anchor={gifSearchAnchor} />
            </Box>
        </Box>
    )
}

export default Chat;