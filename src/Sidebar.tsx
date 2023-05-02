import { Box, ButtonGroup } from "@mui/material"
import LogoutIcon from '@mui/icons-material/Logout';
import InfoIcon from '@mui/icons-material/Info';
import { invoke } from "@tauri-apps/api";
import { useNavigate } from "react-router-dom";
import { useState } from "react";
import { LoadingButton } from "@mui/lab";
import './Sidebar.css'
import ChannelViewer from "./components/ChannelViewer";
import CurrentUserInfo from "./components/CurrentUserInfo";

interface SidebarProps {
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

    return (
        <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column', alignItems: 'center', alignContent: 'center' }} className="sidebar">
            <Box sx={{ flex: 1, overflowY: 'auto', width: '100%' }} >
                <CurrentUserInfo />
                <ChannelViewer />
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
