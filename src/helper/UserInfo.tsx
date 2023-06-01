import { UpdateableUserState, UsersState, updateUserFromUpdateable } from "../store/features/users/userSlice";
import { useDispatch, useSelector } from 'react-redux';
import { RootState } from "../store/store";
import { invoke } from "@tauri-apps/api";

export function updateUserValue(currentUser: UsersState | undefined, update: (currentUser: UsersState, operator: UpdateableUserState) => void) {
    const dispatch = useDispatch();

    if (currentUser) {
        let currentUserClone: UpdateableUserState = { id: currentUser.id };

        update(currentUser, currentUserClone);
        invoke('change_user_state', { userState: currentUserClone });

        dispatch(updateUserFromUpdateable(currentUserClone));
    }
}