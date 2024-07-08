#[derive(serde::Deserialize)]
pub struct Article {
    #[serde(rename = "type")]
    pub article_type: String,
    pub description: String,
    pub directory: String,
}

#[derive(serde::Deserialize)]
pub struct Action {
    pub action: String,
    pub description: String,
    pub command: String,
}

#[derive(serde::Deserialize)]
pub struct GitRule {
    pub branch_naming: String,
    pub commit_message: String,
}

#[derive(serde::Deserialize)]
pub struct RepoRule {
    pub articles: Vec<Article>,
    pub actions: Vec<Action>,
    pub git: GitRule,
}
