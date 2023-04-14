import './App.css'
import { RouterProvider, createBrowserRouter } from 'react-router-dom';
import Login from './Login';
import Chat from './Chat';

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
