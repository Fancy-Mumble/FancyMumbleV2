import { Box, Card, CardContent, List } from "@mui/material";
import React from "react";
import ChatMessage, { TextMessage } from "./ChatMessage";


interface ChatMessageContainerProps {
	messages: TextMessage[]
}

interface ChatMessageContainerState {
}

class ChatMessageContainer extends React.Component<ChatMessageContainerProps, ChatMessageContainerState> {

	private chatContainer: React.RefObject<any> = React.createRef();
	private userScrolled: boolean = false;

	constructor(props: ChatMessageContainerProps) {
		super(props);
		this.state = { userScrolled: false }
	}

	getSnapshotBeforeUpdate(prevProps: ChatMessageContainerProps, prevState: ChatMessageContainerState) {
		if (!this.chatContainer.current) {
			return;
		}

		let el = this.chatContainer.current;
		if (el.scrollTop < el.scrollHeight - el.clientHeight) {
			this.userScrolled = true;
		} else {
			this.userScrolled = false;
		}
	}

	componentDidUpdate(prevProps: ChatMessageContainerProps) {
		if (!this.userScrolled && this.chatContainer.current) {
			let element = this.chatContainer.current;
			element.scrollTop = element.scrollHeight - element.clientHeight;
		}
	}

	render() {
		return (
			<Box sx={{ flex: 1, overflowY: 'auto' }} ref={this.chatContainer}>
				<List sx={{ width: '100%', maxWidth: 360 }}>
					{this.props.messages.map((el, index) => (<ChatMessage key={el.timestamp} message={el} />))}
				</List>
			</Box>
		);
	}
}
export default ChatMessageContainer