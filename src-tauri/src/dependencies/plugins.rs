use super::note::Note;

pub mod github;
pub mod slack;
pub mod trello;

pub trait Plugin {
    fn is_match(&self, url: &str) -> bool;
    fn normalize_url(&self, url: &str) -> String;
    fn fetch_note(&self, note: &Note) -> Result<Note, Box<dyn std::error::Error>>;
    fn name(&self) -> String;
}
