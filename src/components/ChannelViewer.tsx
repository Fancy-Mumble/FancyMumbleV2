import { Avatar, Box, List, ListItem, ListItemAvatar, ListItemIcon, ListItemText, ListSubheader } from "@mui/material";
import { ChannelState } from "../store/features/users/channelSlice";
import { UsersState } from "../store/features/users/userSlice";
import { useSelector } from "react-redux";
import { RootState } from "../store/store";
import { invoke } from "@tauri-apps/api";
import ArrowForwardIosIcon from '@mui/icons-material/ArrowForwardIos';
import MicOffIcon from '@mui/icons-material/MicOff';
import VolumeOffIcon from '@mui/icons-material/VolumeOff';
import DOMPurify from "dompurify";
import { ReactNode } from "react";

function ChannelViewer() {
    const userList = useSelector((state: RootState) => state.reducer.user);
    const channelList = useSelector((state: RootState) => state.reducer.channel);

    function getChannelUserMapping() {
        let channelUserMapping: Map<ChannelState, UsersState[]> = new Map();
        userList.forEach(user => {
            let channel = channelList.find(channel => channel.channel_id === user.channel_id);
            if (channel !== undefined) {
                if (channelUserMapping.has(channel)) {
                    channelUserMapping.get(channel)?.push(user);
                } else {
                    channelUserMapping.set(channel, [user]);
                }
            }
        });
        return channelUserMapping;
    }

    function joinChannel(channelId: number) {
        invoke('join_channel', { channelId: channelId });
    }

    function getBackgroundFromComment(user: UsersState) {
        let cleanMessage = DOMPurify.sanitize(user.comment);
        const parser = new DOMParser();
        const document = parser.parseFromString(cleanMessage, "text/html");
        const images = Array.from(document.querySelectorAll('img')).map(img => img.src);

        if (user.comment) {
            return "url(" + images[images.length - 1] + ")";
        }
    }

    function displayUserInfo(user: UsersState): ReactNode {
        return (
            <span>
                {user.name}
                {user.self_mute ? (<MicOffIcon fontSize="small" />) : (<span />)}
                {user.self_deaf ? (<VolumeOffIcon fontSize="small" />) : (<span />)}
                {user.mute ? (<MicOffIcon color="error" fontSize="small" />) : (<span />)}
                {user.deaf ? (<VolumeOffIcon color="error" fontSize="small" />) : (<span />)}
            </span>
        )
    }

    return (
        <List subheader={<li />}>
            {
                Array.from(getChannelUserMapping()).map(([channel, users]) => (
                    <li key={`channel-${channel.channel_id}`}>
                        <ul style={{ padding: 0 }}>
                            <ListSubheader className="subheader-flex" onClick={e => joinChannel(channel.channel_id)}>
                                {channel.name}
                                <ListItemIcon className="join-button" style={{ cursor: 'pointer' }}>
                                    <ArrowForwardIosIcon />
                                </ListItemIcon>
                            </ListSubheader>
                            {users.map((user) => (
                                <Box key={`user-${user.id}`} sx={{ background: getBackgroundFromComment(user), backgroundSize: 'cover' }}>
                                    <ListItem key={user.id} sx={{ py: 0, minHeight: 32, backdropFilter: "blur(10px)", textShadow: "1px 1px #000000 " }}>
                                        <ListItemAvatar sx={{ width: 24, height: 24, minWidth: 0, marginRight: 1 }}>
                                            <Avatar sx={{ width: 24, height: 24 }} src={user.profile_picture} />
                                        </ListItemAvatar>
                                        <ListItemText primary={displayUserInfo(user)} primaryTypographyProps={{ fontSize: 14, fontWeight: 'medium' }} />
                                    </ListItem>
                                </Box>
                            ))
                            }
                        </ul>
                    </li>
                ))
            }
        </List>
    )
}

export default ChannelViewer;