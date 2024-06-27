use std::io::{stdin, stdout, Write};
use confique::Config;

use fosscopetoolkit_core::parser::html_to_markdown;
use crate::config::AppConfig;

/**
 * Workflow for selecting new articles to translate.
 */
pub async fn select() {
    let config = AppConfig::builder()
        // Used to get the GitHub username and token while developing and testing the CLI
        .file("config/cli.dev.toml")
        // The proper config when in production
        .file("config/config.toml")
        .load()
        .expect("Failed to load the configuration file.");

    println!("欢迎参与开源观察翻译项目！");
    print!("请输入要选题的文章的 URL：");
    let _= stdout().flush();
    let mut url = String::new();
    stdin().read_line(&mut url).unwrap_or(0);
    let url = url.trim();
    println!("您选择的文章 URL 是：{}", url);

    // Convert to Markdown
    let markdown = html_to_markdown(url).await;

    match markdown {
        Ok(markdown) => {
            println!("已成功转换为 Markdown");

            // TODO: Be able to allow the user to set the file name
            let file_name = "article.md";
            std::fs::write(file_name, markdown).expect("无法写入文件");
            println!("已将 Markdown 内容写入文件：{}", file_name);

            let path_to_file = std::env::current_dir().unwrap().join(file_name);
            let editor = &config.editor;
            let _= std::process::Command::new(editor)
                .arg(path_to_file)
                .status()
                .expect("无法打开编辑器");
        }
        Err(e) => {
            eprintln!("Markdown 转换失败：{}", e);
        }
    }
}
