use std::collections::HashMap;
use std::fs;
use std::io::{stdin, stdout, Write};

use fosscopetoolkit_core::apis::GitHubApi;
use fosscopetoolkit_core::config::config::get_config;
use fosscopetoolkit_core::models::GitHubRepo;
use fosscopetoolkit_core::models::repo_rule::get_repo_rule;
use fosscopetoolkit_core::workflow;

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

    // Ask the user to enter the original publishing date of the article.
    // TODO: Automatically fetch the original publishing date from the article, if possible.
    print!("请输入文章的原始发布日期 (格式：YYYYMMDD（例如：20240715)): ");
    let _ = stdout().flush();
    let mut publishing_date = String::new();
    stdin().read_line(&mut publishing_date).unwrap_or(0);
    let publishing_date = publishing_date.trim();
    println!("您输入的文章发布日期是：{}", publishing_date);

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
    vars.insert("type_name", &article_type.r#type);
    vars.insert("type_desc", &article_type.description);

    // Get the article content in Markdown format, rendered using the data in the variables.
    let article = workflow::translate::select::fetch(
        &repo_rule, article_type, &vars
    ).await;
    if article.is_err() {
        eprintln!("Failed to select article: {:?}", article.err());
        return;
    }
    let article = article.unwrap();

    let title = article.1;
    let content = article.0;
    vars.insert("article_title", &title);

    // The article ID is the original publishing date, dash (`-`), followed by the
    // title in all lowercase, with spaces replaced by dashes (`-`), and with all non-alphanumeric
    // characters removed.
    let article_id = format!(
        "{}-{}",
        publishing_date,
        title.to_lowercase().replace(" ", "-").chars().filter(
            |c| c.is_alphanumeric() || *c == '-'
        ).collect::<String>()
    );
    vars.insert("article_id", &article_id);

    let file_name = format!("{}.md", article_id);
    // Write the article content to a file.
    fs::write(&file_name, content).expect("无法写入文件");
    println!("已将 Markdown 内容写入文件：{}", &file_name);

    // Open the file in the user's text editor for them to edit the article if needed.
    let path_to_file = std::env::current_dir().unwrap().join(&file_name);
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

    // Get the content of the file after the user has edited it.
    let content = fs::read_to_string(&file_name).expect("无法读取文章源文件");

    // Commit the article to the user's fork of the Translation Project repository.
    let commit = workflow::translate::select::submit::commit(
        github, contributor_repo, &file_name, &content, article_type, &repo_rule, &vars
    ).await;
    if commit.is_err() {
        eprintln!("Failed to commit article: {:?}", commit.err());
        return;
    }
    println!("文章已提交到您的分支。");

    // TODO: Follow up process of creating a pull request.
}
