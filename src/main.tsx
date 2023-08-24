import React from 'react';
import ReactDOM from 'react-dom/client';

import App from './App';
import { loadTheme } from './helpers';

import './css/index.css';

loadTheme(null);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
