import { useDispatch, useSelector } from "react-redux";
import { RootState } from "../store/store";
import { getBackgroundFromComment, getProfileImage } from "../helper/UserInfoHelper";
import { Avatar, Badge, Box, Container } from "@mui/material";
import ChannelSearch from "./ChannelSearch";
import MicIcon from '@mui/icons-material/Mic';
import VolumeUpIcon from '@mui/icons-material/VolumeUp';
import VolumeOffIcon from '@mui/icons-material/VolumeOff';
import MicOffIcon from '@mui/icons-material/MicOff';
import './styles/CurrentUserInfo.css'
import { invoke } from "@tauri-apps/api";
import { UpdateableUserState, UsersState, updateUser, updateUserFromUpdateable } from "../store/features/users/userSlice";
import "./styles/common.css"

function CurrentUserInfo() {
    const dispatch = useDispatch();
    const userInfo = useSelector((state: RootState) => state.reducer.userInfo);
    const userBackground = getBackgroundFromComment(userInfo.currentUser);

    function updateUserValue(update: (currentUser: UsersState, operator: UpdateableUserState) => void) {
        if (userInfo.currentUser) {
            let currentUser = userInfo.currentUser;
            let currentUserClone: UpdateableUserState = { id: currentUser.id  };

            update(currentUser, currentUserClone);
            invoke('change_user_state', { userState: currentUserClone });
        }
    }

    function muteToggleUser() {
        updateUserValue((currentUser, currentUserClone) => currentUserClone.self_mute = !currentUser.self_mute);
    }

    function deafToggleUser() {
        updateUserValue((currentUser, currentUserClone) => currentUserClone.self_deaf = !currentUser.self_deaf);
    }

    function microphoneState() {
        if (userInfo.currentUser?.self_mute) {
            return (<MicOffIcon className="small_icon" />)
        } else {
            return (<MicIcon className="small_icon" />)
        }
    }

    function volumeState() {
        if (userInfo.currentUser?.self_deaf) {
            return (<VolumeOffIcon className="small_icon" />)
        } else {
            return (<VolumeUpIcon className="small_icon" />)
        }
    }

    return (
        <Box style={{
            background: userBackground,
            display: 'flex',
            padding: '0 10px',
            backgroundSize: 'cover',
        }}
        className={userBackground ? "" : "animated-background"}>
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
                        <Box onClick={() => muteToggleUser()}>
                            {microphoneState()}
                        </Box>
                        <Box onClick={() => deafToggleUser()}>
                            {volumeState()}
                        </Box>
                    </Box>
                </Box>
                <ChannelSearch />
            </Box>
        </Box>)
}
export default CurrentUserInfo;