import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'
import { LineraProvider } from './context/LineraContext'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <LineraProvider>
      <App />
    </LineraProvider>
  </React.StrictMode>,
)
