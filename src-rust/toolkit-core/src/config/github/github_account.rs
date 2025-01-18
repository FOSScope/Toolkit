/// A representation of the GitHub account that is used to interact with the GitHub API.
///
/// # Fields
/// - `username`: The username of the GitHub account.
/// - `token`: The personal access token of the GitHub account.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GitHubAccount {
    pub username: String,
    pub token: String,
}

impl GitHubAccount {
    /// Create a new GitHub account with the specified username and token.
    pub fn new(username: String, token: String) -> Self {
        Self { username, token }
    }
}
