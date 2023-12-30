import { Box, Button, Divider, List, ListItemButton, ListItemIcon, ListItemText, ListSubheader } from "@mui/material";
import React from "react";
import { useNavigate } from "react-router-dom";
import VolumeUpIcon from '@mui/icons-material/VolumeUp';
import PersonIcon from '@mui/icons-material/Person';
import KeyIcon from '@mui/icons-material/Key';
import SecurityIcon from '@mui/icons-material/Security';
import Profile from "../components/settings/Profile";
import AudioSettings from "../components/settings/Audio";
import AdditionalFeatures from "../components/settings/AdditionalFeatures";
import AodIcon from '@mui/icons-material/Aod';
import NotificationsIcon from '@mui/icons-material/Notifications';
import KeyboardIcon from '@mui/icons-material/Keyboard';
import SettingsSuggestIcon from '@mui/icons-material/SettingsSuggest';
import AdvancedSettings from "../components/settings/AdvancedSettings";

enum SettingsTab {
    Profile = 0,
    Audio = 1,
    AdditionalFeatures = 2,
    Privacy = 3,
    Appearance = 4,
    Notifications = 5,
    Hotkeys = 6,
    Advanced = 7,
}

function Settings() {
    const navigate = useNavigate();
    const [selectedIndex, setSelectedIndex] = React.useState(1);

    return (
        <Box sx={{ height: '100%', display: 'flex' }}>
            <Box sx={{ width: '100%', maxWidth: 200, bgcolor: 'background.paper', height: '100%', display: 'flex', flexDirection: 'column' }}>
                <Box sx={{ flexGrow: 1, overflow: 'auto' }}>
                    <List component="nav" aria-label="main mailbox folders" subheader={<ListSubheader>Settings</ListSubheader>}>
                        <ListItemButton
                            selected={selectedIndex === SettingsTab.Profile}
                            onClick={() => setSelectedIndex(SettingsTab.Profile)}
                        >
                            <ListItemIcon>
                                <PersonIcon />
                            </ListItemIcon>
                            <ListItemText primary="Profile" />
                        </ListItemButton>
                        <ListItemButton
                            selected={selectedIndex === SettingsTab.Audio}
                            onClick={() => setSelectedIndex(SettingsTab.Audio)}
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
                            selected={selectedIndex === SettingsTab.Appearance}
                            onClick={(event) => setSelectedIndex(SettingsTab.Appearance)}
                        >
                            <ListItemIcon>
                                <AodIcon />
                            </ListItemIcon>
                            <ListItemText primary="Appearance" />
                        </ListItemButton>
                        <ListItemButton
                            selected={selectedIndex === SettingsTab.Notifications}
                            onClick={(event) => setSelectedIndex(SettingsTab.Notifications)}
                        >
                            <ListItemIcon>
                                <NotificationsIcon />
                            </ListItemIcon>
                            <ListItemText primary="Notifications" />
                        </ListItemButton>
                        <ListItemButton
                            selected={selectedIndex === SettingsTab.Hotkeys}
                            onClick={(event) => setSelectedIndex(SettingsTab.Hotkeys)}
                        >
                            <ListItemIcon>
                                <KeyboardIcon />
                            </ListItemIcon>
                            <ListItemText primary="Hotkeys" />
                        </ListItemButton>
                    </List>
                    <Divider />
                    <List component="nav" aria-label="secondary mailbox folder">
                        <ListItemButton
                            selected={selectedIndex === SettingsTab.AdditionalFeatures}
                            onClick={(event) => setSelectedIndex(SettingsTab.AdditionalFeatures)}
                        >
                            <ListItemIcon>
                                <KeyIcon />
                            </ListItemIcon>
                            <ListItemText primary="Additional Features" />
                        </ListItemButton>
                        <ListItemButton
                            selected={selectedIndex === SettingsTab.Advanced}
                            onClick={(event) => setSelectedIndex(SettingsTab.Advanced)}
                        >
                            <ListItemIcon>
                                <SettingsSuggestIcon />
                            </ListItemIcon>
                            <ListItemText primary="Advanced" />
                        </ListItemButton>
                        <ListItemButton
                            selected={selectedIndex === SettingsTab.Privacy}
                            onClick={(event) => setSelectedIndex(SettingsTab.Privacy)}
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
                <Box sx={{ p: 0 }}>
                    {selectedIndex === SettingsTab.Audio && <AudioSettings />}
                    {selectedIndex === SettingsTab.Profile && <Profile />}
                    {selectedIndex === SettingsTab.AdditionalFeatures && <AdditionalFeatures />}
                    {selectedIndex === SettingsTab.Privacy && <div>Privacy</div>}
                    {selectedIndex === SettingsTab.Advanced && <AdvancedSettings />}
                </Box>
            </Box>
        </Box>
    )
}

export default Settings;