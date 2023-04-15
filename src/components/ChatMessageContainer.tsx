import { Box, Card, CardContent, List } from "@mui/material";
import React from "react";
import ChatMessage, { TextMessage } from "./ChatMessage";


interface ChatMessageContainerProps {
	messages: TextMessage[]
}

interface ChatMessageContainerState {

}

class ChatMessageContainer extends React.Component<ChatMessageContainerProps, ChatMessageContainerState> {
	render() {
		return (
			<List sx={{ width: '100%', maxWidth: 360}}>
				{this.props.messages.map((el, index) => (<ChatMessage key={el.timestamp} message={el} />))}
			</List>
		);
	}
}
export default ChatMessageContainer