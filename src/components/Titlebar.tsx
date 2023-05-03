import { appWindow } from "@tauri-apps/api/window";
import CloseIcon from '@mui/icons-material/Close';
import MaximizeIcon from '@mui/icons-material/Maximize';
import MinimizeIcon from '@mui/icons-material/Minimize';
import FilterNoneIcon from '@mui/icons-material/FilterNone';
import { Box, IconButton, Paper } from "@mui/material";
import './styles/Titlebar.css';

function Titlebar() {
    return (
        <Paper data-tauri-drag-region sx={{ width: '100%', display: 'flex', justifyContent: 'end'}}>
            <IconButton size="small" onClick={(e) => appWindow.minimize()} className="titlebar-button" >
                <MinimizeIcon/>
            </IconButton >
            <IconButton size="small" onClick={(e) => appWindow.toggleMaximize()} className="titlebar-button">
                <FilterNoneIcon />
            </IconButton >
            <IconButton size="small" onClick={(e) => appWindow.close()} className="titlebar-button" color="error">
                <CloseIcon />
            </IconButton >
        </Paper>
    )
}

export default Titlebar;