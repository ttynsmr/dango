use super::super::note::Note;
use super::super::notes::Notes;
use std::collections::HashSet;
use std::env;

pub fn trello_match(url: &str) -> bool {
    url.starts_with("https://trello.com/")
}

pub fn trello_fetch_note(note: &Note) -> Result<Note, Box<dyn std::error::Error>> {
    let trello_token = env::var("TRELLO_TOKEN").expect("TRELLO_TOKEN is not found");
    let trello_api_key = env::var("TRELLO_API_KEY").expect("TRELLO_API_KEY is not found");
    let card_id = note.url.split("/").collect::<Vec<&str>>()[4];
    // println!("request: original url:{} id:{} url:https://api.trello.com/1/cards/{}?key=KEY&token=TOKEN", note.url, card_id, card_id);
    let mut card_json: String = String::default();
    match reqwest::blocking::get(format!(
        "https://api.trello.com/1/cards/{}?key={}&token={}&checklists=all",
        card_id, trello_api_key, trello_token
    )) {
        Ok(card_json_result) => {
            card_json = card_json_result.text()?;
        }
        Err(e) => println!("{}", e),
    }

    // println!("{}", card_json);
    let card = json::parse(&card_json).unwrap();
    let mut checklist_names: Vec<String> = Vec::new();
    for checklist in card["checklists"].members() {
        for check in checklist["checkItems"].members() {
            // println!("{}", check.to_string());
            // println!("checklist item {}", check["name"]);
            checklist_names.push(check["name"].as_str().unwrap().to_string());
        }
    }

    let mut note = Note {
        url: note.url.clone(),
        title: card["name"].as_str().unwrap().to_string(),
        sources: vec![card["desc"].as_str().unwrap().to_string()],
        links: note.links.clone(),
        referenced: note.referenced.clone(),
        need_fetch: false,
    };
    note.sources.extend(checklist_names);

    // note.collect_links();
    // note.dump();

    Ok(note)
}

pub fn trello_call(notes: &mut Notes) -> Result<(), Box<dyn std::error::Error>> {
    let trello_token = env::var("TRELLO_TOKEN").expect("TRELLO_TOKEN is not found");
    let trello_api_key = env::var("TRELLO_API_KEY").expect("TRELLO_API_KEY is not found");
    let card_json = reqwest::blocking::get(format!(
        "https://api.trello.com/1/cards/{}?key={}&token={}",
        "LgGUaRMO", trello_api_key, trello_token
    ))?
    .text()?;

    // println!("{}", card_json);

    let card = json::parse(&card_json).unwrap();
    let mut checklist: Vec<String> = Vec::new();
    for checklist_id in card["idChecklists"].members() {
        let checklist_json = reqwest::blocking::get(format!(
            "https://api.trello.com/1/checklists/{}?key={}&token={}",
            checklist_id, trello_api_key, trello_token
        ))?
        .text()?;
        let checklist_object = json::parse(&checklist_json).unwrap();
        for check in checklist_object["checkItems"].members() {
            // println!("{}", check.to_string());
            // println!("checklist item {}", check["name"]);
            checklist.push(check["name"].as_str().unwrap().to_string());
        }
    }

    let mut note = Note {
        url: card["shortUrl"].as_str().unwrap().to_string(),
        title: card["name"].as_str().unwrap().to_string(),
        sources: vec![card["desc"].as_str().unwrap().to_string()],
        links: HashSet::new(),
        referenced: HashSet::new(),
        need_fetch: false,
    };
    note.sources.extend(checklist);
    note.collect_links();

    notes.append(note);

    Ok(())
}
