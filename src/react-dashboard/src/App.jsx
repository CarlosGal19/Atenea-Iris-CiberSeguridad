import { useContext, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { AuthContext } from './context/AuthContext'
import { createActor } from './declarations/backend'

function App() {
  const { isAuth, login, logout, identity } = useContext(AuthContext);
  const [click, setClick] = useState(false);

  let canisterId = import.meta.env.VITE_CANISTER_ID_BACKEND;
  console.log(identity);
  let backend = createActor(canisterId, {
    agentOptions: {
      host: "http://localhost:4943",
      identity
    }
  });

  async function handleClick() {
    try {
      const response = await backend.greet("Carlos");
      console.log(response);
      setClick(prev => !prev);
    } catch (e) {
      console.error(e);
    }
  }

  return (
    <>
      <header className="App-header">
        <img src={reactLogo} className="App-logo" alt="logo" />
        <img src={viteLogo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.jsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
        {isAuth ? (
          <button onClick={logout}>Logout</button>
        ) : (
          <button onClick={login}>Login</button>
        )}
        {click && <p>Functions fetched!</p>}
        <button onClick={handleClick}>
          Fetch functions
        </button>
      </header>
    </>
  )
}

export default App
