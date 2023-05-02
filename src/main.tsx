import React, { Suspense } from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'
import { ThemeProvider, createTheme } from '@mui/material';
import { store } from './store/store'
import { Provider } from 'react-redux'
import dayjs from 'dayjs';
import isToday from 'dayjs/plugin/isToday';
import isYesterday from 'dayjs/plugin/isYesterday';
import Loader from './components/Loader';

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
    <Suspense fallback={<Loader />}>
    <Provider store={store}>
    <App />
    </Provider>
    </Suspense>
    </ThemeProvider>
  </React.StrictMode>,
)
