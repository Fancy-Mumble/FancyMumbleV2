import { Avatar, Box, Card, CardContent, Grid, List } from "@mui/material";
import React, { ReactElement, useEffect, useMemo, useRef, useState } from "react";
import { MemoChatMessage } from "./ChatMessage";
import { useSelector } from "react-redux";
import { RootState } from "../store/store";
import { TextMessage } from "../store/features/users/chatMessageSlice";
import { UsersState } from "../store/features/users/userSlice";
import { getProfileImage } from "../helper/UserInfoHelper";
import UserInfoPopover from "./UserInfoPopover";

interface ChatMessageContainerProps {
	messages: TextMessage[]
}

interface GroupedMessages {
	user: UsersState | undefined,
	messages: Array<ReactElement>
}


const ChatMessageContainer = (props: ChatMessageContainerProps) => {
	const userList = useSelector((state: RootState) => state.reducer.userInfo);
	const chatContainer: React.RefObject<HTMLDivElement> = React.createRef();
	const messagesEndRef: React.RefObject<HTMLDivElement> = React.createRef();
	const [userInfoAnchor, setUserInfoAnchor] = React.useState<HTMLElement | null>(null);
	const [currentPopoverUserId, setCurrentPopoverUserId]: any = useState(null);
	const [userScrolled, setUserScrolled] = useState(false);
	const prevPropsRef = useRef(props);

	const scrollToBottom = () => {
		//add some minor sleep to make sure the element is rendered
		new Promise(r => setTimeout(r, 100)).then(() => {
			messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
		});
	}

	useEffect(() => {
		if (chatContainer.current) {
			let el = chatContainer.current;
			if (el.scrollTop < el.scrollHeight - el.clientHeight * 2) {
				console.log("User scrolled", el.scrollTop, el.scrollHeight - el.clientHeight);
				setUserScrolled(true);
			} else {
				setUserScrolled(false);
			}
		}
	}, [props, prevPropsRef]); // Depend on props

	useEffect(() => {
		if (!userScrolled && chatContainer?.current) {
			scrollToBottom();
		}
	}, [props, userScrolled]); // Depend on props and userScrolled

	const userIdToUserMap = useMemo(() => {
		if (!userList) return new Map<number, UsersState>();

		const map = new Map<number, UsersState>();
		userList.users.forEach(user => map.set(user.id, user));
		return map;
	}, [userList]);

	const memoizedMessages = useMemo(() => {
		if (!props) return [];

		let groupedMessages: Array<GroupedMessages> = [];
		let prevUser: UsersState | undefined = undefined;

		props.messages.forEach((el) => {
			let currentUser = userIdToUserMap.get(el.sender.user_id);
			if (currentUser?.id !== prevUser?.id || groupedMessages.length === 0) {
				groupedMessages.push({ user: currentUser, messages: [] });
			}

			groupedMessages[Math.max(0, groupedMessages.length - 1)].messages.push(
				<MemoChatMessage
					messageId={el.timestamp}
					key={el.timestamp}
					message={el}
				/>
			);

			prevUser = currentUser;
		});

		return groupedMessages;
	}, [props.messages]);

	const userIdToPopoverMap = useMemo(() => {
		const popoverMap = new Map<number, ReactElement>();
		userIdToUserMap.forEach((user, id) => {
			popoverMap.set(id,
				<UserInfoPopover
					anchorEl={userInfoAnchor}
					onClose={() => {
						setUserInfoAnchor(null);
						setCurrentPopoverUserId(null);
					}}
					userInfo={user}
				/>);
		});
		return popoverMap;
	}, [userIdToUserMap, userInfoAnchor]);

	return (
		<Box sx={{ flex: 1, overflowY: 'auto' }} ref={chatContainer}>
			<List sx={{ width: '100%', maxWidth: '100%' }}>
				{memoizedMessages.map((group, index) => (
					<Grid container className="message-root" key={index} sx={{ width: '100%', flexWrap: 'nowrap' }}>
						<Grid item >
							<Avatar
								sx={{ position: 'sticky', top: 10 }}
								className="avatar"
								src={getProfileImage(group.user?.id || 0, userList)}
								onClick={e => { setCurrentPopoverUserId(group.user?.id); setUserInfoAnchor(e.currentTarget); console.log(e.currentTarget) }}
								variant="rounded"
							/>
						</Grid>
						<Grid item sx={{ flexGrow: 1, display: 'flex', flexDirection: 'column' }}>
							{group.messages}
						</Grid>
					</Grid>

				))}
			</List>
			{currentPopoverUserId && userIdToPopoverMap.get(currentPopoverUserId)}
			<div ref={messagesEndRef} />
		</Box>
	);
}

export default React.memo(ChatMessageContainer);