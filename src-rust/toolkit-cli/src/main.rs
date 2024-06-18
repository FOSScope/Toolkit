mod config;

use config::AppConfig;
use confique::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Current directory: {:?}", std::env::current_dir()?);
    let config = AppConfig::builder()
        // Used to get the GitHub username and token while developing and testing the CLI
        .file("config/cli.dev.toml")
        // The proper config when in production
        .file("config/config.toml")
        .load()?;
    println!("GitHub username: {}", config.github.username);
    println!("GitHub token: {}", config.github.token);
    println!("Upstream repo owner: {}", config.upstream_repo.owner);
    println!("Upstream repo name: {}", config.upstream_repo.repo);
    println!("Contributor repo owner: {}",
             config.contributor_repo.owner.unwrap_or_else(|| config.github.username)
    );
    println!("Contributor repo name: {}",
             config.contributor_repo.repo.unwrap_or_else(|| config.upstream_repo.repo)
    );

    Ok(())
}
