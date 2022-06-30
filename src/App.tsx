import { useState } from 'react'
import logo from './logo.svg'
import './App.css'
import { invoke } from '@tauri-apps/api'
import { Notes, Note } from './notes/note'
import NoteCard from './interface/NoteCard'

const NotesList = ({ notes }: Notes) => (
  <>
    <div className='w-full grid grid-flow-col auto-cols-max gap-0'>
      {Array.from(notes.values()).map((value, index, notes) => (<NoteCard note={value} />))}
    </div>
  </>
);

function App() {
  const [url, setUrl] = useState("")
  const [disable, setDisable] = useState(false)
  const [notes, setNotes] = useState(new Notes)

  return (
    <div className="App">
      <header className="App-header bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500">
        <p className="text-purple-50 bg-purple-600 rounded m-5 p-3">dango</p>
        <div className='flex grid grid-flow-row auto-rows-max'>
          <p><label className='text-lg'>Input URL</label></p>
          <div className='flex'>
            <input
              className='form-input shadow appearance-none border rounded min-w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline'
              type="url"
              value={url}
              onChange={event => setUrl(event.target.value)}
            />
            <button className='bg-blue-500 hover:bg-blue-700 text-purple-50 font-bold py-2 px-4 rounded' type="button" disabled={disable} onClick={() => {
              setDisable(true)
              fetch(url)
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
            }}>Fetch</button>
          </div>
        </div>
        <NotesList notes={notes.notes} />
      </header>
    </div >
  )
}

export default App
