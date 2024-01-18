import { Backdrop, Box, Container, Link, Popover, } from "@mui/material";
import { UsersState } from "../store/features/users/userSlice";
import "./styles/UserInfo.css";
import "./styles/common.css"
import UserInfo from "./UserInfo";
import React, { useState } from "react";
import { openInBrowser } from "../helper/BrowserUtils";
import { useTranslation } from "react-i18next";

interface LightBoxImageProps {
    src: string;
}

function LightBoxImage(props: LightBoxImageProps) {
    const { t, i18n } = useTranslation();
    const [open, setOpen]: any = useState(false);

    const handleClose = () => {
        setOpen(false);
    };

    return (
        <Box>
            <img src={props.src} onClick={() => setOpen(true)} style={{ maxWidth: '100%', maxHeight: '600px', cursor: 'pointer' }} />
            <Backdrop
                sx={{ color: '#fff', zIndex: (theme) => theme.zIndex.drawer + 1, backdropFilter: 'blur(5px)', padding: '50px 10px 10px 10px' }}
                open={open}
                onClick={handleClose}
            >
                <Box sx={{ display: 'flex', flexDirection: 'column', height: '100%', width: '100%' }}>
                    <Box sx={{ flexShrink: 0, display: 'contents' }}>
                        <img src={props.src} style={{ height: 'auto', width: 'auto', maxWidth: '100%', maxHeight: 'calc(100% - 2em)', objectFit: 'contain' }} />
                    </Box>
                    <Box sx={{ flexShrink: 1, textAlign: 'center'}}>
                        <Link href="#" color="inherit" underline="hover" onClick={() => openInBrowser(props.src)}>{t('Open In Browser')}</Link>
                    </Box>
                </Box>
            </Backdrop>
        </Box>
    );
}

export default LightBoxImage;