use super::{super::note::Note, Plugin};
use regex::Regex;
use std::env;

pub struct Slack {}

impl Plugin for Slack {
    fn is_match(&self, url: &str) -> bool {
        Regex::new(r##"https://[a-zA-Z0-9][a-zA-Z0-9-]+[a-zA-Z0-9]\.slack\.com/"##)
            .unwrap()
            .is_match(url)
            && url.split("/").collect::<Vec<&str>>().len() >= 6
    }

    fn normalize_url(&self, url: &str) -> String {
        url.to_string()
    }

    fn fetch_note(&self, note: &Note) -> Result<Note, Box<dyn std::error::Error>> {
        let slack_token = env::var("SLACK_BOT_TOKEN").expect("SLACK_BOT_TOKEN is not found");

        let splitted_url = note.url.split("/").collect::<Vec<&str>>();
        if splitted_url.len() < 6 {
            let e = std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid Url");
            return Err(Box::new(e));
        }
        let slack_channel = splitted_url[4];
        let slack_thread_ts = splitted_url[5];

        let thread_ts_integer = &slack_thread_ts[1..][..slack_thread_ts.len() - (1 + 6)];
        let thread_ts_decimal = &slack_thread_ts[slack_thread_ts.len() - (6)..];
        // println!(
        //     "split thread_ts {} -> {} {} {}",
        //     slack_thread_ts,
        //     &slack_thread_ts[..1],
        //     thread_ts_integer,
        //     thread_ts_decimal
        // );

        // println!("channel:{} thread_ts:{}", slack_channel, slack_thread_ts);

        let mut replies_json: String = String::default();
        match reqwest::blocking::Client::new()
            .get(format!(
                "https://slack.com/api/conversations.replies?channel={}&ts={}.{}&pretty=1",
                slack_channel, thread_ts_integer, thread_ts_decimal
            ))
            .bearer_auth(slack_token)
            .send()
        {
            Ok(replies_json_result) => {
                replies_json = replies_json_result.text()?;
            }
            Err(e) => println!("{}", e),
        }
        // print!("{}", replies_json);
        let mut replies = json::parse(&replies_json).unwrap();

        let mut new_note = Note::default();
        new_note.normalized_url = self.normalize_url(&note.url);
        new_note.url = note.url.clone();
        new_note.sources.extend(
            replies["messages"]
                .members_mut()
                .map(|m| m["text"].as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
        );
        new_note.title = new_note.sources.join(", ");
        new_note.need_fetch = false;
        // new_note.dump();
        // println!("{}", new_note.sources.join(",\n"));

        Ok(new_note)
    }
}
