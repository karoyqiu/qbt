import { PrimeReactProvider } from 'primereact/api';
import React from 'react';
import ReactDOM from 'react-dom/client';

import 'primeicons/primeicons.css';

import App from './App';
import './app.css';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <PrimeReactProvider value={{ ripple: true, nonce: 'Eyb2JqlROQDm6V2LGWjuj' }}>
      <App />
    </PrimeReactProvider>
  </React.StrictMode>,
);
