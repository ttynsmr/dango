use dotenv::dotenv;
use github_rs::client::{Executor, Github};
use linkify::LinkFinder;
use regex::Regex;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::env;

struct Note {
    url: String,
    title: String,
    sources: Vec<String>,
    links: HashSet<String>,
    referenced: HashSet<String>,
    need_fetch: bool,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            url: String::from(""),
            title: String::from(""),
            sources: Vec::new(),
            links: HashSet::new(),
            referenced: HashSet::new(),
            need_fetch: true,
        }
    }
}

impl Note {
    pub fn collect_links(&mut self) {
        let mut finder = LinkFinder::new();
        finder.url_must_have_scheme(true);
        // let prev_link_count = self.links.len();
        for source in self.sources.iter() {
            self.links
                .extend(finder.links(&source).map(|link| link.as_str().to_string()));
        }
        self.links.extend(
            finder
                .links(&self.title)
                .map(|link| link.as_str().to_string()),
        );
        // if prev_link_count != self.links.len() {
        //     println!("{} new link appended", self.links.len() - prev_link_count);
        // }
    }

    pub fn referenced_from(&mut self, from: String) -> &mut Self {
        self.referenced.insert(from);
        return self;
    }

    pub fn dump(&self) {
        println!(
            "[{}]({}) {}\n\tlinks:\n\t\t{}\n\treferenced:\n\t\t{}\n",
            self.title,
            self.url,
            self.need_fetch,
            self.links
                .clone()
                .into_iter()
                .collect::<Vec<String>>()
                .join("\n\t\t"),
            self.referenced
                .clone()
                .into_iter()
                .collect::<Vec<String>>()
                .join("\n\t\t"),
        );
    }
}

struct Notes {
    notes: Option<HashMap<String, Note>>,
}

impl Notes {
    pub fn new() -> Self {
        Notes {
            notes: Some(HashMap::new()),
        }
    }

    pub fn append(&mut self, note: Note) {
        match self.notes.as_mut() {
            Some(notes) => {
                notes.insert(note.url.clone(), note);
            }
            None => {
                println!("None None");
            }
        }
    }

    pub fn referenced_from(&mut self, link: String, from: String) {
        match self.notes.as_mut() {
            Some(notes) => {
                notes
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
            None => {}
        }
    }

    pub fn analyze(&mut self) -> bool {
        // collect links
        match self.notes.as_mut() {
            Some(notes) => {
                for note in notes.iter_mut() {
                    note.1.collect_links();
                }
            }
            None => {}
        }
        // create reference list
        let mut pairs: Vec<(String, String)> = Vec::new();
        match self.notes.as_ref() {
            Some(notes) => {
                for note in notes.values() {
                    let links = note.links.clone();
                    for link in links.iter() {
                        pairs.push((link.clone(), note.url.clone()));
                    }
                }
            }
            None => {}
        }
        // apply reference
        for (to, from) in pairs {
            self.referenced_from(to, from);
        }
        // fetch dirty note
        let mut fetch_request_count = 0;
        let mut fetched_notes: Vec<Note> = Vec::new();
        match self.notes.as_mut() {
            Some(notes) => {
                let needs_fetch_notes = notes
                    .values()
                    .filter(|&n| n.need_fetch && n.title.is_empty());
                let mut needs_fetch_notes2 = notes.values().filter(|&n| n.need_fetch);
                for note in needs_fetch_notes {
                    if note.url.starts_with("https://trello.com/") {
                        let card_id = note.url.split("/").collect::<Vec<&str>>()[4];
                        println!("need fetch as trello {} card_id: {}", note.url, card_id);
                        let note = needs_fetch_notes2.find(|v| v.url == note.url).unwrap();
                        fetch_request_count += 1;
                        match trello_fetch_note(&note) {
                            Ok(note) => {
                                fetched_notes.push(note);
                            }
                            Err(e) => println!("{}", e),
                        }
                    } else if note.url.starts_with("https://github.com/") {
                        if Regex::new(r##"https://github\.com/[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]/[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]/pull/[0-9]+"##).unwrap().is_match(&note.url) {
                            println!("need fetch as github {}", note.url);
                            fetch_request_count += 1;
                            match github_fetch_note(&note) {
                                Ok(note) => {
                                    fetched_notes.push(note);
                                }
                                Err(e) => println!("{}", e),
                            }
                        }
                    } else if Regex::new(
                        r##"https://[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.slack\.com/"##,
                    )
                    .unwrap()
                    .is_match(&note.url)
                    {
                        // println!("need fetch as slack {}", note.url);
                        // match slack_fetch_note(&note) {
                        //     Ok(note) => {
                        //         fetched_notes.push(note);
                        //     }
                        //     Err(e) => println!("{}", e),
                        // }
                    } else {
                        // println!("unknown url {}", note.url);
                    }
                }
            }
            None => {}
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

    pub fn dump(&self) {
        for note in self.notes.as_ref().unwrap().values() {
            note.dump();
        }
    }
}

fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // let slack_token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN is not found");
    let mut notes: Notes = Notes::new();
    match github_call(&mut notes, args.to_vec().split_off(1).join("+")) {
        Ok(()) => {}
        Err(e) => println!("{}", e),
    }
    match trello_call(&mut notes) {
        Ok(()) => {}
        Err(e) => println!("{}", e),
    }
    let mut phase = 1;
    while {
        println!(
            "=========================================== phase {}",
            phase
        );
        phase += 1;
        notes.analyze()
    } {}
    notes.dump();
}

fn github_fetch_note(note: &Note) -> Result<Note, Box<dyn std::error::Error>> {
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not found");
    let client = Github::new(github_token).unwrap();
    let params = note
        .url
        .strip_prefix("https://github.com/")
        .unwrap()
        .split("/")
        .collect::<Vec<&str>>();
    let issues_endpoint = format!("repos/{}/{}/pulls/{}", params[0], params[1], params[3]);
    let mut new_note: Note = Note::default();
    println!("issues_endpoint:{}", issues_endpoint);
    let me = client
        .get()
        .custom_endpoint(&issues_endpoint)
        .execute::<Value>();
    match me {
        Ok((_, _, json)) => {
            // println!("{:#?}", headers);
            // println!("{}", status);
            if let Some(json) = json {
                // println!("{} {}", json["title"], json["html_url"]);
                new_note = Note {
                    url: note.url.clone(),
                    title: json["title"].as_str().unwrap().to_string(),
                    sources: vec![json["body"].as_str().unwrap().to_string()],
                    links: new_note.links.clone(),
                    referenced: new_note.referenced.clone(),
                    need_fetch: false,
                };
            }
        }
        Err(e) => println!("{}", e),
    }
    // note.sources.extend(checklist_names);
    // note.collect_links();
    // new_note.dump();
    Ok(new_note)
}

fn github_call(notes: &mut Notes, query: String) -> Result<(), Box<dyn std::error::Error>> {
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not found");
    let client = Github::new(github_token).unwrap();
    let query = format!("search/issues?q={}", query);
    // println!("{}", query);
    let me = client.get().custom_endpoint(&query).execute::<Value>();
    match me {
        Ok((_, _, json)) => {
            // println!("{:#?}", headers);
            // println!("{}", status);
            if let Some(json) = json {
                // println!("{}", json.clone().to_string());
                for items in json["items"].as_array() {
                    for item in items {
                        // println!("{} {}", item["title"], item["html_url"]);
                        let mut note = Note {
                            url: item["html_url"].as_str().unwrap().to_string(),
                            title: item["title"].as_str().unwrap().to_string(),
                            sources: vec![item["body"].as_str().unwrap_or_default().to_string()],
                            links: HashSet::new(),
                            referenced: HashSet::new(),
                            need_fetch: false,
                        };
                        note.collect_links();
                        notes.append(note);
                    }
                }
            }
        }
        Err(e) => println!("{}", e),
    }
    Ok(())
}

fn trello_fetch_note(note: &Note) -> Result<Note, Box<dyn std::error::Error>> {
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

fn trello_call(notes: &mut Notes) -> Result<(), Box<dyn std::error::Error>> {
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
