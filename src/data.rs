use serde::{Deserialize, Serialize};

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
