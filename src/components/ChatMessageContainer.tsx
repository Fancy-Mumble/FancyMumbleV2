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
	private messagesEndRef: React.RefObject<HTMLDivElement> = React.createRef();

	constructor(props: ChatMessageContainerProps) {
		super(props);
		this.state = { userScrolled: false }
	}

	scrollToBottom() {
		//add some minor sleep to make sure the element is rendered
		new Promise(r => setTimeout(r, 100)).then(() => {
			this.messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
		});
	}

	getSnapshotBeforeUpdate(prevProps: ChatMessageContainerProps, prevState: ChatMessageContainerState) {
		if (this.chatContainer.current) {
			let el = this.chatContainer.current;
			if (el.scrollTop < el.scrollHeight - el.clientHeight * 2) {
				console.log("User scrolled", el.scrollTop, el.scrollHeight - el.clientHeight);
				this.userScrolled = true;
			} else {
				this.userScrolled = false;
			}
		}
		console.log("Update?", this.userScrolled);

		return null;
	}

	componentDidUpdate(prevProps: ChatMessageContainerProps) {
		if (!this.userScrolled && this.chatContainer.current) {
			//let element = this.chatContainer.current;
			//element.scrollTop = element.scrollHeight - element.clientHeight;
			this.scrollToBottom();
		}
	}

	render() {
		return (
			<Box sx={{ flex: 1, overflowY: 'auto' }} ref={this.chatContainer}>
				<List sx={{ width: '100%', maxWidth: '100%' }}>
					{this.props.messages.map((el, index) => (<ChatMessage key={el.timestamp} message={el} />))}
				</List>
				<div ref={this.messagesEndRef} />
			</Box>
		);
	}
}
export default ChatMessageContainer