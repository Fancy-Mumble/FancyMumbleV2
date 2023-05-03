import './App.css'
import { RouterProvider, useNavigate } from 'react-router-dom';
import { router } from './routes/router';
import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { handleBackendMessage } from './helper/BackendMessageHandler';
import { useDispatch } from 'react-redux';
import Titlebar from './components/Titlebar';

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
      <RouterProvider router={router} />
    </div>
  )
}

export default App
