import { Box, Button, ButtonGroup } from "@mui/material"
import LogoutIcon from '@mui/icons-material/Logout';
import InfoIcon from '@mui/icons-material/Info';
import { invoke } from "@tauri-apps/api";
import { useNavigate } from "react-router-dom";
import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { LoadingButton } from "@mui/lab";
import './Sidebar.css'
import ChannelViewer from "./ChannelViewer";
import CurrentUserInfo from "./CurrentUserInfo";
import SettingsIcon from '@mui/icons-material/Settings';
import CastIcon from '@mui/icons-material/Cast';
import { useSelector } from 'react-redux';
import { RootState } from "../store/store";
import { WebRTCStreamer, WebRTCViewer } from "../helper/webrtc/WebRTC";

function Sidebar() {
    const signalingServerUrl = "http://127.0.0.1:4000";
    const navigate = useNavigate();
    const [logoutInProgress, setLogoutInProgress] = useState(false);
    const [showWebRtcWindow, setShowWebRtcWindow] = useState(false);
    const [webRtcStreamer, setWebRtcStreamer] = useState<WebRTCStreamer | undefined>(undefined);
    const [webRtcViewer, setWebRtcViewer] = useState<WebRTCViewer | undefined>(undefined);

    const userList = useSelector((state: RootState) => state.reducer.userInfo);
    let currentUserId = userList.currentUser?.id;
    let currentChannelId = userList.currentUser?.channel_id;

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

    useEffect(() => {
        const viewer = new WebRTCViewer(signalingServerUrl, currentUserId ?? 0, currentChannelId ?? 0);
        viewer.listen();
        viewer.onStream((stream) => {
            setShowWebRtcWindow(true);
        });
        setWebRtcViewer(viewer);
        return () => {
            if (webRtcStreamer) {
                webRtcStreamer.stop();
            }
            viewer.stop();
            setShowWebRtcWindow(false);
        }
    }, []);

    const streamPreview = useMemo(() => {
        if (showWebRtcWindow) {
            return (
                <Box sx={{ overflowY: 'auto', width: '100%', display: 'flex', flexDirection: 'column' }} >
                    <video
                        ref={streamElement => {
                            if (streamElement) {
                                webRtcViewer?.onStream((stream) => {
                                    streamElement.srcObject = stream;
                                });
                            }
                        }}
                        autoPlay
                        playsInline
                        controls>
                    </video>
                </Box>)
        }
        return null;
    }, [webRtcStreamer]);


    const castScreen = async (event: React.MouseEvent<HTMLButtonElement, MouseEvent>): Promise<void> => {

        if (currentUserId === undefined || currentChannelId === undefined) {
            return;
        }
        const rtc = new WebRTCStreamer(signalingServerUrl, currentUserId ?? 0, currentChannelId ?? 0);
        setWebRtcStreamer(rtc);
        setShowWebRtcWindow(true);
        await rtc?.start();
    }

    return (
        <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column', alignItems: 'center', alignContent: 'center', width: '250px' }} className="sidebar">
            <Box sx={{ flex: 1, overflowY: 'auto', width: '100%', display: 'flex', flexDirection: 'column' }} >
                <CurrentUserInfo />
                {streamPreview}
                <ChannelViewer />
                <Box m={3} sx={{ display: 'flex', justifyContent: 'center' }}>
                    <ButtonGroup variant="text">
                        <LoadingButton loading={logoutInProgress} onClick={triggerLogout} color="error"><LogoutIcon /></LoadingButton >
                        <Button color="inherit">
                            <InfoIcon />
                        </Button>
                        <Button onClick={castScreen} color="inherit">
                            <CastIcon />
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
