use confique::Config;

#[derive(Config)]
pub struct AppConfig {
    #[config(nested)]
    pub github: GitHubConfig,
    #[config(nested)]
    pub upstream_repo: UpstreamRepoConfig,
    #[config(nested)]
    pub contributor_repo: ContributorRepoConfig,
}

#[derive(Config)]
pub struct GitHubConfig {
    pub username: String,
    pub token: String,
}

#[derive(Config)]
pub struct UpstreamRepoConfig {
    pub owner: String,
    pub repo: String,
}

#[derive(Config)]
pub struct ContributorRepoConfig {
    pub owner: Option<String>,
    pub repo: Option<String>,
}
