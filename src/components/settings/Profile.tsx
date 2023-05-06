import { Box, Container, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import { ChangeEvent } from "react";

enum ImageType {
    Profile = 'profile',
    Background = 'background'
}

function Profile() {
    function uploadFile(type: ImageType, event: ChangeEvent<HTMLInputElement>) {
        if (!event.target.files || event.target.files.length === 0) return;

        let file = event.target.files[0];

        if (file) {
            //TODO: Just upload the path to the file, not the whole file and let the backend handle it
            let reader = new FileReader();
            reader.readAsDataURL(file);

            reader.onload = function () {
                let base64String = reader.result?.toString().split(',')[1];
                invoke('set_user_image', { image: base64String, imageType: type });
            };
        }
    }

    return (
        <Container>
            <Typography variant="h3">Profile</Typography>
            <Box>Background Image: <input type="file" onChange={(e) => uploadFile(ImageType.Background, e)} /></Box>
            <Box>Profile Image: <input type="file" onChange={(e) => uploadFile(ImageType.Profile, e)} /></Box>
        </Container>
    )
}

export default Profile;