use confique::Config;

#[derive(Config)]
pub struct AppConfig {
    pub github_username: String,
    pub github_token: String,
}
