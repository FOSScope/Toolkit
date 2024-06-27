use confique::Config;

#[derive(Config)]
pub struct AppConfig {
    pub editor: String,
    #[config(nested)]
    pub github: GitHubConfig,
}

#[derive(Config)]
pub struct GitHubConfig {
    pub username: String,
    pub token: String,
}
