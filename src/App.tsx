import { useState } from 'react'
import { invoke } from '@tauri-apps/api'
import { Notes, Note } from './models/Note'
import NotesList from './interface/NoteContainer'
import { Input, Button } from "@material-tailwind/react";
import { fas } from '@fortawesome/free-solid-svg-icons'
import { fab } from '@fortawesome/free-brands-svg-icons'
import { library } from '@fortawesome/fontawesome-svg-core'
library.add(fab, fas)
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import ConfigRecordGroup from './interface/ConfigRecordGroup';
import Config from './models/Config';

const fetch = async (url: string): Promise<Notes> => {
  let notes = new Notes;
  // console.log(url)
  await invoke<string>('fetch_note', { url: url })
    // `invoke` returns a Promise
    .then((response) => {
      let response_as_json_object = JSON.parse(JSON.stringify(response));

      let notes_as_map = response_as_json_object.notes as Map<string, Note>;
      for (let note_key in notes_as_map) {
        notes.notes.set(note_key, new Note(response_as_json_object.notes[note_key]));
      }
    })
  // console.log(notes)
  return notes;
}

function Tokens() {
  const [githubConfigs, setGithubConfigs] = useState([new Config("github", "token", "Token", "")]);
  const [slackConfigs, setSlackConfigs] = useState([new Config("slack", "token", "Token", "")]);
  const [trelloConfigs, setTrelloConfigs] = useState([new Config("trello", "api-key", "API Key", ""), new Config("trello", "token", "Token", "")]);

  return (
    <>
      <ConfigRecordGroup groupName='GitHub' configs={githubConfigs} />
      <ConfigRecordGroup groupName='Slack' configs={slackConfigs} />
      <ConfigRecordGroup groupName='Trello' configs={trelloConfigs} />
    </>
  )
}

function App() {
  const [url, setUrl] = useState("")
  const [disable, setDisable] = useState(false)
  const [notes, setNotes] = useState(new Notes)

  return (
    <div className="flex min-w-full min-h-full justify-center content-center">
      <header className="w-full h-full min-h-screen bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500">
        {/* <header className="w-full h-full min-h-screen bg-gradient-to-r from-dango-pink via-dango-white to-dango-green"> */}
        <div className="shadow-xl text-purple-600 bg-purple-50 m-0 p-1">
          <div className='flex gap-2 m-2'>
            <Input
              className='bg-purple-50'
              label="URL"
              value={url}
              onChange={event => setUrl(event.target.value)}
            />
            <Button
              disabled={disable}
              onClick={() => {
                setDisable(true)
                setNotes(new Notes)
                fetch(url).then((notes) => {
                  setNotes(notes)
                  setDisable(false)
                })
              }}>Fetch</Button>
            <Button
              disabled={disable}
              onClick={() => {
                setDisable(true)
                setNotes(new Notes)
                fetch(url).then((notes) => {
                  setNotes(notes)
                  setDisable(false)
                })
              }}><FontAwesomeIcon icon={["fas", "bars"]} /></Button>
          </div>
        </div>
        {/* <Tokens /> */}
        <NotesList
          notes={notes}
          onClickHandler={(url) => {
            setDisable(true)
            setNotes(new Notes)
            fetch(url).then((notes) => {
              setNotes(notes)
              setDisable(false)
            })
            setDisable(false)
          }}
          onGetNoteInfo={url => {
            return notes.notes.get(url)
          }} />
      </header >
    </div >
  )
}

export default App
