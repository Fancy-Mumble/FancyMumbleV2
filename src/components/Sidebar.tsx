import { Box, Button, ButtonGroup } from "@mui/material"
import LogoutIcon from '@mui/icons-material/Logout';
import InfoIcon from '@mui/icons-material/Info';
import { invoke } from "@tauri-apps/api";
import { useNavigate } from "react-router-dom";
import { useState } from "react";
import { LoadingButton } from "@mui/lab";
import './Sidebar.css'
import ChannelViewer from "./ChannelViewer";
import CurrentUserInfo from "./CurrentUserInfo";
import SettingsIcon from '@mui/icons-material/Settings';

interface SidebarProps {
}

function Sidebar(props: SidebarProps) {

    const navigate = useNavigate();
    const [logoutInProgress, setLogoutInProgress] = useState(false);

    function triggerLogout(event: React.MouseEvent<HTMLButtonElement, MouseEvent>) {
        event.preventDefault();
        setLogoutInProgress(true);
        invoke('logout').then(e => {
            setLogoutInProgress(false);
            navigate("/");
        })
    }

    function openSettings(event: React.MouseEvent<HTMLButtonElement, MouseEvent>): void {
        event.preventDefault();
        navigate("/settings");
    }

    return (
        <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column', alignItems: 'center', alignContent: 'center' }} className="sidebar">
            <Box sx={{ flex: 1, overflowY: 'auto', width: '100%' }} >
                <CurrentUserInfo />
                <ChannelViewer />
            </Box>
            <Box m={3}>
                <ButtonGroup variant="text">
                    <LoadingButton loading={logoutInProgress} onClick={triggerLogout} color="error"><LogoutIcon /></LoadingButton >
                    <Button color="inherit">
                        <InfoIcon />
                    </Button>
                    <Button onClick={openSettings} color="inherit">
                        <SettingsIcon />
                    </Button>
                </ButtonGroup>
            </Box>
        </Box >
    )
}

export default Sidebar
