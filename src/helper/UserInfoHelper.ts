import DOMPurify from "dompurify";
import { UsersState } from "../store/features/users/userSlice";
import { useSelector } from "react-redux";
import { RootState } from "../store/store";

export function getBackgroundFromComment(user: UsersState | undefined, withUrl: boolean = true) {
    let defaultColor = "#0057b7";

    if (!user || !user.comment) {
        return defaultColor;
    }

    let cleanMessage = DOMPurify.sanitize(user.comment);
    const parser = new DOMParser();
    const document = parser.parseFromString(cleanMessage, "text/html");
    const images = Array.from(document.querySelectorAll('img')).map(img => img.src);
    if(images.length === 0) {
        return defaultColor;
    }

    const lastImage = images[images.length - 1];
    return withUrl ? "url(" + lastImage + ")" : lastImage;

}

export function getProfileImage(user_id: number) {
    const userList = useSelector((state: RootState) => state.reducer.userInfo);

    let userIndex = userList.users.findIndex(e => e.id === user_id);
    if (userIndex !== -1) {
        return userList.users[userIndex].profile_picture;
    }

    return "";
}