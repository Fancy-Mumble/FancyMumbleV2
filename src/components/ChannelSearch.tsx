import { Avatar, Backdrop, Box, CircularProgress, Container, IconButton, InputBase, List, ListItem, ListItemAvatar, ListItemButton, ListItemText, Paper } from "@mui/material";
import SearchIcon from '@mui/icons-material/Search';
import React from "react";
import { useSelector } from "react-redux";
import { RootState } from "../store/store";
import { getChannelImageFromDescription } from "../helper/ChannelInfoHelper";
import { invoke } from "@tauri-apps/api";
import './styles/ChannelSearch.css';
import Fuse from 'fuse.js';

const fuseOptions = {
    // isCaseSensitive: false,
    // includeScore: false,
    // shouldSort: true,
    // includeMatches: false,
    // findAllMatches: false,
    // minMatchCharLength: 1,
    // location: 0,
    // threshold: 0.6,
    // distance: 100,
    // useExtendedSearch: false,
    // ignoreLocation: false,
    // ignoreFieldNorm: false,
    // fieldNormWeight: 1,
    keys: [
        "name",
    ]
};

function ChannelSearch() {
    const channelList = useSelector((state: RootState) => state.reducer.channel);
    const fuse = new Fuse(channelList, fuseOptions);

    const [open, setOpen] = React.useState(false);
    const [channelSearchValue, setChannelSearchValue] = React.useState("");
    const handleClose = () => {
        setOpen(false);
        setChannelSearchValue("");
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
        <Box sx={{ background: 'transparent', padding: '4px 0', maxWidth: '150px', paddingLeft: '10px' }}>
            <Paper
                component="form"
                sx={{ p: '2px 4px', display: 'flex', alignItems: 'center', backdropFilter: 'blur(10px)', background: 'rgba(0, 0, 0, 0.5)' }}
                onClick={() => handleOpen()}
            >
                <InputBase
                    sx={{ ml: 1, flex: 1 }}
                    placeholder="Search"
                    inputProps={{ 'aria-label': 'search' }}
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
                <Box
                    className="search-channel"
                    onClick={stopHandleClose}>
                    <Box className="inner-search-channel">
                        <Paper
                            elevation={1}
                            component="form"
                            sx={{
                                p: '2px 4px', display: 'flex', alignItems: 'center', flexGrow: 1, margin: '10px'
                            }}
                        >
                            <InputBase
                                sx={{ ml: 1, flex: 1 }}
                                placeholder="Search Channel"
                                inputProps={{ 'aria-label': 'search channel' }}
                                onChange={e => setChannelSearchValue(e.target.value)}
                                value={channelSearchValue}
                            />
                            <IconButton type="button" sx={{ p: '10px' }} aria-label="search">
                                <SearchIcon />
                            </IconButton>
                        </Paper>
                    </Box>
                    <List dense={true} className="search-channel-list">
                        {(channelSearchValue === ''
                            ? channelList
                            : fuse.search(channelSearchValue).map(e => e.item))
                            .map((channel) => (
                                <ListItemButton key={channel.channel_id} onClick={joinChannel(channel.channel_id)}>
                                    <ListItemAvatar>
                                        <Avatar
                                            alt={channel.name}
                                            sx={{ width: 32, height: 32 }}
                                            src={channel.channelImage}
                                        />
                                    </ListItemAvatar>
                                    <ListItemText id={`channel-${channel.id}`} primary={channel.name} />
                                </ListItemButton>
                            ))}
                    </List>
                </Box>
            </Backdrop>
        </Box>
    )
}

export default ChannelSearch;