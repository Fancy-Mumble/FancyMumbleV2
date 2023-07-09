import { Box, IconButton, Paper, Tooltip } from '@mui/material';

import ChatMessageContainer from '../components/ChatMessageContainer';

import Sidebar from '../components/Sidebar';
import { RootState } from '../store/store';
import { useSelector } from 'react-redux';
import ChatInput from '../components/ChatInput';
import { useEffect, useMemo, useState } from 'react';
import AutoStoriesIcon from '@mui/icons-material/AutoStories';
import KeyboardDoubleArrowRightIcon from '@mui/icons-material/KeyboardDoubleArrowRight';


function Chat() {
    const [showLog, setShowLog] = useState(false);

    const messageLog = useSelector((state: RootState) => state.reducer.chatMessage);
    const currentChannelId = useSelector((state: RootState) => state.reducer.userInfo.currentUser?.channel_id);
    const channelInfo = useSelector((state: RootState) => state.reducer.channel.find(e => e.channel_id === currentChannelId));
    const eventLog = useSelector((state: RootState) => state.eventLog);

    const eventLogBox = useMemo(() => {
        if (!showLog) return null;

        return (
            <Box sx={{
                maxWidth: '300px',
                display: 'flex',
                flexDirection: 'column'
            }}>
                <Paper elevation={0} sx={{ flexGrow: 1 }}>
                    <Box sx={{ display: 'flex', flexDirection: 'row', alignItems: 'flex-start', height: '100%' }}>
                        <Box sx={{ flexGrow: 1, paddingLeft: 1 }}>
                            {eventLog.map((e, i) => {
                                return (
                                    <Box key={i} sx={{ display: 'flex', flexDirection: 'row', alignItems: 'center' }}>
                                        <Box sx={{ flexGrow: 1, fontSize: '0.75rem' }}>
                                            {e.logMessage}
                                        </Box>
                                    </Box>
                                )
                            })}
                        </Box>
                    </Box>
                </Paper>
            </Box>
        )
    }, [showLog, eventLog]);

    const eventLogIcon = useMemo(() => {
        if (!showLog) return (<AutoStoriesIcon sx={{ fontSize: 20 }} />);
        else return (<KeyboardDoubleArrowRightIcon sx={{ fontSize: 20 }} />);
    }, [showLog]);

    return (
        <Box sx={{ height: '100%', display: 'flex', flexDirection: 'row' }}>
            <Sidebar />
            <Box sx={{ flex: 1, overflowY: 'auto' }}>
                <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
                    <Box sx={{ flexShrink: 1 }}>
                        <Paper elevation={0} sx={{ backgroundImage: `url(${channelInfo?.channelImage})`, backgroundSize: 'contain' }}>
                            <Box sx={{ display: 'flex', flexDirection: 'row', alignItems: 'center', backdropFilter: 'blur(20px)', textShadow: '1px 1px #000' }}>
                                <Box sx={{ flexGrow: 1, paddingLeft: 1 }}>
                                    {channelInfo?.name}
                                </Box>
                                <Box sx={{ flexGrow: 0 }}>
                                    <Box sx={{ display: 'flex', flexDirection: 'row', alignItems: 'center' }}>
                                        <Tooltip title={showLog ? 'Hide Log' : 'Show Log'}>
                                            <IconButton size="small" onClick={() => setShowLog(!showLog)}>
                                                {eventLogIcon}
                                            </IconButton>
                                        </Tooltip>
                                    </Box>
                                </Box>
                            </Box>
                        </Paper>
                    </Box>
                    <ChatMessageContainer messages={messageLog}></ChatMessageContainer>
                    <ChatInput />
                </Box>
            </Box>
            {eventLogBox}
        </Box>
    )
}

export default Chat;