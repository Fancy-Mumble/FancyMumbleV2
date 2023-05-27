import { Container, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api";

function AudioSettings() {
    function getAudioDevices() {
        invoke('get_audio_devices')
            .then((devices: any) => {
                console.log(devices);
            })
    }

    return (
        <Container>
            <Typography variant="h3">Audio</Typography>
        </Container>
    )
}

export default AudioSettings;