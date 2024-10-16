import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App'
import '../global.css'

import { AuthContextProvider } from './context/AuthContext'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <AuthContextProvider>
      <App />
    </AuthContextProvider>
  </StrictMode>,
)
