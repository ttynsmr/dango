import { useState } from "react"
import { Note } from "../notes/Note"
import { Tooltip, Chip, Card, CardBody } from "@material-tailwind/react";
import ReactMarkdown from 'react-markdown'
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { IconName } from '@fortawesome/fontawesome-svg-core';

type Props = {
  note: Note,
  onClickHandler: (url: string) => void
}

const getPluginColor = (name: string): string => {
  return 'plugin-' + name.toLowerCase()
}

type IconProps = {
  plugin: string | undefined,
}
const PluginIcon: React.FC<IconProps> = ({ plugin }) => {
  const getPluginName = (plugin: string | undefined) => {
    if (plugin !== undefined && plugin !== 'Unknown') {
      console.log(plugin.toLowerCase())
      return <FontAwesomeIcon className="px-0.5 text-xl" icon={["fab", plugin.toLowerCase() as IconName]} />
    }
    else {
      return <FontAwesomeIcon className="px-0.5 text-xl" icon={["fas", "circle-question"]} />
    }
  }

  return (
    getPluginName(plugin)
  )
}

const NoteCard: React.FC<Props> = ({ note, onClickHandler }) => {
  const [showSource, setShowSource] = useState(false)
  const [handleClick, setHandleClick] = useState()

  return (
    <>
      <Card className="shadow-md bg-purple-50 flex flex-col rounded min-w-[150px] m-0 p-2 justify-self-auto gap-y-1.5 border-4 border-purple-600 hover:border-purple-50">
        <div className="flex gap-2 p-0">
          <Chip icon={<PluginIcon plugin={note.plugin} />} className={`bg-${getPluginColor(note.plugin || "purple-400")}`} value={note.plugin ? note.plugin : 'Unknown'} />
          <Tooltip content={note.title} className="rounded bg-black text-purple-50">
            <a
              className="truncate self-center text-xl font-bold font-sans text-purple-600 text-base p-0 text-left"
              href="#"
              onClick={() => { if (note.url) onClickHandler(note.url) }}
            >
              {note.title}
            </a>
          </Tooltip>
        </div>
        <a className="truncate tracking-tight font-sans text-purple-600 text-xs p-0 hover:underline" href={note.url} target="_blank">
          <FontAwesomeIcon className="pr-1" icon={["fas", "arrow-up-right-from-square"]} />
          {note.url}
        </a>
        <CardBody className="flex flex-col p-0.5" >
          <button className="truncate text-left text-[8px] shadow-inner rounded bg-purple-100 break-words p-1" onClick={() => {
            setShowSource(!showSource)
          }} hidden={note.sources?.join("\n").length === 0}>
            <FontAwesomeIcon icon={["fas", (showSource ? "square-minus" : "square-plus") as IconName]} />
            <code hidden={showSource}>{note.sources?.at(0)?.split('\n').at(0)}</code>
            <div hidden={!showSource}><ReactMarkdown>{note.sources ? note.sources.join("\n") : ""}</ReactMarkdown></div>
          </button>
          <div className="bg-plugin-github" hidden>1</div>
          <div className="bg-plugin-slack" hidden>2</div>
          <div className="bg-plugin-trello hidden">3</div>
          <div className="bg-plugin-unknown hidden">4</div>
        </CardBody>
      </Card >
    </>
  )
};

export default NoteCard
