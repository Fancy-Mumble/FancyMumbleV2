import { Box, Container, FormControlLabel, IconButton, List, ListItem, ListItemButton, ListItemIcon, ListItemText, Radio, RadioGroup,  Typography, } from "@mui/material";
import { useDispatch } from "react-redux";
import { useTheme } from '@mui/material/styles';
import './styles/Profile.css'
import {  useState } from "react";
import { useTranslation } from "react-i18next";


function LanguageSettings() {
    const theme = useTheme();
    const dispatch = useDispatch();
    const [t, i18n] = useTranslation();
    const [language, setLanguage] = useState(i18n.language);

    let sortedLanguages = [...i18n.languages].sort((a, b) => a.localeCompare(b));

    function updateLanguageSettings(language: string) {
        i18n.changeLanguage(language);
        setLanguage(language);
    }

    return (
        <Box>
            <Container className="settingsContainer">
                <Typography variant="h4">{t("Language")}</Typography>
                <List sx={{ width: '100%', bgcolor: 'background.paper' }}>
                    <RadioGroup
                        aria-labelledby="demo-radio-buttons-group-label"
                        defaultValue="female"
                        name="radio-buttons-group"
                        value={language}
                        onChange={(e) => updateLanguageSettings(e.target.value)}
                    >
                        {sortedLanguages.map((e) => {
                            return (
                                <ListItem
                                    key={e}
                                    secondaryAction={
                                        <IconButton edge="end" aria-label="comments">

                                        </IconButton>
                                    }
                                    disablePadding
                                >
                                    <ListItemButton role={undefined} onClick={() => updateLanguageSettings(e)} dense>
                                        <ListItemIcon>
                                        <FormControlLabel value={e} control={<Radio />} label={t(e)} />
                                        </ListItemIcon>
                                        <ListItemText primary={t(e + " native")} />
                                    </ListItemButton>
                                </ListItem>
                            );
                        })}
                    </RadioGroup>
                </List>
            </Container>
        </Box >
    )
}

export default LanguageSettings;