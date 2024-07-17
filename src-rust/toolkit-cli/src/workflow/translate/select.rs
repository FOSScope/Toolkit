use std::collections::HashMap;
use std::fs;
use std::io::{stdin, stdout, Write};

use fosscopetoolkit_core::apis::GitHubApi;
use fosscopetoolkit_core::config::config::get_config;
use fosscopetoolkit_core::models::GitHubRepo;
use fosscopetoolkit_core::models::repo_rule::get_repo_rule;
use fosscopetoolkit_core::workflow::translate::select::select_article;

/// Select an article to translate.
///
/// # Arguments
/// - `contributor_repo`: The repository that the user is contributing to.
/// - `github`: A pointer to the GitHub API wrapper.
pub async fn select(
    contributor_repo: &GitHubRepo,
    github: &GitHubApi,
) {
    // Get the username of the currently signed in GitHub user.
    let user = github.get_user().await;
    let user = match user {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Failed to get GitHub user: {:?}", e);
            return;
        }
    };

    // Fetch the Translation Project repository's rule.
    let repo_rule = get_repo_rule(contributor_repo, github).await;

    if repo_rule.is_err() {
        eprintln!("Failed to get repository rule: {:?}", repo_rule.err());
        return;
    }
    let repo_rule = repo_rule.unwrap();

    // Get the configuration from the configuration file.
    let config = get_config();

    // Ask the user to input the URL of the article they want to select.
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

    // Add information to the variables to be used in the Handlebars template rendering.
    let mut vars = HashMap::new();
    vars.insert("via", url); // The URL of the article
    vars.insert("selector", &user); // The username of the currently signed in GitHub user (as the article selector)

    // Get the article content in Markdown format, rendered using the data in the variables.
    let article = select_article(&repo_rule, article_type, &vars).await;
    if article.is_err() {
        eprintln!("Failed to select article: {:?}", article.err());
        return;
    }
    let article = article.unwrap();

    // Write the article content to a file.
    // TODO: Be able to change the file name according to the repository rule.
    let file_name = "article.md";
    fs::write(file_name, article).expect("无法写入文件");
    println!("已将 Markdown 内容写入文件：{}", file_name);

    // Open the file in the user's text editor for them to edit the article if needed.
    let path_to_file = std::env::current_dir().unwrap().join(file_name);
    let editor = &config.editor;
    let _ = std::process::Command::new(editor)
        .arg(path_to_file)
        .status()
        .expect("无法打开编辑器");
    // We would not be able to detect the program's exit status if it is
    // a program that only uses the terminal to launch the editor.
    // e.g. Visual Studio Code.
    // So the following message is displayed to the user to continue the process.
    println!("如果您已经完成了编辑，请输入任何内容以继续。");
    let _ = stdin().read_line(&mut String::new());

    // TODO: Follow up process like commiting the changes and creating a pull request.
}
