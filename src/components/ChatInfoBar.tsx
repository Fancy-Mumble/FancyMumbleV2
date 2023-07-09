import { Box, IconButton, Paper, Tooltip } from '@mui/material';


import { RootState } from '../store/store';
import { useSelector } from 'react-redux';
import { useEffect, useMemo, useState } from 'react';
import AutoStoriesIcon from '@mui/icons-material/AutoStories';
import KeyboardDoubleArrowRightIcon from '@mui/icons-material/KeyboardDoubleArrowRight';
import React from 'react';

interface ChatInfoBarProps {
    onShowLog: (showLog: boolean) => void;
}

const ChatInfoBar: React.FC<ChatInfoBarProps> = React.memo(({ onShowLog }) => {
    const [showLog, setShowLog] = useState(false);

    const currentChannelId = useSelector((state: RootState) => state.reducer.userInfo.currentUser?.channel_id);
    const channelInfo = useSelector((state: RootState) => state.reducer.channel.find(e => e.channel_id === currentChannelId));

    const eventLogIcon = useMemo(() => {
        if (!showLog) return (<AutoStoriesIcon sx={{ fontSize: 20 }} />);
        else return (<KeyboardDoubleArrowRightIcon sx={{ fontSize: 20 }} />);
    }, [showLog]);

    useEffect(() => {
        onShowLog(showLog);
    }, [showLog, onShowLog]);

    return (
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

    )
});

export default ChatInfoBar;