import { Button } from "@mui/material";
import { useNavigate } from "react-router-dom";

function Settings() {
    const navigate = useNavigate();

    return (
        <div>
            <Button onClick={e => navigate("/chat")}>Go Back</Button>
        </div>
    )
}

export default Settings;