import './App.css'
import { RouterProvider, useNavigate } from 'react-router-dom';
import { router } from './routes/router';
import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { handleBackendMessage } from './helper/BackendMessageHandler';
import { useDispatch } from 'react-redux';
import Titlebar from './components/Titlebar';
import { Box } from '@mui/material';

function App() {
  const dispatch = useDispatch();

  useEffect(() => {

    //listen to a event
    const unlisten = listen("backend_update", e => handleBackendMessage(e, dispatch));

    return () => {
      unlisten.then(f => f());
    }
  });

  return (
    <div className="App">
      <Titlebar />
      <Box sx={{ flex: 1, overflow: 'auto' }}>
        <RouterProvider router={router} />
      </Box>
    </div>
  )
}

export default App
