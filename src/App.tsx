import { useState } from 'react'
import logo from './logo.svg'
import './App.css'
import { invoke } from '@tauri-apps/api'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Hello Vite + React!</p>
        <p>
          <button type="button" onClick={() => setCount((count) => count + 1)}>
            count is: {count}
          </button>
          <button type="button" onClick={() => {
            // now we can call our Command!
            // Right-click the application background and open the developer tools.
            // You will see "Hello, World!" printed in the console!
            invoke('greet', { name: 'World' })
              // `invoke` returns a Promise
              .then((response) => console.log(response))
          }}>hello</button>
          <button type="button" onClick={() => {
            // now we can call our Command!
            // Right-click the application background and open the developer tools.
            // You will see "Hello, World!" printed in the console!
            invoke('fetch_note', { query: 'repo:ttynsmr/potato' })
              // `invoke` returns a Promise
              .then((response) => console.log(response))
          }}>fetch note</button>
        </p>
        <p>
          Edit <code>App.tsx</code> and save to test HMR updates.
        </p>
        <p>
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
          </a>
          {' | '}
          <a
            className="App-link"
            href="https://vitejs.dev/guide/features.html"
            target="_blank"
            rel="noopener noreferrer"
          >
            Vite Docs
          </a>
        </p>
      </header>
    </div >
  )
}

export default App
