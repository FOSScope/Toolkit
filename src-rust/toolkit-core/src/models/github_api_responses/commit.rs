use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub site_admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    pub signature: Option<String>,
    pub payload: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Tree {
    pub url: String,
    pub sha: String,
}

#[derive(Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct GitCommit {
    pub url: String,
    pub author: Struct,
    pub committer: Struct,
    pub message: String,
    pub tree: Tree,
    pub comment_count: i64,
    pub verification: Verification,
}

#[derive(Serialize, Deserialize)]
pub struct Commit {
    pub url: String,
    pub sha: String,
    pub node_id: String,
    pub html_url: String,
    pub comments_url: String,
    pub commit: GitCommit,
    pub author: GitHubUser,
    pub committer: GitHubUser,
    pub parents: Vec<Tree>,
}
