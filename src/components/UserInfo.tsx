import { Typography, Popover, Card, Avatar, CardMedia, CardContent, Paper, IconButton, InputBase, Divider, Box } from "@mui/material";
import { UsersState } from "../store/features/users/userSlice";
import React, { useEffect, useState } from "react";
import { getBackgroundFromComment, getProfileImage, getTextFromcomment } from "../helper/UserInfoHelper";
import SendIcon from '@mui/icons-material/Send';
import "./styles/UserInfo.css";
import dayjs from "dayjs";
import MessageParser from "../helper/MessageParser";
import { ChatMessageHandler } from "../helper/ChatMessage";
import { useDispatch, useSelector } from "react-redux";
import { RootState } from "../store/store";
import "./styles/common.css"

interface UserInfoProps {
    userInfo: UsersState | undefined;
    style: React.CSSProperties;
}

function UserInfo(props: UserInfoProps) {
    const background = getBackgroundFromComment(props.userInfo, false);
    const profileText = getTextFromcomment(props.userInfo);
    const [chatMessage, setChatMessage] = useState("");

    const dispatch = useDispatch();
    const chatMessageHandler = new ChatMessageHandler(dispatch, setChatMessage);

    let mutedText = props.userInfo?.mutedSince ? dayjs(props.userInfo?.mutedSince).fromNow() : '';
    let deafenedText = props.userInfo?.deafenedSince ? dayjs(props.userInfo?.deafenedSince).fromNow() : '';

    function generateCardMedia() {
        if (background) {
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
                <Box sx={{ width: '100%', height: 100 }} className="animated-background" />
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

    function keyDownHandler(e: React.KeyboardEvent<HTMLInputElement | HTMLTextAreaElement>) {
        if (e && e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            chatMessageHandler.sendPrivateMessage(chatMessage, props.userInfo?.id || 0);
        }
    }

    function getUserColors(): React.CSSProperties | undefined {
        if (props.userInfo) {
            let settings = props.userInfo?.commentData?.settings;

            let colors: React.CSSProperties = {};
            if(settings?.primary_color) colors.backgroundColor = settings.primary_color;
            if(settings?.accent_color) colors.backgroundColor = settings.accent_color;

            return colors;
        }
    }

    return (
        <Card sx={{ maxWidth: 345 }} style={props.style}>
            {generateCardMedia()}
            <CardContent sx={{ paddingTop: 0, ...getUserColors()  }}>
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
                        {showStatusBox("Deafened", deafenedText)}
                    </Box>
                    <Divider sx={{ margin: '10px 0' }} />
                    <Box className="user-info-text">
                        <Box className="user-profile-content" dangerouslySetInnerHTML={{ __html: profileText }}></Box>
                    </Box>
                    <Paper
                        component="form"
                        sx={{ p: '2px 4px', display: 'flex', alignItems: 'center' }}
                        elevation={1}
                    >
                        <InputBase
                            sx={{ ml: 1, flex: 1 }}
                            placeholder={"write " + props.userInfo?.name + "..."}
                            inputProps={{ 'aria-label': 'search google maps' }}
                            onChange={e => setChatMessage(e.target.value)}
                            onKeyDown={keyDownHandler}
                            value={chatMessage}
                        />
                        <Divider sx={{ height: 28, m: 0.5 }} orientation="vertical" />
                        <IconButton color="primary" sx={{ p: '10px' }} aria-label="directions">
                            <SendIcon />
                        </IconButton>
                    </Paper>
                </Paper>
            </CardContent>
        </Card>
    );
}

export default UserInfo;