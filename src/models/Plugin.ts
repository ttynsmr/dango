import { Note } from "./Note";

export interface NotePlugin {
  name: string
  isMatch(url: string): boolean
  getNormalizedUrl(url: string): string
  fetchNote(url: string): Promise<Note>
}
