import { appWindow } from "@tauri-apps/api/window";
import CloseIcon from '@mui/icons-material/Close';
import MaximizeIcon from '@mui/icons-material/Maximize';
import MinimizeIcon from '@mui/icons-material/Minimize';
import FilterNoneIcon from '@mui/icons-material/FilterNone';
import { Box, IconButton, Paper } from "@mui/material";
import './styles/Titlebar.css';

function Titlebar() {
    return (
        <Paper data-tauri-drag-region sx={{ width: '100%', display: 'flex', justifyContent: 'end', zIndex: 9999}}>
            <IconButton size="small" onClick={(e) => appWindow.minimize()} className="titlebar-button" >
                <MinimizeIcon sx={{ fontSize: 18 }}/>
            </IconButton >
            <IconButton size="small" onClick={(e) => appWindow.toggleMaximize()} className="titlebar-button">
                <FilterNoneIcon sx={{ fontSize: 18 }}/>
            </IconButton >
            <IconButton size="small" onClick={(e) => appWindow.close()} className="titlebar-button" color="error">
                <CloseIcon sx={{ fontSize: 18 }}/>
            </IconButton >
        </Paper>
    )
}

export default Titlebar;