import { useEffect, useState } from 'react'
import '../App.css'
import { Alert, Box, Container, Grid, TextField, Typography } from '@mui/material'
import LoadingButton from '@mui/lab/LoadingButton';
import { invoke } from '@tauri-apps/api/tauri'
import { useLocation, useNavigate } from 'react-router-dom';
import { useDispatch, useSelector } from 'react-redux';
import { RootState } from '../store/store';

function Login() {

    const userInfo = useSelector((state: RootState) => state.reducer.userInfo);

    const [server, setServer] = useState("magical.rocks");
    const [port, setPort] = useState("64738");
    const [username, setUsername] = useState("Endor");
    const [connecting, setConnecting] = useState(false);
    const [errorInfo, setErrorInfo] = useState({ show: false, text: "" });

    const location = useLocation();
    const dispatch = useDispatch();
    const navigate = useNavigate();

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

    //TODO: We shouldn't just have a binary connected state,
    //TODO: but a state that can have multiple values, like "connecting",
    //TODO: "connected", "disconnected", "error"
    if (userInfo.connected) {
        console.log("connected");
        navigate("/chat");
    }

    function connect() {
        setConnecting(true);
        setErrorInfo({ show: false, text: "" });

        invoke('connect_to_server', { serverHost: server, serverPort: parseInt(port), username: username }).then(e => {
            setConnecting(false);
        }).catch(e => {
            setErrorInfo({ show: true, text: e });
            setConnecting(false);
        });
    }

    let errorBox = errorInfo.show ? (<Box mb={3}><Alert severity="error">{errorInfo.text}</Alert></Box>) : (<div></div>);

    return (
        <Container className='Login'>
            <Typography align='center' variant='h2' gutterBottom>Fancy Mumble</Typography >
            {errorBox}
            <Container>
                <Grid container spacing={1}>
                    <Grid xs={8}>
                        <Box mr={2} mb={2}>
                            <TextField fullWidth label="Server" value={server} onChange={e => setServer(e.target.value)} />
                        </Box>
                    </Grid>
                    <Grid xs={4}>
                        <TextField fullWidth label="Port" value={port} onChange={e => setPort(e.target.value)} />
                    </Grid>
                    <Grid xs={12}>
                        <TextField fullWidth label="Username" value={username} onChange={e => setUsername(e.target.value)} />
                    </Grid>
                    <Grid xs={12} container justifyContent="flex-end">
                        <Box mt={2}>
                            <LoadingButton loading={connecting} variant="outlined" onClick={e => connect()}>Connect</LoadingButton >
                        </Box>
                    </Grid>
                </Grid>
            </Container>
        </Container>
    )
}

export default Login
