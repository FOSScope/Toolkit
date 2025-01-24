use std::io::{stdin, stdout, Write};
use fosscopetoolkit_core::apis::github_api::RepoUpstreamValidationError;
use fosscopetoolkit_core::apis::GitHubApi;
use fosscopetoolkit_core::config::config;
use fosscopetoolkit_core::config::github::GitHubAccount;
use fosscopetoolkit_core::models::GitHubRepo;
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

async fn create_fork(github: &GitHubApi, upstream_repo: &GitHubRepo) -> GitHubRepo {
    loop {
        let result = fork_creation_process(github, upstream_repo).await;
        if result.is_some() {
            return result.unwrap();
        }
    }
}

async fn fork_creation_process(github: &GitHubApi, upstream_repo: &GitHubRepo) -> Option<GitHubRepo> {
    print!("Do you want to use another fork or create a new fork? (y/n) ");
    let mut user_input = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut user_input).unwrap_or(0);
    match user_input.to_lowercase().trim() {
        "y" | "yes" => {
            let owner = github.get_user().await.unwrap();
            println!("Please enter the name of the owner of the forked repository (default: {}):", owner);
            let mut fork_owner = String::new();
            stdin().read_line(&mut fork_owner).unwrap_or(0);
            fork_owner = fork_owner.trim().to_string();
            if fork_owner.is_empty() {
                fork_owner = owner;
            }
            let repo_name = upstream_repo.name.clone();
            println!("Please enter the name of the forked repository (default: {}):", repo_name);
            let mut fork_repo_name = String::new();
            stdin().read_line(&mut fork_repo_name).unwrap_or(0);
            fork_repo_name = fork_repo_name.trim().to_string();
            if fork_repo_name.is_empty() {
                fork_repo_name = repo_name;
            }
            println!("Creating a fork with the owner: {} and the repository name: {}", fork_owner, fork_repo_name);
            let fork_repo = GitHubRepo::new(fork_owner, fork_repo_name);

            // Check if the forked repository exists.
            let fork_exists = github.validate_repo_upstream(&fork_repo, upstream_repo).await;
            match fork_exists {
                Ok(_) => {
                    // Ask the user if they want to use the forked repository.
                    println!("The repo {} is a fork of the upstream repository.", fork_repo.get_full_name());
                    print!("Do you want to use this forked repository as the contributor repository? (y/n) ");
                    let mut user_input = String::new();
                    let _ = stdout().flush();
                    stdin().read_line(&mut user_input).unwrap_or(0);
                    match user_input.to_lowercase().trim() {
                        "y" | "yes" => {
                            Some(fork_repo)
                        }
                        _ => {
                            None
                        }
                    }
                }
                Err(RepoUpstreamValidationError::DoNotExist) => {
                    // Create a new forked repository and use it as the contributor repository.
                    let fork = github.create_fork(
                        &fork_repo, upstream_repo
                    ).await;
                    match fork {
                        Ok(fork) => {
                            Some(fork)
                        }
                        Err(_) => {
                            eprintln!("Failed to create the forked repository.");
                            std::process::exit(1);
                        }
                    }
                }
                Err(_) => {
                    // The repo either is not a fork or is not a fork of the upstream repository.
                    // Ask the user to provide a new owner/repository name.
                    None
                }
            }
        }
        _ => {
            println!("You should use a fork of the upstream repository to contribute. Exiting...");
            std::process::exit(0);
        }
    }
}

async fn contributor_repo_init(github: &GitHubApi, upstream_repo: &GitHubRepo) -> GitHubRepo {
    match github.get_user_fork(&upstream_repo).await {
        Ok(fork) => {
            match fork {
                Some(fork) => {
                    println!("You have forked the upstream repository: {}", upstream_repo.get_full_name());
                    println!("Your fork repo have the name: {}", fork.get_full_name());
                    print!("Do you want to use this forked repository as the contributor repository? (y/n) ");

                    let mut user_input = String::new();
                    let _ = stdout().flush();
                    stdin().read_line(&mut user_input).unwrap_or(0);

                    match user_input.to_lowercase().trim() {
                        "y" | "yes" => {
                            fork
                        }
                        _ => {
                            create_fork(github, upstream_repo).await
                        }
                    }
                }
                None => {
                    println!("You don't have a fork of the upstream repository: {}", upstream_repo.get_full_name());
                    create_fork(github, upstream_repo).await
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to get the user's fork of the upstream repository: {:?}", e);
            std::process::exit(1);
        }
    }
}

#[tokio::main]
async fn main() {
    let config = config::config_process();
    if config.is_err() {
        eprintln!("Error creating/loading config: {:?}", config.err().unwrap());
        std::process::exit(1);
    }
    let mut config = config.unwrap();

    let github = match login().await {
        Ok(github) => github,
        Err(e) => {
            eprintln!("Error logging in with GitHub: {:?}", e);
            std::process::exit(1);
        }
    };

    println!("Please select the upstream repository you want to work with:");
    println!("1. FOSScope/Articles - 开源观察原创文章与中文转载文章源文件");
    println!("2. FOSScope/TranslateProject - 开源观察翻译项目");

    let mut user_input = String::new();
    let mut valid_input = false;
    let mut upstream_repo: GitHubRepo = GitHubRepo::new("".to_string(), "".to_string());
    while !valid_input {
        stdin().read_line(&mut user_input).unwrap_or(0);
        upstream_repo = match user_input.trim() {
            "1" => {
                user_input.clear();
                eprintln!("Not implemented yet. Please select another option.");
                continue
            }
            "2" => {
                valid_input = true;
                GitHubRepo::new("FOSScope".to_string(), "TranslateProject".to_string())
            }
            _ => {
                user_input.clear();
                eprintln!("Invalid input. Please select an option from the list.");
                continue
            }
        }
    }
    user_input.clear();

    if upstream_repo.owner.is_empty() || upstream_repo.name.is_empty() {
        eprintln!("Invalid upstream repository.");
        std::process::exit(1);
    }

    let contributor_repo = match config::get_contributor_repo(&config, &upstream_repo) {
        Some(contributor_repo) => contributor_repo,
        None => {
            let contributor_repo = contributor_repo_init(&github, &upstream_repo).await;
            match config::set_contributor_repo(&mut config, &upstream_repo, contributor_repo.clone()) {
                Ok(_) => {
                    contributor_repo
                }
                Err(e) => {
                    eprintln!("Error setting contributor repo: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
    };
}
