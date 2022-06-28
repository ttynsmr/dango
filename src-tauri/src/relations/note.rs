use linkify::LinkFinder;
use std::collections::HashSet;

pub struct Note {
    pub url: String,
    pub title: String,
    pub sources: Vec<String>,
    pub links: HashSet<String>,      // 参照先
    pub referenced: HashSet<String>, // 参照元
    pub need_fetch: bool,
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

    pub fn to_string(&self) -> String {
        format!(
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
        )
    }

    pub fn dump(&self) {
        println!("{}", self.to_string());
    }
}
