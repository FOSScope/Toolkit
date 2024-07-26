use std::io::Write;

use crate::models::GitHubRepo;

pub mod apis;
pub mod models;
pub mod workflow;
pub mod config;
mod utils;

/// Set the contributor repository of an upstream repository.
/// The information of the contributor repository will be stored in a JSON file.
/// The file will be kept in the directory across different runs.
///
/// # Arguments
/// - `repo`: The contributor repository.
/// - `upstream`: The pointer to the upstream repository.
pub fn set_contributor_repo(repo: GitHubRepo, upstream: &GitHubRepo) {
    println!("Setting the contributor repository to: {}", repo.get_full_name());

    let json_str = serde_json::to_string(&repo);
    match json_str {
        Ok(json_str) => {
            // Write to the runtime storage file.
            // If the directory does not exist, create it.
            std::fs::create_dir_all(".fosscope_toolkit").unwrap();
            let path = format!(
                ".fosscope_toolkit/contributor_repo_{}_{}.json",
                upstream.owner, upstream.name
            );
            let mut file = std::fs::File::create(path).unwrap();
            file.write_all(json_str.as_bytes()).unwrap();
        }
        Err(_) => {
            eprintln!("Failed to set the contributor repository.");
            std::process::exit(1);
        }
    }
}

/// Get the contributor repository of an upstream repository from the JSON file.
///
/// # Arguments
/// - `upstream`: The pointer to the upstream repository.
pub fn get_contributor_repo(upstream: &GitHubRepo) -> Option<GitHubRepo> {
    let path_str = format!(
        ".fosscope_toolkit/contributor_repo_{}_{}.json",
        upstream.owner, upstream.name
    );
    let path = std::path::Path::new(&path_str);
    if !path.exists() {
        return None;
    }
    let file = std::fs::File::open(path);
    match file {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            let contributor_repo: GitHubRepo = serde_json::from_reader(reader).unwrap();
            Some(contributor_repo)
        }
        Err(_) => None
    }
}
