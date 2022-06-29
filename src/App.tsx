import { useState } from 'react'
import logo from './logo.svg'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { Notes, Note } from './notes/note'

const NotesList = ({ notes }: Notes) => (
  <ol>
    {Array.from(notes.values()).map((value, index, notes) => (<li key={index} > <a href={value.url} target="_blank">{value.title !== "" ? value.title : value.url}</a></li>))}
  </ol >
);

function App() {
  const [url, setUrl] = useState("")
  const [disable, setDisable] = useState(false)
  const [notes, setNotes] = useState(new Notes)

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Hello Vite + React!</p>
        <div>
          <p><label>retrieve url: {url}</label></p>
          <input
            type="text"
            value={url}
            onChange={event => setUrl(event.target.value)}
          />
        </div>
        <p>
          <button type="button" disabled={disable} onClick={() => {
            setDisable(true)
            invoke<string>('fetch_note', { url: url })
              // `invoke` returns a Promise
              .then((response) => {
                let response_as_json_object = JSON.parse(JSON.stringify(response));

                let notes = new Notes;
                let notes_as_map = response_as_json_object.notes as Map<string, Note>;
                for (let note_key in notes_as_map) {
                  notes.notes.set(note_key, new Note(response_as_json_object.notes[note_key]));
                }

                setDisable(false)
                setNotes(notes)
                console.log(notes)
              })
          }}>fetch note</button>
        </p>
        <NotesList notes={notes.notes} />
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
