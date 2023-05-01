import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'
import { ThemeProvider, createTheme } from '@mui/material';
import { store } from './store/store'
import { Provider } from 'react-redux'
import dayjs from 'dayjs';
import isToday from 'dayjs/plugin/isToday';
import isYesterday from 'dayjs/plugin/isYesterday';

const darkTheme = createTheme({
  palette: {
    mode: 'dark',
  },
});

dayjs.extend(isToday);
dayjs.extend(isYesterday);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider theme={darkTheme}>
    <Provider store={store}>
    <App />
    </Provider>
    </ThemeProvider>
  </React.StrictMode>,
)
