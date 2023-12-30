import { Alert, Box,  CircularProgress, Container, Divider, Grid, TextField, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import UploadBox from "../UploadBox";
import React, { useState } from 'react';
import DefaultColorPicker from "../ColorPicker";
import { ColorResult } from "react-color";
import UserInfo from "../UserInfo";
import { RootState } from "../../store/store";
import { useDispatch, useSelector } from "react-redux";
import { UpdateableUserState, UsersState, updateUserSettings } from "../../store/features/users/userSlice";
import { encodeUserCommentData } from "../../helper/ProfileDataHelper";
import { useTheme } from '@mui/material/styles';
import './styles/Profile.css'
import FloatingApply from "./components/FloatingApply";

enum ImageType {
    Profile = 'profile',
    Background = 'background'
}

function Profile() {
    const theme = useTheme();
    const dispatch = useDispatch();

    const userInfo = useSelector((state: RootState) => state.reducer.userInfo).currentUser;


    let [errorMessage, setErrorMessage] = useState('');
    let [loading, setLoading] = useState(false);
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

    async function updateUserValue(update: (currentUser: UsersState, operator: UpdateableUserState) => void) {
        if (userInfo) {
            let currentUserClone: UpdateableUserState = { id: userInfo.id };

            await update(userInfo, currentUserClone);
            //await invoke('change_user_state', { userState: currentUserClone }).catch(e => console.log(e));
        }
    }

    async function setPrimaryColorCall(color: ColorResult) {
        setPrimaryColor(color);
        if (userInfo) {
            let userData = {
                ...userInfo.commentData,
                settings:
                {
                    ...userInfo.commentData?.settings,
                    primary_color: color.hex
                }
            };

            updateUserValue(async (currentUser, currentUserClone) => {
                let encoded = await encodeUserCommentData(currentUser.comment, userData);
                currentUserClone.comment = encoded;
            });

            dispatch(updateUserSettings({ user_id: userInfo.id, settings: { ...userInfo.commentData?.settings, primary_color: color.hex } }));
        }
    }

    function setAccentColorCall(color: ColorResult) {
        setAccentColor(color);
        if (userInfo) {
            /*let userData = {
                ...userInfo.commentData,
                settings:
                {
                    ...userInfo.commentData?.settings,
                    accentColor: color.hex
                }
            };

            updateUserValue(async (currentUser, currentUserClone) => {
                let encoded = await encodeUserCommentData(currentUser.comment, userData);
                currentUserClone.comment = encoded;
            });*/

            //dispatch(updateUserSettings({ user_id: userInfo.id, settings: { ...userInfo.commentData?.settings, accent_color: color.hex } }));
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
        <Box>
            <Container className="settingsContainer">
                {showErrorMessage()}
                <Typography variant="h4">Profile</Typography>
                <Divider sx={{ marginBottom: 5 }} />
                <Grid container spacing={{ xs: 2, md: 3 }} columns={{ xs: 4, sm: 8, md: 12, lg: 18 }} sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
                    <Grid item xs={12} sm={12} md={6} lg={12} sx={{ padding: 2 }}>
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
                            <DefaultColorPicker color={primaryColor} onChangeComplete={(color) => setPrimaryColorCall(color)} description="Primary" style={{ marginRight: 15 }} />
                            <DefaultColorPicker color={accentColor} onChangeComplete={(color) => setAccentColorCall(color)} description="Accent" style={{ marginRight: 15 }} />
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
                    </Grid>
                    <Grid item xs={12} sm={12} md={6} lg={6} sx={{ display: 'flex', justifyContent: 'center' }}>
                        <UserInfo
                            userInfo={userInfo}
                            style={{ position: 'sticky', top: theme.spacing(2) }}
                        />
                    </Grid>
                </Grid>
            </Container >
            <FloatingApply discardText="Discard" saveText="Save" onDiscard={() => { }} onSave={() => { }} />
        </Box >
    )
}

export default Profile;