import { Avatar, Box, Button, ButtonGroup, Container, Icon, IconButton, List, ListItem, ListItemAvatar, ListItemIcon, ListItemText, Skeleton } from "@mui/material"
import MicOffIcon from '@mui/icons-material/MicOff';
import MicIcon from '@mui/icons-material/Mic';
import VolumeOffIcon from '@mui/icons-material/VolumeOff';
import VolumeUpIcon from '@mui/icons-material/VolumeUp';
import LogoutIcon from '@mui/icons-material/Logout';
import InfoIcon from '@mui/icons-material/Info';
import { invoke } from "@tauri-apps/api";
import { useNavigate } from "react-router-dom";
import { ReactNode, useState } from "react";
import { LoadingButton } from "@mui/lab";
import './Sidebar.css'
import DOMPurify from "dompurify";

export interface Users {
    channel_id: number,
    comment: string,
    deaf: boolean,
    id: number
    mute: boolean,
    name: string,
    priority_speaker: boolean,
    profile_picture: string,
    recording: boolean,
    self_deaf: boolean,
    self_mute: boolean,
    suppress: boolean,
}

interface SidebarProps {
    users: Users[]
}

function Sidebar(props: SidebarProps) {
    const navigate = useNavigate();
    const [logoutInProgress, setLogoutInProgress] = useState(false);

    function triggerLogout() {
        setLogoutInProgress(true);
        invoke('logout').then(e => {
            setLogoutInProgress(false);
            navigate("/");
        })
    }

    function displayUserInfo(user: Users): ReactNode {
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

    function getBackgroundFromComment(user: Users) {
        let cleanMessage = DOMPurify.sanitize(user.comment);
        const parser = new DOMParser();
        const document = parser.parseFromString(cleanMessage, "text/html");
        const images = Array.from(document.querySelectorAll('img')).map(img => img.src);

        if (user.comment) {
            return "url(" + images[images.length - 1] + ")";
        }
    }

    return (
        <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column', alignItems: 'center', alignContent: 'center' }} className="sidebar">
            <Box sx={{ flex: 1, overflowY: 'auto', width: '100%' }} >
                <Skeleton animation={false} variant="rectangular" width={100} height={60} />
                <Skeleton animation={false} width={100} />

                <List subheader={<li />}>
                    {props.users.map((user) => (
                        <Box sx={{background: getBackgroundFromComment(user), backgroundSize: 'cover' }}>
                            <ListItem key={user.id} sx={{ py: 0, minHeight: 32, backdropFilter: "blur(10px)", textShadow: "1px 1px #000000 "}}>
                                <ListItemAvatar sx={{ width: 24, height: 24, minWidth: 0, marginRight: 1 }}>
                                    <Avatar sx={{ width: 24, height: 24 }} src={user.profile_picture} />
                                </ListItemAvatar>
                                <ListItemText primary={displayUserInfo(user)} primaryTypographyProps={{ fontSize: 14, fontWeight: 'medium' }} />
                            </ListItem>
                        </Box>
                    ))}
                </List>

            </Box>
            <Box m={3}>
                <ButtonGroup variant="contained">
                    <LoadingButton loading={logoutInProgress} onClick={e => triggerLogout()} color="error"><LogoutIcon /></LoadingButton >
                    <LoadingButton color="info">
                        <InfoIcon />
                    </LoadingButton>
                </ButtonGroup>
            </Box>
        </Box >
    )
}

export default Sidebar
