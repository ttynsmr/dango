import { faCropSimple } from "@fortawesome/free-solid-svg-icons";
import { Plugins } from "./Plugins";

class Note {
  constructor(init?: Partial<Note>) {
    this.url = "";
    this.plugin = "Unknown";
    this.title = "";
    this.sources = [];
    this.links = new Set();
    this.referenced = new Set();
    this.needFetch = true;
    Object.assign(this, init)
  }
  public url: string;
  public plugin: string;
  public title: string;
  public sources: string[];
  public links: Set<string>;
  public referenced: Set<string>;
  public needFetch: boolean;
};

class Notes {
  constructor() {
    this.notes = new Map<string, Note>()
  }
  public notes: Map<string, Note>;

  public async analyze(plugins: Plugins): Promise<boolean> {
    let phase = 1
    let hasNeedFetch = false;
    do {
      console.log(`=============== phase ${phase++}`)
      this.notes.forEach(note => {
        note.links.forEach(link => {
          let referencedNote = this.notes.get(plugins.getNormalizedUrl(link));
          if (!referencedNote) {
            referencedNote = new Note()
            referencedNote.url = plugins.getNormalizedUrl(link)
            referencedNote.plugin = plugins.getPluginName(link)
            referencedNote.needFetch = plugins.isMatch(link)
          }

          referencedNote.referenced.add(note.url)
          this.notes.set(referencedNote.url, referencedNote)
        })
      })

      let needFetchNotes = Array.from(this.notes.values()).filter(note => note.needFetch)
      for (let note of needFetchNotes) {
        let n = await plugins.fetchNote(note.url)
        if (n !== undefined) {
          this.notes.set(note.url, n)
        }
      }

      // console.log(this.notes)
      hasNeedFetch = Array.from(this.notes.values()).some(value => value.needFetch)
      console.log("has need fetch", hasNeedFetch)

      if (phase > 50) return false
    } while (hasNeedFetch)
    return true
  }
};

export { Note, Notes }
