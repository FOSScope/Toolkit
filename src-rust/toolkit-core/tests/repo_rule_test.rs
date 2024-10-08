use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use mock_error::setup_error_handler;

mod mock_error;

// This is a fake page that we can use to deserialize the response from the GitHub API.
#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

// This function sets up the mock server with the given template (response).
async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    let mocked_path = "/repos/FOSScope/TranslateProject/contents/REPORULE";

    Mock::given(method("GET"))
        .and(path(mocked_path))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on {mocked_path} was not received"),
    ).await;

    mock_server
}

// This function sets up the Octocrab client with the base URI of the mock server.
fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[cfg(test)]
mod tests {
    use octocrab::models::repos::Content;

    use fosscopetoolkit_core::apis::GitHubApi;
    use fosscopetoolkit_core::models::GitHubRepo;
    use fosscopetoolkit_core::models::repo_rule::{Article, get_repo_rule, GitRule};
    use fosscopetoolkit_core::models::RepoRule;

    use super::*;

    #[tokio::test]
    async fn get_rule() {
        let mocked_response: Content =
            serde_json::from_str(include_str!("resources/repo_rule_content.json")).unwrap();
        let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
        let mock_server = setup_api(template).await;
        let client = setup_octocrab(&mock_server.uri());

        let github = GitHubApi::new("octocat".to_string(), client);
        let rule = get_repo_rule(
            &GitHubRepo {
                owner: "FOSScope".to_string(),
                name: "TranslateProject".to_string(),
            },
            &github,
        ).await;
        assert!(rule.is_ok());

        let template: String = r#"---
title: {{title}}
date: {{release_date}}
author:
  - fosscope-translation-team
  - {{translator}}
  - {{proofreader}}
banner: {{cover_image}}
cover: {{cover_image}}
categories:
  - 翻译
  - {{category}}
tags: {{tags}}
authorInfo: |
  via: {{via}}

  作者：[{{author}}]({{author_link}})
  选题：[{{selector}}](https://github.com/{{selector}})
  译者：[{{translator}}](https://github.com/{{translator}})
  校对：[{{proofreader}}](https://github.com/{{proofreader}})

  本文由 [FOSScope翻译组](https://github.com/FOSScope/TranslateProject) 原创编译，[开源观察](https://fosscope.com/) 荣誉推出
---

<!-- 所有以 `{{variable}}` 形式展现的内容都需要替换为实际内容 -->

{{summary}}

<!-- more -->

{{content}}
"#.to_string();

        let news: Article = Article::new("news".to_string(), "新闻".to_string(), "{{step}}/news".to_string(), None);
        let tech: Article = Article::new("tech".to_string(), "技术".to_string(), "{{step}}/tech".to_string(), None);
        let talk: Article = Article::new("talk".to_string(), "评论".to_string(), "{{step}}/talk".to_string(), None);

        let git_rule: GitRule = GitRule::new("main".to_string(), "{{action_name}}/{{type_name}}/{{article_id}}".to_string(), "[{{action_desc}}][{{type_desc}}] {{article_title}}".to_string());

        let expected = RepoRule::new(template, vec![news, tech, talk], git_rule);

        assert_eq!(rule.unwrap(), expected);
    }
}
