use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub workdir: String,
    pub webroot: String,
    pub template: String,
    pub templateindex: String,
    pub contenttag: String,
    pub titletag: String,
    pub descriptiontag: String,
    pub keywordstag: String,
}
