import { Box } from '@mui/material';

import ChatMessageContainer from '../components/ChatMessageContainer';

import Sidebar from '../components/Sidebar';
import { RootState } from '../store/store';
import { useSelector } from 'react-redux';
import ChatInput from '../components/ChatInput';
import { useState } from 'react';

import ChatInfoBar from '../components/ChatInfoBar';
import EventLog from '../components/EventLog';


function Chat() {
    const [showLog, setShowLog] = useState(false);

    const messageLog = useSelector((state: RootState) => state.reducer.chatMessage);

    return (
        <Box sx={{ height: '100%', display: 'flex', flexDirection: 'row' }}>
            <Sidebar />
            <Box sx={{ flex: 1, overflowY: 'auto', overflowX: 'hidden' }}>
                <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
                    <ChatInfoBar onShowLog={setShowLog} />
                    <ChatMessageContainer messages={messageLog}></ChatMessageContainer>
                    <ChatInput />
                </Box>
            </Box>
            <EventLog showLog={showLog} />
        </Box>
    )
}

export default Chat;