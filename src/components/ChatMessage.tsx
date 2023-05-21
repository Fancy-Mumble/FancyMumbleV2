import { Avatar, Box, Grid, IconButton, Link, Tooltip, Typography } from "@mui/material"
import { makeStyles } from '@mui/styles';
import dayjs from "dayjs";
import 'dayjs/locale/en';
import 'dayjs/plugin/isToday';
import 'dayjs/plugin/isYesterday';
import { grey } from "@mui/material/colors";
import IncomingMessageParser from "../helper/IncomingMessageParser";
import ThumbUpOffAltIcon from '@mui/icons-material/ThumbUpOffAlt';
import { invoke } from "@tauri-apps/api";
import { getProfileImage } from "../helper/UserInfoHelper";
import { TextMessage, deleteChatMessage } from "../store/features/users/chatMessageSlice";
import ClearIcon from '@mui/icons-material/Clear';
import { useDispatch } from "react-redux";
import UserInfo from "./UserInfo";
import React from "react";
import { RootState } from "../store/store";
import { useSelector } from "react-redux";


interface ChatMessageProps {
    message: TextMessage,
    prevCommentBy: number | undefined,
    messageId: number,
}

const useStyles = makeStyles((theme: any) => ({
    root: {
        display: 'flex',
        marginBottom: theme.spacing(2),
    },
    avatar: {
        marginRight: theme.spacing(2),
        margin: theme.spacing(1),
    },
    messageContainer: {
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'space-between',
    },
    messageContainerInner: {
        flexDirection: 'column',
        display: 'flex',
        alignItems: 'flex-start'
    },
    message: {
        padding: theme.spacing(1),
        borderRadius: theme.shape.borderRadius,
        backgroundColor: theme.palette.background.paper,
        wordBreak: 'break-word',
    },
    messageMetadata: {
        display: 'flex',
        justifyContent: 'flex-start',
        alignItems: 'center',
        width: '100%'

    },
    sender: {
        alignSelf: 'flex-end',
    },
    receiver: {
        alignSelf: 'flex-start',
    },
    metadata: {
        fontSize: '0.8rem',
        color: grey[700],
    },
    userInfo: {
        color: grey[700],
        textDecoration: 'none',
    },
}));

function ChatMessage(props: ChatMessageProps) {
    const userList = useSelector((state: RootState) => state.reducer.userInfo);
    const dispatch = useDispatch();
    const classes = useStyles();
    const followUpMessage = props.prevCommentBy === props.message.sender.user_id;
    const [userInfoAnchor, setUserInfoAnchor]: any = React.useState(null);
    const user = userList.users.find(e => e.id === props.message.sender.user_id);

    function parseMessage(message: string | undefined) {
        if (message && message.includes('<')) {
            let messageParser = new IncomingMessageParser(message)
                .parseForMarkdown()
                .parseDOM((dom) => dom
                    .parseForImages()
                    .parseForLinks()
                )
                .build();

            return (
                <div>
                    {messageParser}
                </div>
            )
        }

        return message;
    }

    function generateDate(timestamp: number) {
        let day = dayjs(timestamp).locale('de-de');
        if (day.isToday()) {
            return day.format('HH:mm');
        } else if (day.isYesterday()) {
            return 'Yesterday ' + day.format('HH:mm');
        } else if (day.isBefore(dayjs().subtract(7, 'day'))) {
            return day.format('DD.MM.YYYY HH:mm');
        } else {
            return day.format('dddd HH:mm');
        }
    }

    function likeMessage(messageId: string) {
        invoke('like_message', { messageId: messageId });
    }

    function deleteMessageEvent(messageId: number) {
        dispatch(deleteChatMessage(messageId));
    }

    function showUserInfo() {
        if (userInfoAnchor) {
            return (
                <UserInfo
                    anchorEl={userInfoAnchor}
                    onClose={() => setUserInfoAnchor(null)}
                    userInfo={user}
                />
            )
        }
    }

    return (
        <Grid container className={classes.root} style={{ marginBottom: (followUpMessage ? 0 : '16px') }}>
            <Grid item>
                <Avatar
                    src={getProfileImage(props.message.sender.user_id)}
                    className={classes.avatar} style={{ visibility: (followUpMessage ? 'hidden' : 'visible') }}
                    onClick={e => { setUserInfoAnchor(e.currentTarget); }}
                />
                {/*This is wrong, we create this for every chat message :O*/}
                {/*showUserInfo()*/}
            </Grid>
            <Grid item xs={10} className={classes.messageContainer}>
                <Grid item className={classes.messageContainerInner}>
                    <Box className={`${classes.message} ${false ? classes.sender : classes.receiver}`}>
                        {parseMessage(props.message.message)}
                    </Box>
                </Grid>
                <Grid item className={classes.messageMetadata}>
                    <Typography variant="subtitle2" color="textSecondary" className={classes.metadata}>
                        <Link className={classes.userInfo} href="#">{props.message.sender.user_name}</Link> - {generateDate(props.message.timestamp)}
                    </Typography>
                    <Tooltip title="Like">
                        <IconButton aria-label="Example" size="small" onClick={e => likeMessage("abc")}>
                            <ThumbUpOffAltIcon fontSize="small" color="disabled" />
                        </IconButton>
                    </Tooltip>
                    <Tooltip title="Delete message locally">
                        <IconButton aria-label="Example" size="small" onClick={e => deleteMessageEvent(props.messageId)}>
                            <ClearIcon fontSize="small" color="disabled" />
                        </IconButton>
                    </Tooltip>
                </Grid>
            </Grid>
        </Grid>
    );

}

export const MemoChatMessage = React.memo(ChatMessage);
