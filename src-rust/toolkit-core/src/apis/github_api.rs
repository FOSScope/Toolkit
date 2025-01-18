use octocrab::Octocrab;

/// Possible errors that can occur when interacting with the GitHub API.
#[derive(Debug)]
pub enum GitHubApiError {
    /// Error when the `user` info cannot be retrieved from the GitHub API.
    UserGetFailed,
}

impl std::fmt::Display for GitHubApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitHubApiError::UserGetFailed => write!(f, "Failed to get user information from GitHub API."),
        }
    }
}

/// A wrapper around the GitHub API that provides a set of methods to interact with the GitHub API.
///
/// # Fields
/// - `username`: The username of the currently signed in GitHub user.
/// - `octocrab`: The Octocrab client used to interact with the GitHub API.
pub struct GitHubApi {
    username: String,
    octocrab: Octocrab,
}

impl GitHubApi {
    pub fn new(username: String, octocrab: Octocrab) -> Self {
        Self { username, octocrab }
    }

    /// Get the username (login) of the currently signed in GitHub user.
    ///
    /// # Returns
    /// - `Result<String, GitHubApiError>`: The username of the currently signed-in GitHub user if successful, an error indicating the failure otherwise.
    pub async fn get_user(&self) -> Result<String, GitHubApiError> {
        // Get the user information from the GitHub API.
        let user = self.octocrab.current().user().await;
        match user {
            // If the user information is successfully retrieved, return the content of the `login` field (the username).
            Ok(user) => Ok(user.login),
            // If the user information is not successfully retrieved, return an error message.
            Err(_) => Err(GitHubApiError::UserGetFailed),
        }
    }

    /// Verify that the username of the currently signed in GitHub user
    /// matches the provided username to the GitHubApi instance.
    ///
    /// # Returns
    /// - `Result<bool, GitHubApiError>`:
    ///     - `Ok(true)`: If the username matches the provided username.
    ///     - `Ok(false)`: If the username does not match the provided username.
    ///     - `Err(GitHubApiError)`: If an error occurs during the verification process.
    pub async fn verify_user(&self) -> Result<bool, GitHubApiError> {
        // Get the username of the currently signed in GitHub user.
        let user = self.get_user().await;
        match user {
            // If the username is successfully retrieved, compare it with the username provided to the GitHubApi.
            // Return `true` if they match, `false` otherwise.
            Ok(user) => Ok(user == self.username),
            // If the username is not successfully retrieved, return the error.
            Err(e) => Err(e),
        }
    }
}
