import { useSelector } from "react-redux";
import { RootState } from "../store/store";
import { getBackgroundFromComment, getProfileImage } from "../helper/UserInfoHelper";
import { Avatar, Box, Container } from "@mui/material";

function CurrentUserInfo() {
    const userInfo = useSelector((state: RootState) => state.reducer.userInfo);
    return (
        <Box style={{ background: getBackgroundFromComment(userInfo.currentUser) }}>
            <Box sx={{display: 'flex', background: '#192932', justifyContent: 'space-between', padding: '2px 10px', alignItems: 'center'}}>
                <Avatar
                    alt={userInfo.currentUser?.name}
                    src={getProfileImage(userInfo.currentUser?.id || -1)}
                    sx={{ width: 32, height: 32 }}
                />
                {userInfo.currentUser?.name}
            </Box>
        </Box>)
}
export default CurrentUserInfo;