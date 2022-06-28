use super::super::note::Note;
use super::super::notes::Notes;
use github_rs::client::{Executor, Github};
use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;
use std::env;

pub fn github_match(url: &str) -> bool {
    Regex::new(r##"https://github\.com/[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]/[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]/pull/[0-9]+"##).unwrap().is_match(url)
}

pub fn github_fetch_note(note: &Note) -> Result<Note, Box<dyn std::error::Error>> {
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

pub fn github_call(notes: &mut Notes, query: String) -> Result<(), Box<dyn std::error::Error>> {
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not found");
    let client = Github::new(github_token).unwrap();
    let query = format!("search/issues?q={}", query);
    println!("{}", query);
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
