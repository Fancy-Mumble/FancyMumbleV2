import { Avatar, ImageList, ImageListItem, Link, ListItem, ListItemAvatar, ListItemText, Typography } from "@mui/material"
import ImageIcon from '@mui/icons-material/Image';
import React from "react"
import dayjs from "dayjs";
import 'dayjs/locale/de';
import { blueGrey } from "@mui/material/colors";
import DOMPurify from 'dompurify';
import MessageParser from "../helper/MessageParser";

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
}

interface ChatMessageState {

}

class ChatMessage extends React.Component<ChatMessageProps, ChatMessageState> {

    parseMessage(message: string | undefined) {
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

    render() {
        return (
            <ListItem>
                <ListItemAvatar>
                    <Avatar>
                        <ImageIcon />
                    </Avatar>
                </ListItemAvatar>
                <ListItemText
                    primary={
                        <div>
                            <span style={{ marginRight: 15 }}>
                                <Link underline="hover" href="#">{this.props.message.sender.user_name}</Link>
                            </span>
                            <Typography variant="caption" color={blueGrey['300']}>
                                {dayjs(this.props.message.timestamp).locale('de-de').format('DD.MM.YYYY HH:mm')}
                            </Typography>
                        </div>}
                    secondary={this.parseMessage(this.props.message.message)} />
            </ListItem>);
    }
}

export default ChatMessage