use crate::models::GitHubRepo;
use octocrab::models::Repository;
use octocrab::Octocrab;

/// Possible errors that can occur when getting the content of a file in a repository.
pub enum FileContentGetError {
    NoFileContentFound,
    FileContentGetFailed(String),
}

impl std::fmt::Display for FileContentGetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileContentGetError::NoFileContentFound => {
                write!(f, "No content found for the file.")
            }
            FileContentGetError::FileContentGetFailed(e) => {
                write!(f, "Failed to get content of the file: {}", e)
            }
        }
    }
}

/// Possible errors that can occur when creating a fork of the upstream repository.
pub enum ForkCreationError {
    ForkCreationFailed(String),
    ForkResponseReadFailed(String),
    ForkResponseParseFailed(String),
}

/// Possible errors that can occur when fetching forks of the upstream repository or reading necessary information.
#[derive(Debug)]
pub enum ForkFetchingError {
    ForkFetchingFailed(String),
}

/// Possible errors that can occur when interacting with the GitHub API.
#[derive(Debug)]
pub enum GitHubApiError {
    /// Error when the `user` info cannot be retrieved from the GitHub API.
    UserGetFailed,
}

impl std::fmt::Display for GitHubApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitHubApiError::UserGetFailed => {
                write!(f, "Failed to get user information from GitHub API.")
            }
        }
    }
}

/// Possible errors that can occur when validating a GitHub repository with its upstream.
pub enum RepoUpstreamValidationError {
    NotAForkOfUpstream,
    NotAFork,
    DoNotExist,
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

    /// Create a fork of the upstream repository using the information given.
    ///
    /// # Arguments
    /// - `repo`: A [GitHubRepo](GitHubRepo) containing the information to use to create the fork.
    /// - `upstream`: The upstream repository to fork.
    ///
    /// # Returns
    /// - `Result<GitHubRepo, ForkCreationError>`:
    ///     - `Ok(GitHubRepo)`: The forked repository if successful.
    ///     - `Err(ForkCreationError)`: The error that occurred during the fork creation process.
    pub async fn create_fork(
        &self,
        repo: &GitHubRepo,
        upstream: &GitHubRepo,
    ) -> Result<GitHubRepo, ForkCreationError> {
        // Create a fork of the upstream repository using the information given.
        let response = self
            .octocrab
            ._post(
                format!("/repos/{}/{}/forks", upstream.owner, upstream.name),
                Some(&serde_json::json!({
                    "organization": repo.owner,
                    "name": repo.name,
                })),
            )
            .await;

        if response.is_err() {
            return Err(ForkCreationError::ForkCreationFailed(
                response.unwrap_err().to_string(),
            ));
        }

        let response_body = self.octocrab.body_to_string(response.unwrap()).await;
        if response_body.is_err() {
            return Err(ForkCreationError::ForkResponseReadFailed(
                response_body.unwrap_err().to_string(),
            ));
        }

        let json_response = serde_json::from_str(&*response_body.unwrap());
        match { json_response } {
            Ok(repo) => {
                // Parse the response to get a GitHubRepo object and return it.
                let repo: Repository = repo;
                Ok(GitHubRepo::new(repo.owner.unwrap().login, repo.name))
            }
            Err(e) => Err(ForkCreationError::ForkResponseParseFailed(e.to_string())),
        }
    }

    /// Get the **decoded** content of a file in a repository.
    ///
    /// # Arguments
    /// - `repo`: The repository to get the file from.
    /// - `path`: The path to the file in the repository.
    ///
    /// # Returns
    /// - `Result<String, FileContentGetError>`
    ///     - `Ok(String)`: The decoded content of the file.
    ///     - `Err(FileContentGetError)`: The error that occurred during the file content fetching process.
    pub async fn get_file_content(&self, repo: &GitHubRepo, path: &str) -> Result<String, FileContentGetError> {
        let repo = self.octocrab.repos(repo.owner.clone(), repo.name.clone());
        let content = repo.get_content().path(path).r#ref("main").send().await;
        match content {
            Ok(mut content) => {
                let contents = content.take_items();
                let c = &contents[0];
                let decoded_content = c.decoded_content();
                match decoded_content {
                    Some(decoded_content) => Ok(decoded_content),
                    None => Err(FileContentGetError::NoFileContentFound),
                }
            }
            Err(e) => Err(FileContentGetError::FileContentGetFailed(e.to_string())),
        }
    }

    /// Get the forks of the upstream repository.
    ///
    /// # Arguments
    /// - `upstream`: The upstream repository to get the forks of.
    ///
    /// # Returns
    /// - `Result<Vec<GitHubRepo>, ForkFetchingError>`:
    ///     - `Ok(Vec<GitHubRepo>)`: A list of [GitHubRepo](GitHubRepo) representing the forks of the upstream repository.
    ///     - `Err(ForkFetchingError)`: The error that occurred during the fork fetching process.
    pub async fn get_forks(
        &self,
        upstream: &GitHubRepo,
    ) -> Result<Vec<GitHubRepo>, ForkFetchingError> {
        let mut forks = Vec::new();
        let mut page: u32 = 1;

        loop {
            // Get the forks of the upstream repository from the GitHub API page by page.
            let response = match self
                .octocrab
                .repos(upstream.owner.clone(), upstream.name.clone())
                .list_forks()
                .page(page)
                .per_page(100)
                .send()
                .await
            {
                Ok(response) => response,
                Err(e) => return Err(ForkFetchingError::ForkFetchingFailed(e.to_string())),
            };

            // This is to attempt to get the next page of forks, so it is possible to know if there's
            // another page of forks to fetch.
            // Yes, this is a bit of a hacky way...
            // If you know a better way to do this, please let me know (or submit a PR)!
            // - Cubik
            let next_page = response.next.clone();

            // Only include the owner and name of the forked repository.
            forks.extend(
                response
                    .into_iter()
                    .map(|repo| GitHubRepo::new(repo.owner.unwrap().login, repo.name)),
            );

            // If there's no next page, break out of the loop.
            if next_page.is_none() {
                break;
            }

            // Move to the next page.
            page += 1;
        }

        Ok(forks)
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

    /// Get the fork of upstream repository that belongs to the currently signed in GitHub user.
    ///
    /// # Arguments
    /// - `upstream`: The upstream repository to get the user's fork of.
    ///
    /// # Returns
    /// - `Result<Option<GitHubRepo>, ForkFetchingError>`:
    ///     - `Ok(Option<GitHubRepo>)`
    ///         - `Some(GitHubRepo)`: A [GitHubRepo](GitHubRepo) representing the user's fork of the upstream repository.
    ///         - `None`: If the currently signed in GitHub user does not have a fork of the upstream repository.
    ///     - `Err(ForkFetchingError)`: The error that occurred during the fork fetching process.
    pub async fn get_user_fork(
        &self,
        upstream: &GitHubRepo,
    ) -> Result<Option<GitHubRepo>, ForkFetchingError> {
        // Get all forks of the upstream repository.
        let forks = self.get_forks(upstream).await;
        match forks {
            // If the forks are successfully retrieved, find the fork that belongs to the currently signed in GitHub user.
            Ok(forks) => {
                // Find the fork that belongs to the currently signed in GitHub user.
                for fork in forks {
                    if fork.owner == self.username {
                        return Ok(Some(fork));
                    }
                }
                // If the user does not have a fork of the upstream repository, return `None`.
                Ok(None)
            }
            // If the forks are not successfully retrieved, return an error message.
            Err(e) => Err(e),
        }
    }

    /// Validate that the given repository is a fork of the given upstream repository.
    ///
    /// # Arguments
    /// - `repo`: The repository to validate.
    /// - `upstream`: The upstream repository that the repository should be a fork of.
    ///
    /// # Returns
    /// - `Result<(), RepoUpstreamValidationError>`:
    ///     - `Ok(())`: If the repository is a fork of the upstream repository.
    ///     - `Err(RepoUpstreamValidationError::NotAForkOfUpstream)`: If the repository is not a fork of the provided upstream repository.
    ///     - `Err(RepoUpstreamValidationError::NotAFork)`: If the repository is not a fork.
    ///     - `Err(RepoUpstreamValidationError::DoNotExist)`: If the repository does not exist.
    pub async fn validate_repo_upstream(
        &self,
        repo: &GitHubRepo,
        upstream: &GitHubRepo,
    ) -> Result<(), RepoUpstreamValidationError> {
        // Get the repository information from the GitHub API.
        if let Ok(repo) = self.octocrab.repos(&repo.owner, &repo.name).get().await {
            if repo.fork.unwrap_or(false) {
                // Check if the parent of the repository is the upstream repository by comparing the full names.
                if repo.parent.unwrap().full_name.unwrap() == upstream.get_full_name() {
                    Ok(())
                } else {
                    Err(RepoUpstreamValidationError::NotAForkOfUpstream)
                }
            } else {
                Err(RepoUpstreamValidationError::NotAFork)
            }
        } else {
            Err(RepoUpstreamValidationError::DoNotExist)
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
