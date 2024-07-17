use octocrab::Octocrab;

use fosscopetoolkit_core::apis::GitHubApi;

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
    pub fn new(username: String, token: String) -> Self {
        Self {
            username,
            token,
        }
    }

    /// Login to the GitHub API using the personal access token.
    ///
    /// # Returns
    /// - `Result<GitHubApi, &str>`:
    ///     - `Ok(GitHubApi)`: The [`GitHubApi`](GitHubApi) wrapper if the login is successful.
    ///     - `Err(&str)`: An error message indicating the reason for the login failure.
    pub async fn login(&self) -> Result<GitHubApi, &str> {
        // Create an Octocrab client with the personal access token.
        let octocrab = Octocrab::builder().personal_token(
            self.token.clone()
        ).build();

        match octocrab {
            Ok(octocrab) => {
                // Create a GitHubApi wrapper with the username and the Octocrab client.
                let github = GitHubApi::new(
                    self.username.clone(),
                    octocrab,
                );
                // Verify the GitHub user.
                if !github.verify_user().await {
                    Err("Failed to verify the GitHub user. Check your token.")
                } else {
                    Ok(github)
                }
            }

            // If the Octocrab client cannot be created, return an error message.
            Err(_) => Err("Failed to login to GitHub. Check your token.")
        }
    }
}
