import { Note, Notes } from "../models/Note";
import NoteCard from "./NoteCard";

type Props = {
  notes: Notes,
  onClickHandler: (url: string) => void
  onGetNoteInfo: (url: string) => Note | undefined
}

const NotesList: React.FC<Props> = ({ notes, onClickHandler, onGetNoteInfo }) => {
  return (
    <>
      <div className='w-full grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-1 p-4 content-start justify-self-auto'>
        {Array.from(notes.notes.values()).map((value, index, notes) => <NoteCard note={value} onClickHandler={onClickHandler} onGetNoteInfo={onGetNoteInfo} />)}
      </div>
    </>
  )
};

export default NotesList
