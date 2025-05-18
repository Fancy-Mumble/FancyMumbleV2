import { Box, IconButton, Link, Skeleton, Tooltip, Typography } from "@mui/material"
import Grid from '@mui/material/Grid';
import dayjs from "dayjs";
import 'dayjs/locale/en';
import 'dayjs/plugin/isToday';
import 'dayjs/plugin/isYesterday';
import MessageParser from "../helper/MessageParser";
import ThumbUpOffAltIcon from '@mui/icons-material/ThumbUpOffAlt';
import { invoke } from "@tauri-apps/api/core";
import { TextMessage, deleteChatMessage } from "../store/features/users/chatMessageSlice";
import ClearIcon from '@mui/icons-material/Clear';
import { useDispatch, useSelector } from "react-redux";
import React, { Suspense, use, useEffect } from "react";
import { RootState } from "../store/store";
import "./styles/ChatMessage.css";
import MessageUIHelper from "../helper/MessageUIHelper";
import { useTranslation } from "react-i18next";

const messageCache = new Map();

interface ChatMessageProps {
    message: TextMessage,
    messageId: number,
    onLoaded: () => void,
}

const parseMessage = async (message: string | undefined) => {
    if (message && message.includes('<')) {
        let messageParser = (await new MessageParser(message)
            .parseMarkdown())
            .parseDOM((dom) => dom
                .parseForImages()
                .parseForVideos()
                .parseForLinks()
            )
            .buildString();

        return messageParser;
    }

    return message;
}

const parseUI = (message: string | undefined, onLoaded: () => void) => {
    if (message && message.includes('<')) {
        let messageParser = new MessageUIHelper(message, () => onLoaded());
        let element = messageParser.build();

        return { standalone: messageParser.containsImages, element: element };
    }

    return { standalone: false, element: message };
}

const generateDate = (timestamp: number, locale = 'en') => {
    let day = dayjs(timestamp).locale(locale);
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

function getCachedMessage(message: string) {
    if (!messageCache.has(message)) {
        messageCache.set(message, parseMessage(message))
    }
    return messageCache.get(message)
}

const MessageRenderer: React.FC<{ messagePromise: Promise<string>, onLoaded: () => void }> = ({ messagePromise, onLoaded }) => {
    const parsedMessage = React.use(messagePromise);
    const parsedUI = parseUI(parsedMessage, onLoaded);

    if (parsedUI.standalone) {
        return (<Grid className="message-container-inner">{parsedUI.element}</Grid>);
    }

    return (
        <Grid className="message-container-inner">
            <Box className={`message ${false ? "sender" : "receiver"}`}>
                {parsedUI.element}
            </Box>
        </Grid>);
}


const MessageContent: React.FC<{ message: string, onLoaded: () => void }> = ({ message, onLoaded }) => {
    const messagePromise = getCachedMessage(message);
    console.log("Rendering message: ", message);

    return (
        <MessageRenderer messagePromise={messagePromise} onLoaded={onLoaded} />
    );
}

const ChatMessage: React.FC<ChatMessageProps> = React.memo(({ message, messageId, onLoaded }) => {
    const userList = useSelector((state: RootState) => state.reducer.userInfo);
    const locale = useSelector((state: RootState) => state.reducer.frontendSettings.language?.language);
    const dispatch = useDispatch();
    const { t } = useTranslation();

    useEffect(() => {
        console.log("ChatMessage: ", message);
        const videoRepeatLength = 10; // seconds
        console.log('Adding event listeners');
        // yes, I know this is a bad practice, but I'm not sure how to do it better
        document.querySelectorAll<HTMLVideoElement>('.user-video-element').forEach(e => {
            console.log(e);
            e.preload = "metadata";
            e.addEventListener('loadedmetadata', (e) => {
                e.preventDefault();
                e.stopPropagation();
                if (e.target) {
                    const videoElement = e.target as HTMLVideoElement;
                    console.log(videoElement.duration);
                    const videoLength = videoRepeatLength * Math.ceil(videoElement.duration / videoRepeatLength);

                    videoElement.loop = true;
                    videoElement.play();
                    setTimeout(() => {
                        videoElement.pause();
                        videoElement.loop = false;
                    }, videoLength * 1000);
                }
            });

            e.addEventListener('mouseenter', (e) => {
                e.preventDefault();
                e.stopPropagation();
                if (e.target) {
                    const videoElement = e.target as HTMLVideoElement;
                    videoElement.play();
                    videoElement.loop = true;
                }
            });

            e.addEventListener('mouseleave', (e) => {
                e.preventDefault();
                e.stopPropagation();
                if (e.target) {
                    const videoElement = e.target as HTMLVideoElement;
                    videoElement.pause();
                    videoElement.loop = false;
                }
            });
        });
    }, []);


    const user = React.useMemo(() =>
        userList.users.find(e => e.id === message.sender.user_id)
        , [userList, message.sender.user_id]);
    const date = React.useMemo(() => generateDate(message.timestamp, locale), [message.timestamp]);

    const deleteMessageEvent = React.useCallback(() => {
        dispatch(deleteChatMessage(messageId));
    }, [dispatch, messageId]);

    const likeMessage = React.useCallback((messageId: string) => {
        invoke('like_message', { messageId: messageId, reciever: userList.users.map(e => e.id) });
    }, []);

    const metadata = React.useMemo(() => (
        <Grid className="message-metadata">
            <Typography variant="subtitle2" className="metadata">
                <Link className="user-info" href="#">{message.sender.user_name}</Link> -
                {date}
            </Typography>
            <Tooltip title={t("Like")}>
                <IconButton aria-label="Like" size="small" onClick={() => likeMessage(message.id)}>
                    <ThumbUpOffAltIcon fontSize="small" color="disabled" />
                </IconButton>
            </Tooltip>
            <Tooltip title="Delete message locally">
                <IconButton aria-label="Delete" size="small" onClick={deleteMessageEvent}>
                    <ClearIcon fontSize="small" color="disabled" />
                </IconButton>
            </Tooltip>
        </Grid>
    ), [message, locale, t, messageId]);

    return (
        <Grid size={10} className="message-container">
            <Grid className="message-container-inner">
                <Suspense fallback={<Grid className="message-container-inner"><Skeleton animation="wave" width={40} /></Grid>}>
                    <MessageContent
                        message={message.message}
                        onLoaded={onLoaded}
                        key={messageId}
                    />
                </Suspense>
            </Grid>
            {metadata}
        </Grid>
    );
});

export const MemoChatMessage = ChatMessage;
