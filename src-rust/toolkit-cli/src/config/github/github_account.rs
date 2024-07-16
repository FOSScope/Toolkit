use octocrab::Octocrab;
use fosscopetoolkit_core::apis::GitHubApi;

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

    pub async fn login(&self) -> Result<GitHubApi, &str> {
        let octocrab = Octocrab::builder().personal_token(
            self.token.clone()
        ).build();

        match octocrab {
            Ok(octocrab) => {
                let github = GitHubApi::new(
                    self.username.clone(),
                    octocrab
                );
                if !github.verify_user().await {
                    Err("Failed to verify the GitHub user. Check your token.")
                } else {
                    Ok(github)
                }
            },
            Err(_) => Err("Failed to login to GitHub. Check your token.")
        }
    }
}
