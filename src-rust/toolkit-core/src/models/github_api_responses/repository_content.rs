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

impl FileContent {
    /// Get the decoded content of the GitHub file content.
    ///
    /// If the encoding is base64, the content will be decoded and returned.
    /// If the encoding is not base64, `None` will be returned.
    ///
    /// Decode Code Credit: https://github.com/XAMPPRocky/octocrab/blob/bd8b648282f4a912c839dc0f73f72c5b8d891d57/src/models/repos.rs#L228, licensed under the MIT license.
    pub fn decoded_content(&self) -> Option<String> {
        match self.encoding.as_deref() {
            Some("base64") => {
                use base64::Engine;
                self.content.as_ref().map(|c| {
                    let mut content = c.as_bytes().to_owned();
                    content.retain(|b| !b" \n\t\r\x0b\x0c".contains(b));
                    let c = base64::prelude::BASE64_STANDARD.decode(content).unwrap();
                    String::from_utf8_lossy(&c).into_owned()
                })
            }
            _ => None
        }
    }
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
