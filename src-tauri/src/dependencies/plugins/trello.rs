use super::super::note::Note;
use super::Plugin;
use std::env;

pub struct Trello {}

impl Plugin for Trello {
    fn is_match(&self, url: &str) -> bool {
        url.starts_with("https://trello.com/")
    }

    fn normalize_url(&self, url: &str) -> String {
        let card_id = url.split("/").collect::<Vec<&str>>()[4];
        format!("https://trello.com/c/{}", card_id)
    }

    fn fetch_note(&self, note: &Note) -> Result<Note, Box<dyn std::error::Error>> {
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
        let card = json::parse(&card_json).unwrap_or(json::JsonValue::Null);
        let mut checklist_names: Vec<String> = Vec::new();
        for checklist in card["checklists"].members() {
            for check in checklist["checkItems"].members() {
                // println!("{}", check.to_string());
                // println!("checklist item {}", check["name"]);
                checklist_names.push(check["name"].as_str().unwrap().to_string());
            }
        }

        let mut note = Note {
            normalized_url: self.normalize_url(note.url.as_ref()),
            url: note.url.clone(),
            plugin: self.name(),
            title: card["name"].as_str().unwrap_or_default().to_string(),
            sources: vec![card["desc"].as_str().unwrap_or_default().to_string()],
            links: note.links.clone(),
            referenced: note.referenced.clone(),
            need_fetch: false,
        };
        note.sources.extend(checklist_names);

        // note.collect_links();
        // note.dump();

        Ok(note)
    }

    fn name(&self) -> String {
        String::from("Trello")
    }
}
