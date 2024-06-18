use octocrab::Octocrab;

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

    async fn get_user(&self) -> String {
        self.octocrab.current().user().await.unwrap().login
    }

    pub async fn verify_user(&self) -> bool {
        self.username == self.get_user().await
    }
}
