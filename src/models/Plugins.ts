import { Note } from "./Note";
import { NotePlugin } from "./Plugin";

export class Plugins {
  constructor() {
    this.plugins = []
  }

  plugins: NotePlugin[]

  public addPlugin(plugin: NotePlugin) {
    this.plugins.push(plugin)
  }

  private getPlugin(url: string): NotePlugin | undefined {
    for (var plugin of this.plugins) {
      if (plugin.isMatch(url)) { return plugin; }
    }
  }


  public getPluginName(url: string) {
    let plugin = this.getPlugin(url);
    if (plugin) {
      return plugin.name
    }
    else {
      return "Unknown"
    }
  }

  public isMatch(url: string): boolean {
    let plugin = this.getPlugin(url);
    if (plugin) {
      return true
    }
    else {
      return false
    }
  }

  public getNormalizedUrl(url: string): string {
    let plugin = this.getPlugin(url);
    if (plugin) {
      return plugin.getNormalizedUrl(url)
    }
    else {
      return url
    }
  }

  public fetchNote(url: string): Promise<Note> | undefined {
    let plugin = this.getPlugin(url);
    if (plugin) {
      return plugin.fetchNote(url)
    }
    else {
      return undefined
    }
  }
}