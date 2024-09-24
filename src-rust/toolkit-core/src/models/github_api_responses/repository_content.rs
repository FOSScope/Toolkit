use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_url: String,
    pub git: String,
    pub html: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileContent {
    pub encoding: Option<String>,
    pub size: usize,
    pub name: String,
    pub path: String,
    pub content: Option<String>,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: String,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Serialize, Deserialize)]
pub struct DirContent {
    pub size: usize,
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: Option<String>,
    #[serde(rename = "_links")]
    pub links: Links,
    pub entries: Option<Vec<RepositoryContent>>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RepositoryContent {
    #[serde(rename = "file")]
    File(FileContent),
    #[serde(rename = "dir")]
    Dir(DirContent),
}
