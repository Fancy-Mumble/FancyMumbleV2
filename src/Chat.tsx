import { Box, Divider, IconButton, IconButtonClasses, IconButtonTypeMap, InputBase, Paper, Skeleton } from '@mui/material';
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

function Chat() {
    const [chatMessage, setChatMessage] = useState("");
    const [messageLog, setMessageLog] = useState<TextMessage[]>([]);
    const [showGifSearch, setShowGifSearch] = useState(false);
    const [gifSearchAnchor, setGifSearchAnchor] = useState<HTMLElement>();

    useEffect(() => {
        //listen to a event
        const unlisten = listen("text_message", (e) => {
            let message: TextMessage = JSON.parse(e.payload as any);
            message.timestamp = Date.now();
            setMessageLog(messageLog => [...messageLog, message]);
        });

        return () => {
            unlisten.then(f => f());
        }
    });

    function customChatMessage(data: string) {
        invoke('send_message', { chatMessage: data });
        setChatMessage("");
    }

    function sendChatMessage(e: any) {
        customChatMessage(chatMessage)
    }



    function keyDownHandler(e: React.KeyboardEvent<HTMLInputElement | HTMLTextAreaElement>) {
        if (e && e.key === 'Enter') {
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
            <Box>
                <Skeleton animation={false} variant="rectangular" width={100} height={60} />
                <Skeleton animation={false} width={100} />
                <Skeleton animation={false} width={100} />
                <Skeleton animation={false} width={100} />
            </Box>
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

export default Chat
