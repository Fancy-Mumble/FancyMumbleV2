import { Container, FormControl, IconButton, InputLabel, MenuItem, Select, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import LoopIcon from '@mui/icons-material/Loop';
import { useState } from "react";

function AudioSettings() {
    let [inputDevice, setInputDevice] = useState('');
    let [inputDeviceList, setInputDeviceList] = useState([]);

    function getAudioDevices() {
        invoke('get_audio_devices')
            .then((devices: any) => {
                setInputDeviceList(devices);
            })
    }

    return (
        <Container>
            <Typography variant="h3">Audio</Typography>
            <FormControl sx={{ m: 1, minWidth: 120, justifyContent: 'center' }} size="small">
                <InputLabel id="demo-simple-select-label">Input Device</InputLabel>
                <Select
                    labelId="demo-simple-select-helper-label"
                    id="demo-simple-select-helper"
                    value={inputDevice}
                    label="Input Device"
                    onChange={(e) => setInputDevice(e.target.value as any)}
                >
                    <MenuItem value="">
                        <em>None</em>
                    </MenuItem>
                    {inputDeviceList.map((value) => {
                        return (<MenuItem value={value}>{value}</MenuItem>);
                    })}
                </Select>
            </FormControl>
            <IconButton color="primary" onClick={getAudioDevices}><LoopIcon /></IconButton >
        </Container>
    )
}

export default AudioSettings;