import { Alert, Box, Button, CircularProgress, Container, Divider, FormControl, InputLabel, MenuItem, Select, TextField, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import UploadBox from "../UploadBox";
import React, { useState } from 'react';
import DefaultColorPicker from "../ColorPicker";
import { HSLColor } from "react-color";
import UserInfo from "../UserInfo";
import { RootState } from "../../store/store";
import { useSelector } from "react-redux";

enum ImageType {
    Profile = 'profile',
    Background = 'background'
}

function Profile() {
    const userInfo = useSelector((state: RootState) => state.reducer.userInfo).currentUser;

    let [errorMessage, setErrorMessage] = useState('');
    let [loading, setLoading] = useState(false);
    let [profilePicResolution, setProfilePicResolution] = useState(0);
    const [primaryColor, setPrimaryColor] = React.useState({
        hex: '#ffffff'
    })
    const [accentColor, setAccentColor] = React.useState({
        hex: '#ffffff'
    })

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
        <Container sx={{ maxWidth: '600px' }}>
            {showErrorMessage()}
            <Typography variant="h3" sx={{ marginBottom: 5 }}>Profile</Typography>
            <Box sx={{ display: "flex", flexDirection: "row" }}>
                <Box sx={{ flexGrow: 1, marginRight: 6 }}>
                    <Typography variant="h5">Images</Typography>
                    <Box sx={{
                        display: 'flex',
                        flexDirection: 'row',
                        alignContent: 'center',
                        maxWidth: '100%'
                    }}>
                        <UploadBox onUpload={(path) => uploadFile(path, ImageType.Background)}>{displayLoadingText("Background Image")}</UploadBox>
                    </Box>
                    <Box sx={{
                        display: 'flex',
                        flexDirection: 'row',
                        alignContent: 'center',
                        maxWidth: '100%'
                    }}>
                        <UploadBox onUpload={(path) => uploadFile(path, ImageType.Profile)}>{displayLoadingText("Profile Image")}</UploadBox>
                    </Box>
                    <Typography variant="h5" sx={{ marginTop: 4, marginBottom: 1 }}>Colors</Typography>
                    <Box sx={{ display: 'flex' }}>
                        <DefaultColorPicker color={primaryColor} onChangeComplete={(color) => setPrimaryColor(color)} description="Primary" style={{ marginRight: 15 }} />
                        <DefaultColorPicker color={accentColor} onChangeComplete={(color) => setAccentColor(color)} description="Accent" style={{ marginRight: 15 }} />
                    </Box>
                    <Typography variant="h5" sx={{ marginTop: 4, marginBottom: 1 }}>About Me</Typography>
                    <Box sx={{ display: 'flex' }}>
                        <TextField
                            placeholder="Tell us about yourself!"
                            rows={4}
                            multiline
                            sx={{ flexGrow: 1, marginRight: 2 }}
                        />
                    </Box>
                </Box>
                <Box>
                    <UserInfo
                        userInfo={userInfo}
                    />
                </Box>
            </Box>
        </Container >
    )
}

export default Profile;