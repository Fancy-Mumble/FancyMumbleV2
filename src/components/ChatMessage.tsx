import { Box, Grid, IconButton, Link, Tooltip, Typography } from "@mui/material"
import dayjs from "dayjs";
import 'dayjs/locale/en';
import 'dayjs/plugin/isToday';
import 'dayjs/plugin/isYesterday';
import MessageParser from "../helper/MessageParser";
import ThumbUpOffAltIcon from '@mui/icons-material/ThumbUpOffAlt';
import { invoke } from "@tauri-apps/api";
import { TextMessage, deleteChatMessage } from "../store/features/users/chatMessageSlice";
import ClearIcon from '@mui/icons-material/Clear';
import { useDispatch, useSelector } from "react-redux";
import React, { } from "react";
import { RootState } from "../store/store";
import "./styles/ChatMessage.css";
import MessageUIHelper from "../helper/MessageUIHelper";


interface ChatMessageProps {
    message: TextMessage,
    messageId: number,
    onLoaded: () => void,
}

const parseMessage = (message: string | undefined) => {
    if (message && message.includes('<')) {
        let messageParser = new MessageParser(message)
            .parseMarkdown()
            .parseDOM((dom) => dom
                .parseForImages()
                .parseForLinks()
            )
            .buildString();

        return messageParser;
    }

    console.log(message);

    return message;
}
const parseUI = (message: string | undefined, onLoaded: () => void) => {
    if (message && message.includes('<')) {
        let messageParser = new MessageUIHelper(message, () => onLoaded());

        return messageParser.build();
    }

    return message;
}

const generateDate = (timestamp: number) => {
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

const ChatMessage: React.FC<ChatMessageProps> = React.memo(({ message, messageId, onLoaded }) => {
    const userList = useSelector((state: RootState) => state.reducer.userInfo);
    const dispatch = useDispatch();
    const [loaded, setLoaded] = React.useState(false);

    const user = React.useMemo(() =>
        userList.users.find(e => e.id === message.sender.user_id)
        , [userList, message.sender.user_id]);

    const parsedMessage = React.useMemo(() => parseUI(parseMessage(message.message), onLoaded), [message.message]);
    const date = React.useMemo(() => generateDate(message.timestamp), [message.timestamp]);

    const deleteMessageEvent = React.useCallback(() => {
        dispatch(deleteChatMessage(messageId));
    }, [dispatch, messageId]);

    const likeMessage = React.useCallback((messageId: string) => {
        invoke('like_message', { messageId: messageId });
    }, []);

    return (
        <Grid item xs={10} className="message-container">
            <Grid item className="message-container-inner">
                <Box className={`message ${false ? "sender" : "receiver"}`}>
                    {parsedMessage}
                </Box>
            </Grid>
            <Grid item className="message-metadata">
                <Typography variant="subtitle2" className="metadata">
                    <Link className="user-info" href="#">{message.sender.user_name}</Link> - {date}
                </Typography>
                <Tooltip title="Like">
                    <IconButton aria-label="Example" size="small" onClick={e => likeMessage("abc")}>
                        <ThumbUpOffAltIcon fontSize="small" color="disabled" />
                    </IconButton>
                </Tooltip>
                <Tooltip title="Delete message locally">
                    <IconButton aria-label="Example" size="small" onClick={deleteMessageEvent}>
                        <ClearIcon fontSize="small" color="disabled" />
                    </IconButton>
                </Tooltip>
            </Grid>
        </Grid>
    );
});

export const MemoChatMessage = ChatMessage;
