import { Avatar, Chip, Grid, IconButton, ImageList, ImageListItem, Link, ListItem, ListItemAvatar, ListItemText, Typography } from "@mui/material"
import { makeStyles } from '@mui/styles';
import ImageIcon from '@mui/icons-material/Image';
import React from "react"
import dayjs from "dayjs";
import 'dayjs/locale/de';
import 'dayjs/plugin/isToday';
import 'dayjs/plugin/isYesterday';
import { blueGrey, grey } from "@mui/material/colors";
import DOMPurify from 'dompurify';
import MessageParser from "../helper/MessageParser";
import { useSelector } from "react-redux";
import { RootState } from "../store/store";
import ThumbUpOffAltIcon from '@mui/icons-material/ThumbUpOffAlt';
import { invoke } from "@tauri-apps/api";

export interface SenderInfo {
    user_id: number,
    user_name: string,
}
export interface TextMessage {
    // The message sender, identified by its session.
    actor: number,
    sender: SenderInfo,
    // The channels to which the message is sent, identified by their
    // channel_ids.
    channel_id: number[]
    // The root channels when sending message recursively to several channels,
    // identified by their channel_ids.
    tree_id: number[]
    // The UTF-8 encoded message. May be HTML if the server allows.
    message: string,
    // custom property to keep track of time
    timestamp: number
}


interface ChatMessageProps {
    message: TextMessage
    prevCommentBy: number | undefined
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
    const userList = useSelector((state: RootState) => state.reducer.user);
    const classes = useStyles();
    const followUpMessage = props.prevCommentBy === props.message.sender.user_id;

    function parseMessage(message: string | undefined) {
        if (message && message.includes('<')) {
            let messageParser = new MessageParser(message).parseForImages().parseForLinks().parseForEmojis().build();

            return (
                <div>
                    {messageParser.map(e => e)}
                </div>
            )
        }

        return message;
    }

    function getProfileImage(user_id: number) {
        let userIndex = userList.findIndex(e => e.id === user_id);
        if (userIndex !== -1) {
            return userList[userIndex].profile_picture;
        }

        return "";
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

    return (
        <Grid container className={classes.root} style={{marginBottom: (followUpMessage ? 0 : '16px')}}>
            <Grid item>
                <Avatar src={getProfileImage(props.message.sender.user_id)} className={classes.avatar} style={{ visibility: (followUpMessage ? 'hidden' : 'visible') }} />
            </Grid>
            <Grid item xs={10} className={classes.messageContainer}>
                <Grid item className={classes.messageContainerInner}>
                    <Typography
                        variant="body1"
                        className={`${classes.message} ${false ? classes.sender : classes.receiver
                            }`}
                    >
                        {parseMessage(props.message.message)}
                    </Typography>
                </Grid>
                <Grid item className={classes.messageMetadata}>
                    <Typography variant="subtitle2" color="textSecondary" className={classes.metadata}>
                        <Link className={classes.userInfo} href="#">{props.message.sender.user_name}</Link> - {generateDate(props.message.timestamp)}
                    </Typography>
                    <IconButton aria-label="Example" size="small" onClick={e => likeMessage("abc")}>
                        <ThumbUpOffAltIcon fontSize="small" color="disabled" />
                    </IconButton>
                </Grid>
            </Grid>
        </Grid>
    );

}

export default ChatMessage