import { useSelector } from "react-redux";
import { RootState } from "../store/store";
import { getBackgroundFromComment, getProfileImage } from "../helper/UserInfoHelper";
import { Avatar, Badge, Box, Container } from "@mui/material";
import ChannelSearch from "./ChannelSearch";

function CurrentUserInfo() {
    const userInfo = useSelector((state: RootState) => state.reducer.userInfo);
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
                    {userInfo.currentUser?.name ?? 'Unknown'}
                </Box>
                <ChannelSearch />
            </Box>
        </Box>)
}
export default CurrentUserInfo;