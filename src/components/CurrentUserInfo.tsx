import { useSelector } from "react-redux";
import { RootState } from "../store/store";
import { getBackgroundFromComment, getProfileImage } from "../helper/UserInfoHelper";
import { Avatar, Container } from "@mui/material";

function CurrentUserInfo() {
    const userInfo = useSelector((state: RootState) => state.reducer.userInfo);
    return (
        <Container style={{background: getBackgroundFromComment(userInfo.currentUser)}}>
            <Avatar
                alt="Remy Sharp"
                src={getProfileImage(userInfo.currentUser?.id || -1)}
                sx={{ width: 24, height: 24 }}
            />
            {userInfo.currentUser?.name}
        </Container>)
}
export default CurrentUserInfo;