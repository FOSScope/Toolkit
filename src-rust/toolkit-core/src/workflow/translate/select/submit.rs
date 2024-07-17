use std::collections::HashMap;

use handlebars::Handlebars;

use crate::apis::GitHubApi;
use crate::models::{GitHubRepo, RepoRule};
use crate::models::repo_rule::Article;

pub async fn commit(
    github: &GitHubApi,
    contributor_repo: &GitHubRepo,
    file_name: &str,
    content: &str,
    article_type: &Article,
    repo_rule: &RepoRule,
    vars: &HashMap<&str, &str>,
) -> Result<(), String> {
    let mut local_vars = vars.clone();
    local_vars.insert("action_name", "select");
    local_vars.insert("action_desc", "选题");
    local_vars.insert("step", "sources");

    let handlebars = Handlebars::new();
    let branch_name = handlebars.render_template(&*repo_rule.git.branch_naming, &local_vars).unwrap();
    let commit_message = handlebars.render_template(&*repo_rule.git.commit_message, &local_vars).unwrap();
    let path = format!(
        "{}/{}",
        handlebars.render_template(&*article_type.directory, &local_vars).unwrap(),
        file_name,
    );

    let r = github.create_new_reference(contributor_repo, &branch_name).await;
    if r.is_err() {
        return Err(format!("Failed to create a new branch: {:?}", r.err()));
    }

    let r = github.create_file(
        contributor_repo, &branch_name, &path, content, &commit_message
    ).await;
    if r.is_err() {
        return Err(r.err().unwrap());
    }

    Ok(())
}

pub async fn create_pr(
    github: &GitHubApi,
    upstream_repo: &GitHubRepo,
    contributor_repo: &GitHubRepo,
    repo_rule: &RepoRule,
    vars: &HashMap<&str, &str>,
) -> Result<(), String> {
    let mut local_vars = vars.clone();
    local_vars.insert("action_name", "select");
    local_vars.insert("action_desc", "选题");
    local_vars.insert("step", "sources");

    let handlebars = Handlebars::new();
    let branch_name = handlebars.render_template(&*repo_rule.git.branch_naming, &local_vars).unwrap();
    let pr_title = handlebars.render_template(&*repo_rule.git.commit_message, &local_vars).unwrap();

    let r = github.create_pull_request(
        upstream_repo,
        contributor_repo,
        &pr_title,
        &branch_name,
        &repo_rule.git.base,
    ).await;
    if r.is_err() {
        return Err(r.err().unwrap());
    }

    Ok(())
}
