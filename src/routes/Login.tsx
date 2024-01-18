import { useEffect, useMemo, useState } from 'react'
import '../App.css';
import './styles/Login.css';
import { Accordion, AccordionDetails, AccordionSummary, Alert, Avatar, Box, ButtonGroup, Button, Container, Grid, IconButton, LinearProgress, List, ListItem, ListItemAvatar, ListItemButton, ListItemIcon, ListItemText, MenuItem, Select, TextField, Tooltip, Typography, Menu } from '@mui/material'
import LoadingButton from '@mui/lab/LoadingButton';
import { invoke } from '@tauri-apps/api/tauri'
import { useLocation, useNavigate } from 'react-router-dom';
import { useDispatch, useSelector } from 'react-redux';
import { RootState } from '../store/store';
import React from 'react';
import StorageIcon from '@mui/icons-material/Storage';
import SendIcon from '@mui/icons-material/Send';
import UnfoldMoreIcon from '@mui/icons-material/UnfoldMore';
import { n } from '@tauri-apps/api/fs-4bb77382';
import { useTranslation } from 'react-i18next';
import LanguageIcon from '@mui/icons-material/Language';


interface ServerEntry {
    description: string,
    host: string,
    port: number,
    username: string,
}

function Login() {

    const userInfo = useSelector((state: RootState) => state.reducer.userInfo);
    const [expanded, setExpanded] = React.useState<string | false>('panel1');

    const [description, setDescription] = useState("Magical Rocks");
    const [server, setServer] = useState("magical.rocks");
    const [port, setPort] = useState("64738");
    const [username, setUsername] = useState("Endor");
    const [identity, setIdentity] = useState("none");
    const [identityCerts, setIdentityCerts] = useState(new Array<string>());
    const [connecting, setConnecting] = useState(false);
    const [errorInfo, setErrorInfo] = useState({ show: false, text: "" });
    const [serverInfo, setServerInfo] = useState({ show: false, text: "" });
    const [showAdditionalOptions, setShowAdditionalOptions] = useState(false);
    const [languageMenuAnchorEl, setLanguageMenuAnchorEl] = React.useState<null | HTMLElement>(null);
    const languageMenuOpen = Boolean(languageMenuAnchorEl);

    const [serverList, setServerList] = useState<ServerEntry[]>([]);

    const location = useLocation();
    const dispatch = useDispatch();
    const navigate = useNavigate();
    const { t, i18n } = useTranslation();

    useEffect(() => {
        console.log("effect: " + location.pathname);
        switch (location.pathname) {
            case "/":
                console.log("logout triggered");
                dispatch({ type: "logout" });
                break;
            default:
                break;
        }
    }, [location]);

    useEffect(() => {
        invoke('get_server_list').then((e: any) => {
            setServerList(e);
        }).catch(e => {
            console.log("error getting server list: ", e);
        });

        invoke('get_identity_certs').then((e: any) => {
            console.log("identity certs: ", e);
            setIdentityCerts(e);
        });
    }, []);

    //TODO: We shouldn't just have a binary connected state,
    //TODO: but a state that can have multiple values, like "connecting",
    //TODO: "connected", "disconnected", "error"
    if (userInfo.connected) {
        console.log("connected");
        navigate("/chat");
    }

    function connect(serverHost: string = server, serverPort: number = parseInt(port), serverUsername: string = username) {
        console.log("connecting to server: ", serverHost, serverPort, serverUsername);
        setConnecting(true);
        setErrorInfo({ show: false, text: "" });

        invoke('connect_to_server', { serverHost: serverHost, serverPort: serverPort, username: serverUsername }).then(e => {
            setConnecting(false);
        }).catch(e => {
            setErrorInfo({ show: true, text: e });
            setConnecting(false);
        });
    }

    function saveServer() {
        setErrorInfo({ show: false, text: "" });
        setServerInfo({ show: false, text: "" });

        invoke('save_server', { description: description, serverHost: server, serverPort: parseInt(port), username: username }).then(e => {
            setServerInfo({ show: true, text: "Server saved" });
            setServerList([...serverList, { description: description, host: server, port: parseInt(port), username: username }]);
        }).catch(e => {
            console.log("error saving server: ", e);
            setErrorInfo({ show: true, text: e });
        })
    }

    const handleChange =
        (panel: string) => (event: React.SyntheticEvent, newExpanded: boolean) => {
            setExpanded(newExpanded ? panel : false);
        };

    let errorBox = errorInfo.show ? (<Box mb={3}><Alert severity="error">{errorInfo.text}</Alert></Box>) : (<div></div>);
    let serverAddInfoBoxBox = serverInfo.show ? (<Box mb={3} mt={-2}><Alert severity="info">{serverInfo.text}</Alert></Box>) : (<div></div>);
    let connectionLoading = connecting ? (<LinearProgress />) : (<div></div>);

    let additionalOptions = useMemo(() => {
        if (showAdditionalOptions) {
            return (
                <Grid item={true} xs={12}>
                    <Box mt={2}>
                        <ButtonGroup variant="outlined" aria-label="outlined button group">
                            <Button>One</Button>
                            <Select fullWidth label="Identity" value={identity} onChange={e => setIdentity(e.target.value)}>
                                {
                                    identityCerts.map((e) => {
                                        return (
                                            <MenuItem key={e} value={e}>{e}</MenuItem >
                                        )
                                    })
                                }
                            </Select>
                        </ButtonGroup>
                    </Box>
                </Grid>
            )
        }
        return null;
    }, [showAdditionalOptions, identity]);

    return (
        <Box sx={{ height: '100%', display: 'flex', maxHeight: '100%', overflow: 'hidden' }}>
            <Box className='login' sx={{ height: '100%', maxWidth: '40%', minWidth: '500px', marginLeft: 2 }}>
                <Box sx={{ flexGrow: 1, alignContent: "center", justifyContent: "center", display: "flex", flexDirection: "column" }}>
                    <Typography
                        align='center'
                        variant='h3'
                        gutterBottom
                        sx={{
                            fontFamily: 'Comfortaa',
                            fontWeight: 'bold',
                            background: '-webkit-linear-gradient(right, #667db6, #0082c8, #0082c8, #667db6)',
                            WebkitBackgroundClip: 'text',
                            WebkitTextFillColor: 'transparent',
                        }}>
                        {t('Fancy Mumble Title')}
                    </Typography >
                    {errorBox}
                    <Accordion expanded={expanded === 'panel1'} onChange={handleChange('panel1')}>
                        <AccordionSummary aria-controls="panel2d-content" id="panel2d-header">
                            <Typography>{t('User Profiles')}</Typography>
                        </AccordionSummary>
                        <AccordionDetails>
                            {connectionLoading}
                            <List>
                                {serverList.map((e) => {
                                    return (
                                        <ListItem disablePadding key={(e.host || '') + (e.port || '') + (e.username || '')}>
                                            <ListItemButton onClick={() => connect(e.host, e.port, e.username)}>
                                                <ListItemAvatar>
                                                    <Avatar>
                                                        <StorageIcon />
                                                    </Avatar>
                                                </ListItemAvatar>
                                                <ListItemText primary={e.description} />
                                            </ListItemButton>
                                        </ListItem>)
                                })}
                            </List>
                        </AccordionDetails>
                    </Accordion>

                    <Accordion expanded={expanded === 'panel2'} onChange={handleChange('panel2')}>
                        <AccordionSummary aria-controls="panel1d-content" id="panel1d-header">{t('Add New Server')}</AccordionSummary>
                        <AccordionDetails>
                            {serverAddInfoBoxBox}
                            <Container className='login-form'>
                                <Grid container spacing={1}>
                                    <Grid item={true} xs={12}>
                                        <TextField fullWidth label="Description" value={description} onChange={e => setDescription(e.target.value)} />
                                    </Grid>
                                    <Grid item={true} xs={8} mt={2}>
                                        <Box mr={2} mb={2}>
                                            <TextField fullWidth label="Server" value={server} onChange={e => setServer(e.target.value)} />
                                        </Box>
                                    </Grid>
                                    <Grid item={true} xs={4} mt={2}>
                                        <TextField fullWidth label="Port" value={port} onChange={e => setPort(e.target.value)} />
                                    </Grid>
                                    <Grid item={true} xs={12}>
                                        <TextField fullWidth label="Username" value={username} onChange={e => setUsername(e.target.value)} />
                                    </Grid>
                                    {additionalOptions}
                                    <Grid item={true} xs={6} container justifyContent="flex-start">
                                        <Box mt={2}>
                                            <LoadingButton loading={connecting} variant="contained" onClick={e => saveServer()}>{t('Save')}</LoadingButton >
                                        </Box>
                                    </Grid>
                                    <Grid item={true} xs={6} container justifyContent="flex-end">
                                        <Box mt={2}>
                                            <Tooltip title="More Options">
                                                <IconButton color="primary" onClick={e => setShowAdditionalOptions(!showAdditionalOptions)} >
                                                    <UnfoldMoreIcon />
                                                </IconButton>
                                            </Tooltip>
                                        </Box>

                                        <Box mt={2}>
                                            <LoadingButton loading={connecting} variant="outlined" onClick={e => connect()} endIcon={<SendIcon />}>{t('Connect')}</LoadingButton >
                                        </Box>
                                    </Grid>
                                </Grid>
                            </Container>
                        </AccordionDetails>
                    </Accordion>
                </Box>
                <Box>
                    <IconButton onClick={(event) => setLanguageMenuAnchorEl(event.currentTarget)}>
                        <LanguageIcon />
                    </IconButton>
                    <Menu
                        id="basic-menu"
                        anchorEl={languageMenuAnchorEl}
                        open={languageMenuOpen}
                        onClose={() => setLanguageMenuAnchorEl(null)}
                        MenuListProps={{
                            'aria-labelledby': 'basic-button',
                        }}
                    >
                        {i18n.languages.map((e) => {
                            return (<MenuItem key={e} onClick={() => {
                                i18n.changeLanguage(e);
                                setLanguageMenuAnchorEl(null)
                            }}>
                                {t(e)}
                            </MenuItem>)

                        })}
                    </Menu>
                </Box>
            </Box>
            <Box sx={{ flexGrow: 1, backgroundImage: 'url(' + window.location.origin + '/login_bg.png)', backgroundSize: 'cover', margin: 2, borderRadius: 7 }}>
            </Box>
        </Box>
    )
}

export default Login
