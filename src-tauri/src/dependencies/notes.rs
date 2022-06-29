use super::note::Note;
use crate::dependencies::plugins;
use std::collections::{HashMap, HashSet};

pub struct Notes {
    notes: HashMap<String, Note>,
}

impl Notes {
    pub fn new() -> Self {
        Notes {
            notes: HashMap::new(),
        }
    }

    pub fn append_url(&mut self, url: &str) {
        let mut note = Note::default();
        note.url = url.to_string();
        note.need_fetch = true;
        self.notes.insert(note.url.clone(), note);
    }

    pub fn append(&mut self, note: Note) {
        self.notes.insert(note.url.clone(), note);
    }

    pub fn referenced_from(&mut self, link: String, from: String) {
        self.notes
            .entry(link.clone())
            .or_insert(Note {
                url: link.clone(),
                title: String::from(""),
                sources: Vec::new(),
                links: HashSet::new(),
                referenced: HashSet::new(),
                need_fetch: true,
            })
            .referenced_from(from);
    }

    pub fn analyze(&mut self) -> bool {
        // collect links
        for note in self.notes.iter_mut() {
            note.1.collect_links();
        }

        // create reference list
        let mut pairs: Vec<(String, String)> = Vec::new();
        for note in self.notes.values() {
            let links = note.links.clone();
            for link in links.iter() {
                pairs.push((link.clone(), note.url.clone()));
            }
        }

        // apply reference
        for (to, from) in pairs {
            self.referenced_from(to, from);
        }

        // fetch dirty note
        let mut fetch_request_count = 0;
        let mut fetched_notes: Vec<Note> = Vec::new();
        let needs_fetch_notes = self
            .notes
            .values()
            .filter(|&n| n.need_fetch && n.title.is_empty());
        for note in needs_fetch_notes {
            println!("what is {}", note.url);
            if plugins::trello::trello_match(&note.url) {
                let card_id = note.url.split("/").collect::<Vec<&str>>()[4];
                println!("need fetch as trello {} card_id: {}", note.url, card_id);
                fetch_request_count += 1;
                match plugins::trello::trello_fetch_note(&note) {
                    Ok(note) => {
                        fetched_notes.push(note);
                    }
                    Err(e) => println!("{}", e),
                }
            } else if plugins::github::github_match(&note.url) {
                println!("need fetch as github {}", note.url);
                fetch_request_count += 1;
                match plugins::github::github_fetch_note(&note) {
                    Ok(note) => {
                        fetched_notes.push(note);
                    }
                    Err(e) => println!("{}", e),
                }
            } else if plugins::slack::slack_match(&note.url) {
                println!("need fetch as slack {}", note.url);
                match plugins::slack::slack_fetch_note(&note) {
                    Ok(note) => {
                        fetched_notes.push(note);
                    }
                    Err(e) => println!("{}", e),
                }
            } else {
                // println!("unknown url {}", note.url);
            }
        }

        println!(
            "fetched notes count {}/{}",
            fetched_notes.len(),
            fetch_request_count
        );

        let fetched = !fetched_notes.is_empty();
        for note in fetched_notes {
            // println!("fetched note {}:{}:{}", note.need_fetch, note.url, note.title);
            self.append(note);
        }

        fetched
    }

    pub fn to_string(&self) -> String {
        let notes = self
            .notes
            .values()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        format!("notes count {}\n{}", self.notes.len(), notes)
    }

    pub fn dump(&self) {
        for note in self.notes.values() {
            note.dump();
        }
    }
}
