import { Box, Button, Divider, List, ListItemButton, ListItemIcon, ListItemText, ListSubheader } from "@mui/material";
import React from "react";
import { useNavigate } from "react-router-dom";
import VolumeUpIcon from '@mui/icons-material/VolumeUp';
import PersonIcon from '@mui/icons-material/Person';
import KeyIcon from '@mui/icons-material/Key';
import SecurityIcon from '@mui/icons-material/Security';
import Profile from "../components/settings/Profile";
import AudioSettings from "../components/settings/Audio";

function Settings() {
    const navigate = useNavigate();
    const [selectedIndex, setSelectedIndex] = React.useState(1);

    return (
        <Box sx={{ height: '100%', display: 'flex' }}>
            <Box sx={{ width: '100%', maxWidth: 360, bgcolor: 'background.paper', height: '100%', display: 'flex', flexDirection: 'column' }}>
                <Box sx={{ flexGrow: 1, overflow: 'auto' }}>
                    <List component="nav" aria-label="main mailbox folders" subheader={<ListSubheader>Settings</ListSubheader>}>
                        <ListItemButton
                            selected={selectedIndex === 1}
                            onClick={() => setSelectedIndex(1)}
                        >
                            <ListItemIcon>
                                <PersonIcon />
                            </ListItemIcon>
                            <ListItemText primary="Profile" />
                        </ListItemButton>
                        <ListItemButton
                            selected={selectedIndex === 0}
                            onClick={() => setSelectedIndex(0)}
                        >
                            <ListItemIcon>
                                <VolumeUpIcon />
                            </ListItemIcon>
                            <ListItemText primary="Audio" />
                        </ListItemButton>
                    </List>
                    <Divider />
                    <List component="nav" aria-label="secondary mailbox folder">
                        <ListItemButton
                            selected={selectedIndex === 2}
                            onClick={(event) => setSelectedIndex(2)}
                        >
                            <ListItemIcon>
                                <KeyIcon />
                            </ListItemIcon>
                            <ListItemText primary="Additional Features" />
                        </ListItemButton>
                        <ListItemButton
                            selected={selectedIndex === 3}
                            onClick={(event) => setSelectedIndex(3)}
                        >
                            <ListItemIcon>
                                <SecurityIcon />
                            </ListItemIcon>
                            <ListItemText primary="Privacy" />
                        </ListItemButton>
                    </List>
                </Box>
                <Box>
                    <Button onClick={e => navigate("/chat")}>Go Back</Button>
                </Box>
            </Box>
            <Box sx={{ flexGrow: 1, overflow: 'auto' }}>
                <Box sx={{ p: 3 }}>
                    {selectedIndex === 0 && <AudioSettings />}
                    {selectedIndex === 1 && <Profile />}
                    {selectedIndex === 2 && <div>Additional Features</div>}
                    {selectedIndex === 3 && <div>Privacy</div>}
                </Box>
            </Box>
        </Box>
    )
}

export default Settings;