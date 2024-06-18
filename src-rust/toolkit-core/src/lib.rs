mod apis;

#[cfg(test)]
mod tests {
    use confique::Config;

    #[derive(Config)]
    pub struct AppConfig {
        pub github_username: String,
        pub github_token: String,
    }

    use crate::apis::GitHubApi;

    #[tokio::test]
    async fn github_verify_user() {
        let config = AppConfig::builder()
            .file("config/cli.dev.toml")
            .load().unwrap();
        let github_api = GitHubApi::new(config.github_username, config.github_token);
        assert!(github_api.verify_user().await);
    }
}
