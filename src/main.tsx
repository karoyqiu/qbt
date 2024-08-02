import { PrimeReactProvider } from 'primereact/api';
import React from 'react';
import ReactDOM from 'react-dom/client';

import 'primeicons/primeicons.css';

import App from './App';
import './app.css';

if (!import.meta.env.DEV) {
  document.addEventListener('contextmenu', (event) => {
    if (!event.target || !('tagName' in event.target) || event.target.tagName !== 'INPUT') {
      event.preventDefault();
    }
  });
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <PrimeReactProvider value={{ ripple: true, nonce: 'Eyb2JqlROQDm6V2LGWjuj' }}>
      <App />
    </PrimeReactProvider>
  </React.StrictMode>,
);
