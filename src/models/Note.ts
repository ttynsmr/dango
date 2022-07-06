class Note {
  constructor(init?: Partial<Note>) {
    this.url = "";
    this.plugin = "";
    this.title = "";
    this.sources = [];
    this.links = [];
    this.referenced = [];
    this.need_fetch = true;
    Object.assign(this, init)
  }
  public url: string;
  public plugin: string;
  public title: string;
  public sources: string[];
  public links: string[];
  public referenced: string[];
  public need_fetch: boolean;
};

class Notes {
  constructor() {
    this.notes = new Map<string, Note>()
  }
  public notes: Map<string, Note>;
};

export { Note, Notes }
