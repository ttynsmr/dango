import { useState } from "react"
import { Note } from "../notes/note"

type Props = {
  note: Note
}

const NoteCard: React.FC<Props> = ({ note }) => (
  <>
    <div className="bg-purple-50 flex flex-col rounded m-2 w-[300px]">
      <button className="truncate font-bold font-sans text-purple-600 text-base p-0">
        {note.title}
      </button>
      <a className="truncate tracking-tight font-sans text-purple-600 text-xs p-1" href={note.url} target="_blank">{note.url}</a>
    </div>
  </>
);

export default NoteCard
