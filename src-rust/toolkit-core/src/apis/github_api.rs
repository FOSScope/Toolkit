use octocrab::models::Repository;
use octocrab::Octocrab;
use serde_json;

use crate::models::GitHubRepo;

/// A wrapper around the GitHub API that provides a set of methods to interact with the GitHub API.
///
/// # Fields
/// - `username`: The username of the currently signed in GitHub user.
/// - `octocrab`: The Octocrab client that is used to interact with the GitHub API.
pub struct GitHubApi {
    username: String,
    octocrab: Octocrab,
}

impl GitHubApi {
    pub fn new(username: String, octocrab: Octocrab) -> Self {
        Self {
            username,
            octocrab,
        }
    }

    /// Get the username (login) of the currently signed in GitHub user.
    ///
    /// # Returns
    /// - `Result<String, &str>`: The username of the currently signed-in GitHub user if successful, an error message otherwise.
    pub async fn get_user(&self) -> Result<String, &str> {
        // Get the user information from the GitHub API.
        let user = self.octocrab.current().user().await;
        match user {
            // If the user information is successfully retrieved, return the content of the `login` field (the username).
            Ok(user) => Ok(user.login),
            // If the user information is not successfully retrieved, return an error message.
            Err(_) => {
                Err(
                    "Failed to get the username of the currently signed in GitHub user. Could be due to an invalid token."
                )
            }
        }
    }

    /// Verify that the username of the currently signed in GitHub user matches the username
    ///
    /// # Returns
    /// - `bool`: `true` if the username of the currently signed in GitHub user matches the username provided to the GitHubApi, `false` otherwise.
    pub async fn verify_user(&self) -> bool {
        // Get the username of the currently signed in GitHub user.
        let user = self.get_user().await;
        match user {
            // If the username is successfully retrieved, compare it with the username provided to the GitHubApi.
            // Return `true` if they match, `false` otherwise.
            Ok(user) => {
                user == self.username
            }
            // If the username is not successfully retrieved, return `false` anyway.
            Err(_) => false,
        }
    }

    /// Validate that the given repository is a fork of the given upstream repository.
    ///
    /// # Arguments
    /// - `repo`: The repository to validate.
    /// - `upstream`: The upstream repository that the repository should be a fork of.
    ///
    /// # Returns
    /// - `Result<(), &str>`: `Ok(())` if the repository is a fork of the upstream repository, an error message indicating why the repository is not a fork otherwise.
    pub async fn validate_repo(&self, repo: GitHubRepo, upstream: GitHubRepo) -> Result<(), &str> {
        // Get the repository information from the GitHub API.
        if let Ok(repo) = self.octocrab.repos(repo.owner, repo.name).get().await {
            if repo.fork.unwrap_or(false) {
                // Check if the parent of the repository is the upstream repository by comparing the full names.
                if repo.parent.unwrap().full_name.unwrap() == upstream.get_full_name() {
                    Ok(())
                } else {
                    Err("Repository is not a fork of the upstream repository")
                }
            } else {
                Err("Repository is not a fork")
            }
        } else {
            Err("Repository does not exist")
        }
    }

    /// Get the forks of the upstream repository.
    ///
    /// # Arguments
    /// - `upstream`: The upstream repository to get the forks of.
    ///
    /// # Returns
    /// - `Vec<GitHubRepo>`: A list of [GitHubRepo](GitHubRepo) representing the forks of the upstream repository.
    pub async fn get_forks(&self, upstream: GitHubRepo) -> Vec<GitHubRepo> {
        let mut forks = Vec::new();
        let mut page: u32 = 1;

        loop {
            // Get the forks of the upstream repository from the GitHub API page by page.
            let response = self.octocrab.repos(
                upstream.owner.clone(), upstream.name.clone(),
            ).list_forks().page(page).per_page(100).send().await.unwrap();

            // This is to attempt to get the next page of forks, so it is possible to know if there's
            // another page of forks to fetch.
            // Yes, this is a bit of a hacky way...
            // If you know a better way to do this, please let me know (or submit a PR)!
            // - Cubik
            let next_page = response.next.clone();

            // Only include the owner and name of the forked repository.
            forks.extend(response.into_iter().map(
                |repo| GitHubRepo::new(repo.owner.unwrap().login, repo.name)
            ));

            // If there's no next page, break out of the loop.
            if next_page.is_none() {
                break;
            }

            // Move to the next page.
            page += 1;
        }

        forks
    }

    /// Get the fork of upstream repository that belongs to the currently signed in GitHub user.
    ///
    /// # Arguments
    /// - `upstream`: The upstream repository to get the user's fork of.
    ///
    /// # Returns
    /// - `Result<GitHubRepo, &str>`
    ///     - `Ok(GitHubRepo)`: A [GitHubRepo](GitHubRepo) representing the user's fork of the upstream repository.
    ///     - `Err(&str)`: An error message indicating why the method failed.
    pub async fn get_user_fork(&self, upstream: GitHubRepo) -> Result<GitHubRepo, &str> {
        // Get all forks of the upstream repository.
        let forks = self.get_forks(upstream).await;

        // Find the fork that belongs to the currently signed in GitHub user.
        for fork in forks {
            if fork.owner == self.username {
                return Ok(fork);
            }
        }

        Err("User has not forked the upstream repository.")
    }

    /// Create a fork of the upstream repository using the information given.
    ///
    /// # Arguments
    /// - `repo`: A [GitHubRepo](GitHubRepo) containing the information to use to create the fork.
    /// - `upstream`: The upstream repository to fork.
    ///
    /// # Returns
    /// - `Result<GitHubRepo, &str>`
    ///     - `Ok(GitHubRepo)`: A [GitHubRepo](GitHubRepo) representing the created fork.
    ///     - `Err(&str)`: An error message indicating why the method failed.
    pub async fn create_fork(&self, repo: GitHubRepo, upstream: GitHubRepo) -> Result<GitHubRepo, &str> {
        // Create a fork of the upstream repository using the information given.
        let response = self.octocrab._post(
            format!("/repos/{}/{}/forks", upstream.owner, upstream.name),
            Some(&serde_json::json!({
                "organization": repo.owner,
                "name": repo.name,
            })),
        ).await;

        if response.is_err() {
            return Err("Failed to create fork");
        }

        let response_body = self.octocrab.body_to_string(response.unwrap()).await;
        if response_body.is_err() {
            return Err("Failed to read fork response");
        }

        let json_response = serde_json::from_str(&*response_body.unwrap());
        match { json_response } {
            Ok(repo) => {
                // Parse the response to get a GitHubRepo object and return it.
                let repo: Repository = repo;
                Ok(GitHubRepo::new(repo.owner.unwrap().login, repo.name))
            }
            Err(_) => Err("Failed to parse fork response"),
        }
    }

    /// Get the **decoded** content of a file in a repository.
    ///
    /// # Arguments
    /// - `repo`: The repository to get the file from.
    /// - `path`: The path to the file in the repository.
    ///
    /// # Returns
    /// - `Result<String, &str>`
    ///     - `Ok(String)`: The decoded content of the file.
    ///     - `Err(&str)`: An error message indicating why the method failed.
    pub async fn get_file_content(&self, repo: &GitHubRepo, path: &str) -> Result<String, &str> {
        let repo = self.octocrab.repos(repo.owner.clone(), repo.name.clone());
        let content = repo
            .get_content()
            .path(path)
            .r#ref("main")
            .send()
            .await;
        match content {
            Ok(mut content) => {
                let contents = content.take_items();
                let c = &contents[0];
                let decoded_content = c.decoded_content();
                match decoded_content {
                    Some(decoded_content) => Ok(decoded_content),
                    None => Err("No file content found"),
                }
            }
            Err(_) => Err("Failed to get file content"),
        }
    }
}
