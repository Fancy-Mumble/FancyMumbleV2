import { Alert, Box, Button, CircularProgress, Container, FormControl, InputLabel, MenuItem, Select, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import UploadBox from "../UploadBox";
import React, { useState } from 'react';

enum ImageType {
    Profile = 'profile',
    Background = 'background'
}

function Profile() {
    let [errorMessage, setErrorMessage] = useState('');
    let [loading, setLoading] = useState(false);
    let [profilePicResolution, setProfilePicResolution] = useState(0);

    function showErrorMessage() {
        if (errorMessage) {
            return <Alert severity="error">{errorMessage}</Alert>
        }
    }

    function displayLoadingText(text: string) {
        if (loading) {
            return (<CircularProgress />)
        } else {
            return (<Typography>{text}</Typography>)
        }
    }

    function gennerateResolutions(length: number) {
        return Array.from({ length: length }, (value, index) => Math.pow(2, 5 + index));
    }

    async function uploadFile(path: string, type: ImageType) {
        setErrorMessage('');
        setLoading(true);
        invoke('set_user_image', { imagePath: path, imageType: type })
            .catch(e => {
                setErrorMessage(e);
            })
            .finally(() => {
                setLoading(false);
            });
    }

    return (
        <Container>
            {showErrorMessage()}
            <Typography variant="h3">Profile</Typography>
            <Box sx={{
                display: 'flex',
                flexDirection: 'row',
                alignContent: 'center',
                maxWidth: '100%'
            }}>
                <UploadBox onUpload={(path) => uploadFile(path, ImageType.Background)}>{displayLoadingText("Background Image")}</UploadBox>
                <FormControl sx={{ m: 1, minWidth: 120, justifyContent: 'center' }} size="small">
                    <InputLabel id="demo-select-small-label">Size</InputLabel>
                    <Select
                        labelId="demo-simple-select-helper-label"
                        id="demo-simple-select-helper"
                        value={profilePicResolution}
                        label="Age"
                        onChange={(e) => setProfilePicResolution(e.target.value as any)}
                    >
                        <MenuItem value="">
                            <em>None</em>
                        </MenuItem>
                        {gennerateResolutions(5).map((value) => {
                            return (<MenuItem value={value}>{value}px</MenuItem>);
                        })}
                    </Select>
                </FormControl>
            </Box>
            <Box sx={{
                display: 'flex',
                flexDirection: 'row',
                alignContent: 'center',
                maxWidth: '100%'
            }}>
                <UploadBox onUpload={(path) => uploadFile(path, ImageType.Profile)}>{displayLoadingText("Profile Image")}</UploadBox>
                <FormControl sx={{ m: 1, minWidth: 120, justifyContent: 'center' }} size="small">
                    <InputLabel id="demo-select-small-label">Size</InputLabel>
                    <Select
                        labelId="demo-simple-select-helper-label"
                        id="demo-simple-select-helper"
                        value={profilePicResolution}
                        label="Age"
                        onChange={(e) => setProfilePicResolution(e.target.value as any)}
                    >
                        <MenuItem value="">
                            <em>None</em>
                        </MenuItem>
                        {gennerateResolutions(5).map((value) => {
                            return (<MenuItem value={value}>{value}px</MenuItem>);
                        })}
                    </Select>
                </FormControl>
            </Box>
        </Container >
    )
}

export default Profile;