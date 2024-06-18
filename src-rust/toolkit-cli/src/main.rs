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
    println!("GitHub username: {}", config.github_username);
    println!("GitHub token: {}", config.github_token);

    Ok(())
}
