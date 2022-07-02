import { useState } from "react"
import { Note } from "../notes/Note"
import { Tooltip, Button } from "@material-tailwind/react";

type Props = {
  note: Note
}

const getPluginColor = (name: string): string => {
  return 'plugin-' + name.toLowerCase()
}

const NoteCard: React.FC<Props> = ({ note }) => (
  <>
    <div className="bg-purple-50 flex flex-col rounded min-w-[150px] m-0 p-2 justify-self-auto gap-y-1.5">
      <div className="flex gap-2 p-0">
        <div className={`text-xs truncate min-w-[70px] bg-${getPluginColor(note.plugin || "purple-400")} text-purple-50 rounded px-2 py-1`}>
          {note.plugin !== 'Unknown' && <p className={"pr-1 fa-brands fa-" + note.plugin?.toLowerCase()} />}{note.plugin}
        </div>
        <Tooltip content={note.title} className="rounded bg-black text-purple-50">
          <Button variant="text" size="lg" color="deep-purple" ripple={false} className="truncate font-bold font-sans text-purple-600 text-base p-0 text-left">
            {note.title}
          </Button>
        </Tooltip>
        {/* <button className="truncate font-bold font-sans text-purple-600 text-base p-0 text-left">
          {note.title}
        </button> */}
      </div>
      <a className="truncate tracking-tight font-sans text-purple-600 text-xs p-0 hover:underline" href={note.url} target="_blank">
        <p className="pr-1 fa-solid fa-arrow-up-right-from-square" />
        {note.url}
      </a>
      <div className="bg-plugin-github" hidden>1</div>
      <div className="bg-plugin-slack" hidden>2</div>
      <div className="bg-plugin-trello hidden">3</div>
      <div className="bg-plugin-unknown hidden">4</div>
    </div>
  </>
);

export default NoteCard
