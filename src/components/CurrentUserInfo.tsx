import { useSelector } from "react-redux";
import { RootState } from "../store/store";
import { getBackgroundFromComment, getProfileImage } from "../helper/UserInfoHelper";
import { Avatar, Badge, Box, Container } from "@mui/material";
import ChannelSearch from "./ChannelSearch";
import MicIcon from '@mui/icons-material/Mic';
import './styles/CurrentUserInfo.css'
import { invoke } from "@tauri-apps/api";

function CurrentUserInfo() {
    const userInfo = useSelector((state: RootState) => state.reducer.userInfo);

    function muteToggleUser() {
        if (userInfo.currentUser) {
            //TODO: This won't work, because we don't have a setter for the currentUser (mute) property
            userInfo.currentUser.mute = !userInfo.currentUser.mute;
            invoke('change_user_state', { userId: userInfo.currentUser.id, userInfo: userInfo.currentUser });
        }
    }

    return (
        <Box style={{
            background: getBackgroundFromComment(userInfo.currentUser),
            display: 'flex',
            padding: '0 10px',
            backgroundSize: 'cover',
        }}>
            <Box sx={{
                display: 'flex',
                flexDirection: 'column',
                justifyContent: 'center',
            }}>
                <Avatar
                    alt={userInfo.currentUser?.name}
                    src={getProfileImage(userInfo.currentUser?.id || -1)}
                    sx={{ width: 48, height: 48 }}
                />
            </Box>
            <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'flex-end' }}>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', padding: '2px 10px', flexDirection: 'column', alignItems: 'center', width: '100%', textShadow: '1px 1px #000' }}>
                    <Box sx={{ display: 'flex', flexDirection: 'row', alignItems: 'center' }}>
                        {userInfo.currentUser?.name ?? 'Unknown'}
                        <MicIcon onClick={() => muteToggleUser()} className="small_icon" />
                    </Box>
                </Box>
                <ChannelSearch />
            </Box>
        </Box>)
}
export default CurrentUserInfo;