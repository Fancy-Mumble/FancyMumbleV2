import { Container, TextField } from '@mui/material';
import { invoke } from '@tauri-apps/api';
import { useState } from 'react';

function Chat() {
    const [chatMessage, setChatMessage] = useState("");

    function keyDownHandler(e: any) {
        if (e && e.key === 'Enter') {
            invoke('send_message', { chatMessage: chatMessage });
            setChatMessage("");
        }
    }

    return (
        <Container>
            <TextField fullWidth label="Input" value={chatMessage} onChange={e => setChatMessage(e.target.value)} onKeyDown={keyDownHandler} />
        </Container>
    )
}

export default Chat
