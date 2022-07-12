import { Note } from "./Note"
import { NotePlugin } from "./Plugin"
import * as linkify from 'linkifyjs';
import { fetch } from '@tauri-apps/api/http';

interface TrelloCard {
  name: string
  desc: string
  shortUrl: string
}

export class TrelloPlugin implements NotePlugin {
  constructor(apiKey: string, apiToken: string) {
    this.name = "Trello"
    this.apiKey = apiKey
    this.apiToken = apiToken
  }

  name: string
  apiKey: string
  apiToken: string

  isMatch(url: string): boolean {
    return url.startsWith("https://trello.com/c/")
  }

  getNormalizedUrl(url: string): string {
    let splitted = url.split('/')
    let shortLink = splitted[4]
    return `https://trello.com/c/${shortLink}`
  }

  async fetchNote(url: string): Promise<Note> {
    //https://trello.com/c/LgGUaRMO
    let splitted = url.split('/')
    let shortLink = splitted[4]
    let response = await fetch(`https://api.trello.com/1/cards/${shortLink}?key=${this.apiKey}&token=${this.apiToken}&checklists=all`, {
      method: "GET",
      headers: {
        Accept: "application/json",
      }
    });

    let card: TrelloCard = response.data as TrelloCard

    let note = new Note()
    note.plugin = this.name
    note.title = card.name;
    note.url = card.shortUrl;
    note.sources.push(card.desc)
    note.needFetch = false

    note.sources.map(value => {
      return Array.from(linkify.find(value, "url")).map(link => { return link.href })
    }).flat().forEach(value => { note.links.add(value) })

    return note
  }
}
