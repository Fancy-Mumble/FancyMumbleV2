import { Typography, Popover, Card, Avatar, CardMedia, CardContent, Paper, IconButton, InputBase, Divider, Box } from "@mui/material";
import { UsersState } from "../store/features/users/userSlice";
import React, { useEffect } from "react";
import { getBackgroundFromComment, getProfileImage } from "../helper/UserInfoHelper";
import SendIcon from '@mui/icons-material/Send';
import "./styles/UserInfo.css";

interface UserInfoProps {
    anchorEl: HTMLElement | null;
    userInfo: UsersState | undefined;
    onClose: () => void;
}

function UserInfo(props: UserInfoProps) {
    const background = getBackgroundFromComment(props.userInfo, false);

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

    return (
        <Popover
            open={Boolean(props.anchorEl)}
            anchorEl={props.anchorEl}
            onClose={props.onClose}
            anchorOrigin={{
                vertical: 'bottom',
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
                            sx={{ width: 50, height: 50, marginTop: '-25px', border: '2px solid #000' }}
                        />
                    </Box>
                    <Paper elevation={0} sx={{ padding: '10px', margin: '2px' }}>
                        <Typography gutterBottom variant="h5" component="div">
                            {props.userInfo?.name}
                            <Typography variant="body2" component="span">
                                {props.userInfo?.id}
                            </Typography>
                        </Typography>
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