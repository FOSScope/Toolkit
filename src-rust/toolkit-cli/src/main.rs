use std::io::{stdin, stdout, Write};
use fosscopetoolkit_core::apis::GitHubApi;
use fosscopetoolkit_core::config::config::config_process;
use fosscopetoolkit_core::config::github::GitHubAccount;
use fosscopetoolkit_core::utils::github::github_login::{github_login, GitHubLoginError};

async fn login() -> Result<GitHubApi, GitHubLoginError> {
    let file_path = std::path::Path::new(".fosscope_toolkit/github_account.json");
    if file_path.exists() {
        let file = std::fs::File::open(file_path);
        match file {
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                let github_account = serde_json::from_reader(reader);
                match github_account {
                    Ok(github_account) => {
                        github_login(&github_account).await
                    }
                    Err(_) => {
                        Err(GitHubLoginError::ParseError)
                    }
                }
            }
            Err(_) => {
                Err(GitHubLoginError::FileOpenError)
            }
        }
    } else {
        loop {
            let mut user_input = String::new();
            user_input.clear();
            print!("Please enter your GitHub username: ");
            let _ = stdout().flush();
            stdin().read_line(&mut user_input).unwrap_or(0);
            let username = user_input.trim().to_string();
            user_input.clear();
            print!("Please enter your GitHub personal access token: ");
            let _ = stdout().flush();
            stdin().read_line(&mut user_input).unwrap_or(0);
            let token = user_input.trim().to_string();

            let github_account = GitHubAccount::new(username, token);
            let github = github_login(&github_account).await;
            match github {
                Ok(github) => {
                    let json_str = serde_json::to_string(&github_account);
                    match json_str {
                        Ok(json_str) => {
                            match std::fs::create_dir_all(".fosscope_toolkit") {
                                Ok(_) => {}
                                Err(_) => return Err(GitHubLoginError::DirCreateError)
                            }
                            let file = std::fs::File::create(file_path);
                            if file.is_err() {
                                return Err(GitHubLoginError::FileCreateError)
                            }
                            return match file.unwrap().write_all(json_str.as_bytes()) {
                                Ok(_) => Ok(github),
                                Err(_) => Err(GitHubLoginError::FileWriteError)
                            }
                        }
                        Err(_) => {
                            eprintln!("Failed to save the GitHub account.");
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    match e {
                        GitHubLoginError::AuthenticationError(message) => {
                            eprintln!(
                                "Failed to authenticate with GitHub. Reason: {}.\
                                Please try again.", message
                            );
                            continue;
                        }
                        _ => {
                            return Err(e);
                        }
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let config = config_process();
    if config.is_err() {
        eprintln!("Error creating/loading config: {:?}", config.err().unwrap());
        std::process::exit(1);
    }
    let config = config.unwrap();

    let github = match login().await {
        Ok(github) => github,
        Err(e) => {
            eprintln!("Error logging in with GitHub: {:?}", e);
            std::process::exit(1);
        }
    };
}
