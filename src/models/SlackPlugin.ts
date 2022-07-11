import { Note } from "./Note"
import { NotePlugin } from "./Plugin"
import * as linkify from 'linkifyjs';
import { fetch } from '@tauri-apps/api/http';

interface Message {
  text: string
}

interface Replies {
  messages: Message[]
  ok: boolean
}

export class SlackPlugin implements NotePlugin {
  constructor(token: string) {
    this.name = "Slack"
    this.token = token
  }

  name: string
  token: string

  isMatch(url: string): boolean {
    let regexp = new RegExp("https://[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.slack\.com/")
    return regexp.test(url)
  }

  getNormalizedUrl(url: string): string {
    return url
  }

  async fetchNote(url: string): Promise<Note> {
    let splitted = url.split('/')
    let slack_channel = splitted[4]
    let slack_thread_ts = splitted[5]
    let thread_ts_integer = slack_thread_ts.substring(1, slack_thread_ts.length - 6)
    let thread_ts_decimal = slack_thread_ts.substring(slack_thread_ts.length - 6)

    const response = await fetch(`https://slack.com/api/conversations.replies?channel=${slack_channel}&ts=${thread_ts_integer}.${thread_ts_decimal}&pretty=1`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${this.token}`,
      },
    });

    let replies: Replies = response.data as Replies

    let note = new Note()
    note.plugin = this.name
    if (replies.ok) {
      note.title = "[TS]" + replies.messages[0]?.text;
      note.url = this.getNormalizedUrl(url);
      note.sources.concat(replies.messages.map(message => message.text))
      note.needFetch = false
    }
    else {
      note.title = "[TS]" + "🔒";
      note.url = this.getNormalizedUrl(url);
      note.needFetch = false
    }

    note.sources.map(value => {
      return Array.from(linkify.find(value, "url")).map(link => { return link.href })
    }).flat().forEach(value => { note.links.add(value) })

    return note
  }
}
