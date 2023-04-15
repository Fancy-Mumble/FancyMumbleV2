import { Avatar, ImageList, ImageListItem, Link, ListItem, ListItemAvatar, ListItemText, Typography } from "@mui/material"
import ImageIcon from '@mui/icons-material/Image';
import React from "react"
import dayjs from "dayjs";
import 'dayjs/locale/de';
import { blueGrey } from "@mui/material/colors";
import DOMPurify from 'dompurify';

export interface TextMessage {
    // The message sender, identified by its session.
    actor: number,
    // Target users for the message, identified by their session.
    session: number[]
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
    parseMessage(message: string) {
        if (message.includes('<')) {
            let cleanMessage = DOMPurify.sanitize(message);
            const parser = new DOMParser();
            const doc = parser.parseFromString(cleanMessage, "text/html");

            const images = Array.from(doc.querySelectorAll('img')).map(img => img.src);

            return (<ImageList cols={Math.min(images.length, 3)} rowHeight={164}>
                {images.map(e =>
                    <ImageListItem key={e}>
                        <img src={e} />
                    </ImageListItem>
                )}
            </ImageList>);
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
                                <Link underline="hover" href="#">{this.props.message.actor}</Link>
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