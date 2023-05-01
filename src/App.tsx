import './App.css'
import { RouterProvider, createBrowserRouter, useLocation } from 'react-router-dom';
import Login from './Login';
import Chat from './Chat';
import { useEffect } from 'react';

let router = createBrowserRouter([
  {
    path: "/",
    Component: Login,
  },
  {
    path: "/chat",
    Component: Chat,
  },
]);

function App() {

  return (
    <div className="App">
      <RouterProvider router={router} />
    </div>
  )
}

export default App
function dispatch(arg0: any) {
  throw new Error('Function not implemented.');
}

