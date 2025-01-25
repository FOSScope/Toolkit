use crate::apis::GitHubApi;
use crate::config::github::GitHubAccount;
use octocrab::Octocrab;

/// Possible errors that can occur when logging in with GitHub.
#[derive(Debug)]
pub enum GitHubLoginError {
    /// Error occurred while trying to authenticate with the GitHub API.
    AuthenticationError(String),
    /// Error when cannot create the configuration directory.
    DirCreateError,
    /// Error when cannot create the configuration file.
    FileCreateError,
    /// Error when cannot open the configuration file.
    FileOpenError,
    /// Error when cannot write to the configuration file.
    FileWriteError,
    /// Error occurred when Octocrab client was failed to build.
    OctocrabBuildError,
    /// Error when cannot parse the GitHub Account data from stored JSON.
    ParseError,
}

/// Login to the GitHub API using the personal access token.
///
/// # Returns
/// - `Result<GitHubApi, GitHubLoginError>`:
///     - `Ok(GitHubApi)`: The [`GitHubApi`](GitHubApi) wrapper if the login is successful.
///     - `Err(GitHubLoginError)`: Reason why the GitHub login process failed.
pub async fn github_login(github_account: &GitHubAccount) -> Result<GitHubApi, GitHubLoginError> {
    // Create an Octocrab client with the personal access token.
    let octocrab = Octocrab::builder()
        .personal_token(github_account.token.clone())
        .build();
    match octocrab {
        Ok(octocrab) => {
            // Create a GitHubApi wrapper with the username and the Octocrab client.
            let github = GitHubApi::new(github_account.username.clone(), octocrab);
            // Verify the GitHub user.
            let user_verification = github.verify_user().await;
            match user_verification {
                Ok(true) => Ok(github),
                Ok(false) => Err(GitHubLoginError::AuthenticationError(
                    "UsernameMismatch".to_string(),
                )),
                Err(e) => Err(GitHubLoginError::AuthenticationError(e.to_string())),
            }
        }
        Err(_) => Err(GitHubLoginError::OctocrabBuildError),
    }
}
