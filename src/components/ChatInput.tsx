import { Box, Button, Divider, Fade, IconButton, InputBase, Paper, Popper, Tooltip } from "@mui/material";
import SendIcon from '@mui/icons-material/Send';
import DeleteIcon from '@mui/icons-material/Delete';
import GifIcon from '@mui/icons-material/Gif';
import { useCallback, useMemo, useState } from "react";
import { ChatMessageHandler } from "../helper/ChatMessage";
import { useDispatch, useSelector } from "react-redux";
import { RootState } from "../store/store";
import { formatBytes } from "../helper/Fomat";
import GifSearch from "./GifSearch";

function ChatInput() {
    const dispatch = useDispatch();

    const [showGifSearch, setShowGifSearch] = useState(false);
    const [showDeleteMessageConfirmation, setShowDeleteMessageConfirmation] = useState(false);

    const [chatMessage, setChatMessage] = useState("");
    const [gifSearchAnchor, setGifSearchAnchor] = useState<HTMLElement>();
    const [messageDeleteAnchor, setMessageDeleteAnchor] = useState<HTMLElement>();
    const currentUser = useSelector((state: RootState) => state.reducer.userInfo?.currentUser);
    const channelInfo = useSelector((state: RootState) => state.reducer.channel);

    const currentChannel = useMemo(() => channelInfo.find(e => e.channel_id === currentUser?.channel_id)?.name, [channelInfo, currentUser]);
    const chatMessageHandler = useMemo(() => new ChatMessageHandler(dispatch, setChatMessage), [dispatch]);

    const deleteMessages = useCallback(() => {
        chatMessageHandler.deleteMessages();
    }, [chatMessageHandler]);

    const keyDownHandler = useCallback((e: React.KeyboardEvent<HTMLInputElement | HTMLTextAreaElement>) => {
        if (e && e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            chatMessageHandler.sendChatMessage(chatMessage, currentUser);
        }
    }, [chatMessage, chatMessageHandler, currentUser]);

    const showGifPreview = useCallback((e: any) => {
        setShowGifSearch(prev => !prev);
        setGifSearchAnchor(e.currentTarget)
    }, []);

    const showDeleteMessageConfirmationDialog = useCallback((e: any) => {
        setShowDeleteMessageConfirmation(prev => !prev);
        setMessageDeleteAnchor(e.currentTarget)
    }, []);

    const pasteEvent = useCallback((event: any) => {
        let items = event.clipboardData.items;
        for (const item of items) {
            if (item.type.indexOf('image') !== -1) {
                const file = item.getAsFile();
                const reader = new FileReader();
                reader.readAsDataURL(file);
                reader.onload = function () {
                    if (reader.result && (reader.result as string).length > 0x7fffff) {
                        chatMessageHandler.sendCustomChatMessage("[[ Image too large ( " + formatBytes((reader.result as string).length) + " out of " + formatBytes(0x7fffff) + ") ]]", currentUser);
                        return;
                    }
                    let img = '<img src="' + reader.result + '" />';
                    chatMessageHandler.sendCustomChatMessage(img, currentUser);
                };
            }
        }
    }, [chatMessageHandler, currentUser]);

    return (
        <Box m={2} sx={{ display: 'flex' }}>
            <Paper
                component="form"
                sx={{ p: '2px 4px', display: 'flex', alignItems: 'center', width: 400, flexGrow: 1 }}
            >
                <Tooltip title="Delete all messages">
                    <IconButton sx={{ p: '10px' }} aria-label="menu" onClick={showDeleteMessageConfirmationDialog}>
                        <DeleteIcon />
                    </IconButton>
                </Tooltip>
                <InputBase
                    sx={{ ml: 1, flex: 1 }}
                    placeholder={"Send Message to " + currentChannel}
                    inputProps={{ 'aria-label': 'Send Message to ' + currentChannel }}
                    onChange={e => setChatMessage(e.target.value)}
                    onKeyDown={keyDownHandler}
                    value={chatMessage}
                    onPaste={pasteEvent}
                    multiline
                />
                <IconButton onClick={showGifPreview}>
                    <GifIcon />
                </IconButton>
                <Divider sx={{ height: 28, m: 0.5 }} orientation="vertical" />
                <IconButton sx={{ p: '10px' }} aria-label="Send Message" onClick={() => chatMessageHandler.sendChatMessage(chatMessage, currentUser)}>
                    <SendIcon />
                </IconButton>
            </Paper>
            <GifSearch open={showGifSearch} anchor={gifSearchAnchor} />
            <Popper open={showDeleteMessageConfirmation} anchorEl={messageDeleteAnchor} transition>
                {({ TransitionProps }) => (
                    <Fade {...TransitionProps}>
                        <Paper sx={{ p: 1 }}>
                            <Box sx={{ p: 1 }}>
                                Do you really want to delete all messages?
                                <Box sx={{ display: 'flex', justifyContent: 'flex-end' }}>
                                    <Button onClick={(data) => {
                                        deleteMessages();
                                        showDeleteMessageConfirmationDialog(data);
                                    }}
                                        color="error">Yes</Button>
                                    <Button onClick={showDeleteMessageConfirmationDialog} color="primary">No</Button>
                                </Box>
                            </Box>
                        </Paper>
                    </Fade>
                )}
            </Popper>
        </Box>
    )
}

export default ChatInput;