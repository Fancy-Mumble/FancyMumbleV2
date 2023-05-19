import { Typography, Popover, Card, Avatar, CardMedia, CardContent, Paper, IconButton, InputBase, Divider, Box } from "@mui/material";
import { UsersState } from "../store/features/users/userSlice";
import React, { useEffect } from "react";
import { getBackgroundFromComment, getProfileImage } from "../helper/UserInfoHelper";
import SendIcon from '@mui/icons-material/Send';
import "./styles/UserInfo.css";
import dayjs from "dayjs";

interface UserInfoProps {
    anchorEl: HTMLElement | null;
    userInfo: UsersState | undefined;
    onClose: () => void;
}

function UserInfo(props: UserInfoProps) {
    const background = getBackgroundFromComment(props.userInfo, false);

    let mutedText = props.userInfo?.mutedSince ? dayjs(props.userInfo?.mutedSince).fromNow() : '';
    let deafenedText = props.userInfo?.deafenedSince ? dayjs(props.userInfo?.deafenedSince).fromNow() : '';

    function generateCardMedia() {
        if (!background.startsWith("#")) {
            return (
                <CardMedia
                    component="img"
                    height="100"
                    image={background}
                    alt="green iguana"
                />
            );
        } else {
            return (
                <Box sx={{ bgcolor: 'primary.main', width: '100%', height: 100 }} />
            );
        }
    }

    function showStatusBox(statusText: string, status: string) {
        if (status) {
            return (
                <Box className="user-info-item">
                    <span className="user-text-title">{statusText}</span><span>{status}</span>
                </Box>
            )
        }
    }

    return (
        <Popover
            open={Boolean(props.anchorEl)}
            anchorEl={props.anchorEl}
            onClose={props.onClose}
            anchorOrigin={{
                vertical: 'center',
                horizontal: 'right',
            }}
            transformOrigin={{
                vertical: 'center',
                horizontal: 'left',
            }}
        >
            <Card sx={{ maxWidth: 345 }}>
                {generateCardMedia()}
                <CardContent sx={{ paddingTop: 0 }}>
                    <Box className="user-info-avatar">
                        <Avatar
                            alt={props.userInfo?.name}
                            src={getProfileImage(props.userInfo?.id || 0)}
                            sx={{ width: 80, height: 80, marginTop: '-40px', border: '2px solid #000' }}
                        />
                    </Box>
                    <Paper elevation={0} sx={{ padding: '10px', margin: '-30px 0 0 0' }}>
                        <Typography gutterBottom variant="h5" component="div" sx={{ textAlign: 'end' }}>
                            {props.userInfo?.name}
                        </Typography>
                        <Divider sx={{ margin: '10px 0' }} />
                        <Box className="user-info-list">
                            <Box className="user-info-item">
                                <span className="user-text-title">User ID</span><span>#{props.userInfo?.id}</span>
                            </Box>
                            {showStatusBox("Muted", mutedText)}
                            {showStatusBox("Muted", deafenedText)}
                        </Box>
                        <Paper
                            component="form"
                            sx={{ p: '2px 4px', display: 'flex', alignItems: 'center' }}
                            elevation={1}
                        >
                            <InputBase
                                sx={{ ml: 1, flex: 1 }}
                                placeholder={"Send a message to " + props.userInfo?.name}
                                inputProps={{ 'aria-label': 'search google maps' }}
                            />
                            <Divider sx={{ height: 28, m: 0.5 }} orientation="vertical" />
                            <IconButton color="primary" sx={{ p: '10px' }} aria-label="directions">
                                <SendIcon />
                            </IconButton>
                        </Paper>
                    </Paper>
                </CardContent>
            </Card>
        </Popover>
    );
}

export default UserInfo;