import { Box, Button, Container, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import UploadBox from "../UploadBox";

enum ImageType {
    Profile = 'profile',
    Background = 'background'
}

function Profile() {
    async function uploadFile(path: string, type: ImageType) {
                invoke('set_user_image', { imagePath: path, imageType: type });
    }

    return (
        <Container>
            <Typography variant="h3">Profile</Typography>
            {/*<Box>Background Image: <input type="file" onChange={(e) => uploadFile(ImageType.Background, e)} /></Box>
            <Box>Profile Image: <input type="file" onChange={(e) => uploadFile(ImageType.Profile, e)} /></Box>*/}
            <UploadBox onUpload={(path) => uploadFile(path, ImageType.Background)}>Background Image</UploadBox>
            <UploadBox onUpload={(path) => uploadFile(path, ImageType.Profile)}>Profile Image</UploadBox>
        </Container>
    )
}

export default Profile;