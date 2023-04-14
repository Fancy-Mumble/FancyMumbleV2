import { Box, Card, CardContent, Container, TextField } from '@mui/material';
import { invoke } from '@tauri-apps/api';
import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event'


function Chat() {
    const [chatMessage, setChatMessage] = useState("");
    const [messageLog, setMessageLog] = useState<any[]>([]);
    useEffect(() => {
        //listen to a event
        const unlisten = listen("text_message", (e) => {
            let message = JSON.parse(e.payload as any);
            console.log(message);
            setMessageLog(messageLog => [...messageLog, message]);
        });

        return () => {
            unlisten.then(f => f());
        }
    });

    function keyDownHandler(e: any) {
        if (e && e.key === 'Enter') {
            invoke('send_message', { chatMessage: chatMessage });
            setChatMessage("");
        }
    }

    return (
        <Container>
            <Container>
                {messageLog.map((el, index) => (<Box mb={2}><Card key={"message_" + index}><CardContent><strong>{el.actor}:</strong> {el.message}</CardContent></Card></Box>))}
            </Container>
            <TextField fullWidth label="Input" value={chatMessage} onChange={e => setChatMessage(e.target.value)} onKeyDown={keyDownHandler} />
        </Container>
    )
}

export default Chat
