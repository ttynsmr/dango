use super::super::note::Note;
use super::Plugin;
use github_rs::client::Executor;
use regex::Regex;
use serde_json::Value;
use std::env;

pub struct Github {}

impl Plugin for Github {
    fn is_match(&self, url: &str) -> bool {
        Regex::new(r##"https://github\.com/[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]/[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]/pull/[0-9]+"##).unwrap().is_match(url)
    }

    fn normalize_url(&self, url: &str) -> String {
        url.to_string()
    }

    fn fetch_note(&self, note: &Note) -> Result<Note, Box<dyn std::error::Error>> {
        let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not found");
        let client = github_rs::client::Github::new(github_token).unwrap();
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
                        normalized_url: self.normalize_url(&note.url),
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
}
