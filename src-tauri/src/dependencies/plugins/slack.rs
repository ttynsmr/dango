use crate::dependencies::note;

use super::super::note::Note;
use super::super::notes::Notes;
use regex::Regex;
use std::collections::HashSet;
use std::env;

pub fn slack_match(url: &str) -> bool {
    Regex::new(r##"https://[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.slack\.com/"##)
        .unwrap()
        .is_match(url)
}

pub fn slack_fetch_note(note: &Note) -> Result<Note, Box<dyn std::error::Error>> {
    // let client = slack_api::requests::default_client().unwrap();
    // let params = Default::default();
    // let response = slack_api::channels::list(&client, &token, &params);

    let mut new_note = Note::default();
    new_note.url = note.url.clone();
    new_note.need_fetch = false;
    Ok(new_note)
}
