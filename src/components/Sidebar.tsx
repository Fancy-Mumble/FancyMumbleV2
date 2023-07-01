import { Box, Button, ButtonGroup } from "@mui/material"
import LogoutIcon from '@mui/icons-material/Logout';
import InfoIcon from '@mui/icons-material/Info';
import { invoke } from "@tauri-apps/api";
import { useNavigate } from "react-router-dom";
import { useCallback, useState } from "react";
import { LoadingButton } from "@mui/lab";
import './Sidebar.css'
import ChannelViewer from "./ChannelViewer";
import CurrentUserInfo from "./CurrentUserInfo";
import SettingsIcon from '@mui/icons-material/Settings';
import React from "react";

function Sidebar() {

    const navigate = useNavigate();
    const [logoutInProgress, setLogoutInProgress] = useState(false);

    const triggerLogout = useCallback((event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
        event.preventDefault();
        setLogoutInProgress(true);
        invoke('logout').then(e => {
            setLogoutInProgress(false);
            navigate("/");
        })
    }, [navigate]); // add dependencies here

    const openSettings = useCallback((event: React.MouseEvent<HTMLButtonElement, MouseEvent>): void => {
        event.preventDefault();
        navigate("/settings");
    }, [navigate]); // add dependencies here

    return (
        <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column', alignItems: 'center', alignContent: 'center', width: '250px' }} className="sidebar">
            <Box sx={{ flex: 1, overflowY: 'auto', width: '100%', display: 'flex', flexDirection: 'column' }} >
                <CurrentUserInfo />
                <ChannelViewer />
                <Box m={3} sx={{ display: 'flex', justifyContent: 'center' }}>
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
            </Box>
        </Box >
    )
}

export default React.memo(Sidebar);
