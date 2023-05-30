use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub workdir: String,
    pub metadatafilename: String,
    pub template: String,
    pub contenttag: String,
    pub titletag: String,
    pub descriptiontag: String,
    pub keywordstag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub description: String,
    pub keywords: Vec<String>,
}

#[derive(Debug)]
pub struct Post {
    pub path: String,
    pub folder: String,
    pub markdown: String,
    pub metadata: Option<Metadata>,
    pub html: String,
}

impl Post {
    pub fn markdown_to_html(&mut self) {
        let contents = fs::read_to_string(self.path.clone()).expect("unable to read file");
        self.markdown = contents.clone();
        // convert contents to html
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(&contents, options);
        let mut html_out = String::new();
        html::push_html(&mut html_out, parser);
        // html
        self.html = html_out;
    }

    pub fn mangle_template(&mut self, template: &String, settings: &Settings) {
        let metadata = &self.metadata.as_ref().expect("could not get post metadata");
        let mut contents = template.clone();
        contents = contents.replace(&settings.titletag, &metadata.title);
        contents = contents.replace(&settings.descriptiontag, &metadata.description);
        contents = contents.replace(&settings.keywordstag, &metadata.keywords.join(", "));
        contents = contents.replace(&settings.contenttag, &self.html);
        self.html = contents;
    }

    pub fn save_html(&mut self) {
        let file_path = format!("{}/index.html", &self.folder);
        fs::write(file_path, &self.html).expect("could not write to html file");
    }
}
