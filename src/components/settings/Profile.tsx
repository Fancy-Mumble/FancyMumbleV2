import { Alert, Box, Button, CircularProgress, Container, Typography } from "@mui/material";
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
            {/*<Box>Background Image: <input type="file" onChange={(e) => uploadFile(ImageType.Background, e)} /></Box>
            <Box>Profile Image: <input type="file" onChange={(e) => uploadFile(ImageType.Profile, e)} /></Box>*/}
            <UploadBox onUpload={(path) => uploadFile(path, ImageType.Background)}>{displayLoadingText("Background Image")}</UploadBox>
            <UploadBox onUpload={(path) => uploadFile(path, ImageType.Profile)}>{displayLoadingText("Profile Image")}</UploadBox>
        </Container>
    )
}

export default Profile;