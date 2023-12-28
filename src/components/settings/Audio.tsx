import { Box, Button, Collapse, Container, FormControl, FormControlLabel, Grid, IconButton, InputLabel, LinearProgress, MenuItem, Select, Slider, Switch, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import LoopIcon from '@mui/icons-material/Loop';
import { useState } from "react";

function AudioSettings() {
    let [inputDevice, setInputDevice] = useState('');
    let [inputDeviceList, setInputDeviceList] = useState([]);
    const [voiceHold, setVoiceHold] = useState<number>(10);
    const [fadeOutDuration, setFadeOutDuration] = useState<number>(10);
    const [advancedOptions, showAdvanceOptions] = useState(false);
    const [amplification, setAmplification] = useState<number>(10);
    const [voiceHysteresis, setVoiceHysteresis] = useState<number[]>([20, 37]);
    const [audioLevel, setAudioLevel] = useState<number>(10);

    function getAudioDevices() {
        invoke('get_audio_devices')
            .then((devices: any) => {
                setInputDeviceList(devices);
            })
    }

    function setAudioSetting() {
        invoke('set_setting', {
            settings: {
                amplification: amplification,
                voice_hold: Math.floor(calculateVoiceHold(voiceHold)),
                fade_out_duration: Math.floor(calculateVoiceHold(fadeOutDuration)),
                voice_hysteresis_lower_threshold: voiceHysteresis[0],
                voice_hysteresis_upper_threshold: voiceHysteresis[1],
            }
        });
    }

    function calculateVoiceHold(value: number) {
        return 1.2 ** value;
    }

    function valueLabelFormat(ms: number): string {
        const timeUnits = [
            { unit: 'd', value: 86400000 },
            { unit: 'h', value: 3600000 },
            { unit: 'm', value: 60000 },
            { unit: 's', value: 1000 },
            { unit: 'ms', value: 1 }
        ];

        for (const element of timeUnits) {
            if (ms >= element.value) {
                return `${(ms / element.value).toFixed(2)}${element.unit}`;
            }
        }

        return "0ms";
    }

    function handleVoiceHoldChange(event: Event, newValue: number | number[]) {
        if (typeof newValue === 'number') {
            setVoiceHold(newValue);
        }
    }

    function handleFadeOutDuration(event: Event, newValue: number | number[]) {
        if (typeof newValue === 'number') {
            setFadeOutDuration(newValue);
        }
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
            <LinearProgress variant="buffer" value={audioLevel} valueBuffer={voiceHysteresis[1]} />
            <Box>
                <FormControlLabel label="Automatically detect Microphone sensitivity" control={<Switch checked={!advancedOptions} onChange={() => showAdvanceOptions(!advancedOptions)} />} />
                <Collapse in={advancedOptions}>
                    <Grid container spacing={{ xs: 2, md: 3 }} columns={{ xs: 4, sm: 8, md: 12, lg: 18 }} sx={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'center' }}>
                        <Grid item xs={4} sm={8} md={12} lg={12}>
                            <Typography id="non-linear-slider" gutterBottom>
                                Audio hold duration: {valueLabelFormat(calculateVoiceHold(voiceHold))}
                            </Typography>
                            <Slider
                                value={voiceHold}
                                min={0}
                                step={1}
                                max={60}
                                scale={calculateVoiceHold}
                                getAriaValueText={valueLabelFormat}
                                valueLabelFormat={valueLabelFormat}
                                onChange={handleVoiceHoldChange}
                                valueLabelDisplay="auto"
                                aria-labelledby="non-linear-slider"
                            />
                        </Grid>
                        <Grid item xs={4} sm={8} md={12} lg={12}>
                            <Typography id="non-linear-slider" gutterBottom>
                                Fade-out Duration: {valueLabelFormat(calculateVoiceHold(fadeOutDuration))}
                            </Typography>
                            <Slider
                                value={fadeOutDuration}
                                min={0}
                                step={1}
                                max={60}
                                scale={calculateVoiceHold}
                                getAriaValueText={valueLabelFormat}
                                valueLabelFormat={valueLabelFormat}
                                onChange={handleFadeOutDuration}
                                valueLabelDisplay="auto"
                                aria-labelledby="non-linear-slider"
                            />
                        </Grid>
                        <Grid item xs={4} sm={8} md={12} lg={12}>
                            <Typography id="non-linear-slider" gutterBottom>
                                Amplification: +{amplification}dB
                            </Typography>
                            <Slider
                                value={amplification}
                                min={0}
                                step={1}
                                max={20}
                                onChange={(e, v) => setAmplification(v as number)}
                                valueLabelDisplay="auto"
                                aria-labelledby="non-linear-slider"
                            />
                        </Grid>
                        <Grid item xs={4} sm={8} md={12} lg={12}>
                            <Typography id="non-linear-slider" gutterBottom>
                                Voice Hysteresis: {voiceHysteresis[0]} - {voiceHysteresis[1]}
                            </Typography>
                            <Slider
                                min={0}
                                step={0.01}
                                max={3}
                                getAriaLabel={() => ''}
                                value={voiceHysteresis}
                                onChange={(e, v) => setVoiceHysteresis(v as number[])}
                                valueLabelDisplay="auto"
                                disableSwap
                            />
                        </Grid>
                    </Grid>
                </Collapse>
            </Box>
            <Box>
                <Button variant="contained" onClick={() => setAudioSetting()}>Apply</Button>
            </Box>
        </Container>
    )
}

export default AudioSettings;