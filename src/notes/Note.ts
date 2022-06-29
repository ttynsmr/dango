class Note {
    constructor(init?: Partial<Note>) {
        Object.assign(this, init)
    }
    public url: string | undefined;
    public title: string | undefined;
    public sources: string[] | undefined;
    public links: string[] | undefined;
    public referenced: string[] | undefined;
    public need_fetch: boolean | undefined;
};

class Notes {
    constructor() {
        this.notes = new Map<string, Note>()
    }
    public notes: Map<string, Note>;
};

export { Note, Notes }
