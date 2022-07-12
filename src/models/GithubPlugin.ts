import { Note } from "./Note"
import { NotePlugin } from "./Plugin"
import * as linkify from 'linkifyjs';
import { fetch } from '@tauri-apps/api/http';

interface GithubIssue {
  title: string
  body: string
  html_url: string
  comments_url: string
}

interface IssueComments {
  body: string
}

export class GithubPlugin implements NotePlugin {
  constructor(token: string) {
    this.name = "GitHub"
    this.token = token
  }

  name: string
  token: string

  isPullRequest(url: string): boolean {
    let regexp = new RegExp("https://github\.com/[a-zA-Z0-9][a-zA-Z0-9-\._]+[a-zA-Z0-9]/[a-zA-Z0-9][a-zA-Z0-9-\._]+[a-zA-Z0-9]/pull/[0-9]+")
    if (regexp.test(url)) {
      return true
    }
    return false
  }

  isIssue(url: string): boolean {
    let regexp = new RegExp("https://github\.com/[a-zA-Z0-9][a-zA-Z0-9-\._]+[a-zA-Z0-9]/[a-zA-Z0-9][a-zA-Z0-9-\._]+[a-zA-Z0-9]/issues/[0-9]+")
    if (regexp.test(url)) {
      return true
    }
    return false
  }

  getUrlType(url: string): string {
    if (this.isIssue(url)) {
      return "issues";
    }

    if (this.isPullRequest(url)) {
      return "pulls";
    }

    return "pulls";
  }

  isMatch(url: string): boolean {
    return this.isPullRequest(url) || this.isIssue(url)
  }

  getNormalizedUrl(url: string): string {
    return url
  }

  async fetchNote(url: string): Promise<Note> {
    //https://github.com/ttynsmr/dango/issues/5
    let splitted = url.split('/')
    let owner = splitted[3]
    let repo = splitted[4]
    let issue_number = Number.parseInt(splitted[6])

    let response = await fetch(`https://api.github.com/repos/${owner}/${repo}/${this.getUrlType(url)}/${issue_number}`, {
      method: "GET",
      headers: {
        Accept: "application/vnd.github+json",
        Authorization: `token ${this.token}`
      }
    });

    let issue = response.data as GithubIssue

    let note = new Note()
    note.plugin = this.name
    note.title = issue.title;
    note.url = issue.html_url;
    if (issue.body) {
      note.sources.push(issue.body)
    }
    note.needFetch = false

    let comments = await this.fetchComments(issue.comments_url)
    note.sources.concat(comments);

    note.sources.map(value => {
      return Array.from(linkify.find(value, "url")).map(link => { return link.href })
    }).flat().forEach(value => { note.links.add(value) })

    return note
  }

  async fetchComments(url: string): Promise<string[]> {
    let response = await fetch(url, {
      method: "GET",
      headers: {
        Accept: "application/vnd.github+json",
        Authorization: `token ${this.token}`
      }
    });

    let comments = response.data as IssueComments[]

    return comments.map(value => value.body)
  }
}
