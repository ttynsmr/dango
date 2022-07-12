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
import { Plugins } from './models/Plugins';
import { GithubPlugin } from './models/GithubPlugin';
import { TrelloPlugin } from './models/TrelloPlugin';
import { SlackPlugin } from './models/SlackPlugin';

const fetch = async (url: string): Promise<Notes> => {
  let plugins = new Plugins()

  let username = ""
  plugins.addPlugin(new GithubPlugin(await invoke("load_token", { username: username, service: "GITHUB_TOKEN" })))
  plugins.addPlugin(new TrelloPlugin(
    await invoke("load_token", { username: username, service: "TRELLO_API_KEY" }),
    await invoke("load_token", { username: username, service: "TRELLO_TOKEN" })))
  plugins.addPlugin(new SlackPlugin(await invoke("load_token", { username: username, service: "SLACK_BOT_TOKEN" })))

  let rootNote = await plugins.fetchNote(url)
  if (rootNote === undefined) {
    rootNote = new Note({ url: url })
  }

  let notes = new Notes;
  notes.notes.set(rootNote.url, rootNote)

  await notes.analyze(plugins)

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

  document.addEventListener('contextmenu', event => event.preventDefault());

  return (
    <div className="flex min-w-full min-h-full justify-center content-center">
      <header className="w-full h-full min-h-screen bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500">
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
              onClick={async () => {
                setDisable(true)
                setNotes(new Notes)
                let responseNotes = await fetch(url)
                responseNotes.notes = new Map([...responseNotes.notes.entries()])
                setNotes(responseNotes)
                setDisable(false)
              }}>Fetch</Button>
            <Button
              disabled={disable}
              onClick={async () => {
                setNotes(new Notes)
              }}><FontAwesomeIcon icon={["fas", "bars"]} /></Button>
          </div>
        </div>
        {/* <Tokens /> */}
        <NotesList
          notes={notes}
          onClickHandler={async (url) => {
            setDisable(true)
            setNotes(new Notes)
            let responseNotes = await fetch(url)
            responseNotes.notes = new Map([...responseNotes.notes.entries()])
            setNotes(responseNotes)
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
