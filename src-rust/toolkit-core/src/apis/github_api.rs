use octocrab::Octocrab;
use super::super::models::GitHubRepo;

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

    /**
     * Get the username (login) of the currently signed-in GitHub user.
     */
    async fn get_user(&self) -> String {
        self.octocrab.current().user().await.unwrap().login
    }

    /**
     * Verify that the username of the currently signed-in GitHub user matches the username
     * provided to the GitHubApi.
     */
    pub async fn verify_user(&self) -> bool {
        self.username == self.get_user().await
    }

    /**
     * Validate that the given repository exists and is a fork of the upstream repository.
     */
    pub async fn validate_repo(&self, repo: GitHubRepo, upstream: GitHubRepo) -> Result<(), &str> {
        if let Ok(repo) = self.octocrab.repos(repo.owner, repo.name).get().await {
            if repo.fork.unwrap_or(false) {
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
}
