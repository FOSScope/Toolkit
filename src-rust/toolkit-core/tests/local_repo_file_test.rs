#[cfg(test)]
mod tests {
    use fosscopetoolkit_core::{get_contributor_repo, set_contributor_repo};
    use fosscopetoolkit_core::models::GitHubRepo;

    #[test]
    fn read_and_write_repo() {
        let repo = GitHubRepo::new("octocat".to_string(), "Hello-World".to_string());
        set_contributor_repo(repo.clone());
        let contributor_repo = get_contributor_repo().unwrap();
        assert_eq!(contributor_repo, repo);
    }
}
