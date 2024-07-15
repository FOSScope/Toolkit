use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use tokio::runtime::Runtime;
use fosscopetoolkit_core::apis::github_api::GitHubApi;
use fosscopetoolkit_core::models::github_repo::GitHubRepo;
use fosscopetoolkit_core::models::repo_rule::get_repo_rule;
use fosscopetoolkit_core::workflow::translate::select::select_article;
use crate::config::get_config;

/**
 * Workflow for selecting new articles to translate.
 */
pub async fn select(
    contributor_repo: &GitHubRepo,
    github: &GitHubApi,
) {
    let user = github.get_user().await;
    let user = match user {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Failed to get GitHub user: {:?}", e);
            return;
        }
    };

    let repo_rule = get_repo_rule(contributor_repo, github).await;

    if repo_rule.is_err() {
        eprintln!("Failed to get repository rule: {:?}", repo_rule.err());
        return;
    }
    let repo_rule = repo_rule.unwrap();

    let config = get_config();

    println!("欢迎参与开源观察翻译项目！");
    print!("请输入要选题的文章的 URL：");
    let _ = stdout().flush();
    let mut url = String::new();
    stdin().read_line(&mut url).unwrap_or(0);
    let url = url.trim();
    println!("您选择的文章 URL 是：{}", url);

    // Get the type of the article
    let article_types = &repo_rule.articles;
    println!("请选择文章类型：");
    for (i, article_type) in article_types.iter().enumerate() {
        println!("{}. {}", i + 1, article_type.description);
    }
    print!("请输入文章类型的编号：");
    let _ = stdout().flush();
    let mut article_type_index = String::new();
    stdin().read_line(&mut article_type_index).unwrap_or(0);
    let article_type_index = article_type_index.trim().parse::<usize>().unwrap_or(0);
    let article_type = &article_types[article_type_index - 1];
    println!("您选择的文章类型是：{}", article_type.description);

    let mut vars = HashMap::new();
    vars.insert("via", url);
    vars.insert("selector", &user);

    let article = select_article(&repo_rule, article_type, &vars);
    if article.is_err() {
        eprintln!("Failed to select article: {:?}", article.err());
        return;
    }
    let article = article.unwrap();

    // TODO: Be able to allow the user to set the file name
    let file_name = "article.md";
    std::fs::write(file_name, article).expect("无法写入文件");
    println!("已将 Markdown 内容写入文件：{}", file_name);

    let path_to_file = std::env::current_dir().unwrap().join(file_name);
    let editor = &config.editor;
    let _ = std::process::Command::new(editor)
        .arg(path_to_file)
        .status()
        .expect("无法打开编辑器");
    // We would not be able to detect the program's exit status if it is
    // a program that only uses the terminal to launch the editor.
    // e.g. Visual Studio Code.

    println!("如果您已经完成了编辑，请输入任何内容以继续。");
    let _ = stdin().read_line(&mut String::new());
}
