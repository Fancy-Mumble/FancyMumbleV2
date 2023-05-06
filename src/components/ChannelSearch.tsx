import { Avatar, Backdrop, Box, CircularProgress, Container, IconButton, InputBase, List, ListItem, ListItemAvatar, ListItemButton, ListItemText, Paper } from "@mui/material";
import SearchIcon from '@mui/icons-material/Search';
import React from "react";
import { useSelector } from "react-redux";
import { RootState } from "../store/store";
import { getChannelImageFromDescription } from "../helper/ChannelInfoHelper";
import { invoke } from "@tauri-apps/api";

function ChannelSearch() {
    const channelList = useSelector((state: RootState) => state.reducer.channel);

    const [open, setOpen] = React.useState(false);
    const handleClose = () => {
        setOpen(false);
    };
    const handleOpen = () => {
        setOpen(true);
    };
    const stopHandleClose = (event: any) => {
        event?.stopPropagation();
    };

    const joinChannel = (channelId: number) => (event: any) => {
        event?.stopPropagation();
        invoke('join_channel', { channelId: channelId });
    }


    return (
        <Container sx={{ background: 'transparent', padding: '4px 0' }}>
            <Paper
                component="form"
                sx={{ p: '2px 4px', display: 'flex', alignItems: 'center', backdropFilter: 'blur(10px)', background: 'rgba(0, 0, 0, 0.5)' }}
                onClick={() => handleOpen()}
            >
                <InputBase
                    sx={{ ml: 1, flex: 1 }}
                    placeholder="Search Channel"
                    inputProps={{ 'aria-label': 'search channel' }}
                />
                <IconButton type="button" sx={{ p: '5px' }} aria-label="search">
                    <SearchIcon />
                </IconButton>
            </Paper>
            <Backdrop
                sx={{ color: '#fff', zIndex: (theme) => theme.zIndex.drawer + 1 }}
                open={open}
                onClick={handleClose}
            >
                <Box sx={{
                    display: 'flex',
                    flexDirection: 'column',
                    overflow: 'auto',
                    maxHeight: '100%'
                }}
                    onClick={stopHandleClose}>
                    <Box sx={{
                        position: 'sticky',
                        top: '0',
                        zIndex: 10,
                        backdropFilter: 'blur(10px)',
                        background: 'rgb(10 10 10 / 50%)'
                    }}>
                        <Paper
                            elevation={1}
                            component="form"
                            sx={{
                                p: '2px 4px', display: 'flex', alignItems: 'center', flexGrow: 1, margin: '10px', maxWidth: '600px'
                            }}
                        >
                            <InputBase
                                sx={{ ml: 1, flex: 1 }}
                                placeholder="Search Channel"
                                inputProps={{ 'aria-label': 'search channel' }}
                            />
                            <IconButton type="button" sx={{ p: '10px' }} aria-label="search">
                                <SearchIcon />
                            </IconButton>
                        </Paper>
                    </Box>
                    <List dense={true} sx={{
                        backdropFilter: 'blur(10px)',
                    }}>
                        {channelList.map((channel) => (
                            <ListItemButton key={channel.id} onClick={joinChannel(channel.channel_id)}>
                                <ListItemAvatar>
                                    <Avatar
                                        alt={channel.name}
                                        sx={{ width: 32, height: 32 }}
                                    />
                                </ListItemAvatar>
                                <ListItemText id={`channel-${channel.id}`} primary={channel.name} />
                            </ListItemButton>
                        ))}
                    </List>
                </Box>
            </Backdrop>
        </Container>
    )
}

export default ChannelSearch;