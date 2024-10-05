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
async fn setup_api(template: ResponseTemplate, mocked_path: &str) -> MockServer {
    let mock_server = MockServer::start().await;

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
    use fosscopetoolkit_core::apis::GitHubApi;
    use fosscopetoolkit_core::models::{github_api_responses, GitHubRepo};
    use super::*;

    #[tokio::test]
    async fn get_latest_commit_sha() {
        let mocked_response: Vec<github_api_responses::commit::Commit> =
            serde_json::from_str(include_str!("resources/commits.json")).unwrap();
        let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
        let mock_server = setup_api(template, "/repos/octocat/Hello-World/commits").await;
        let client = setup_octocrab(&mock_server.uri());

        let github = GitHubApi::new("octocat".to_string(), client);
        let result = github.get_latest_commit_sha(
            &GitHubRepo {
                owner: "octocat".to_string(),
                name: "Hello-World".to_string(),
            },
        ).await.unwrap();

        assert_eq!(result, "7fd1a60b01f91b314f59955a4e4d4e80d8edf11d");
    }

    #[tokio::test]
    async fn get_file_content() {
        let mocked_response: github_api_responses::repository_content::RepositoryContent =
            serde_json::from_str(include_str!("resources/repo_content_file.json")).unwrap();
        let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
        let mock_server = setup_api(template, "/repos/octocat/Hello-World/contents/README.md").await;
        let client = setup_octocrab(&mock_server.uri());

        let github = GitHubApi::new("octocat".to_string(), client);
        let result = github.get_contents(
            &GitHubRepo {
                owner: "octocat".to_string(),
                name: "Hello-World".to_string(),
            },
            "README.md"
        ).await.unwrap();

        match result {
            github_api_responses::repository_content::RepositoryContent::File(content) => {
                assert_eq!(content.encoding, Some("base64".to_string()));
                assert_eq!(content.path, "README.md");
                assert_eq!(content.decoded_content().unwrap().trim(), "Hello World!");
            }
            github_api_responses::repository_content::RepositoryContent::Dir(_) => {
                panic!("Expected file, got directory.");
            }
        }
    }

    #[tokio::test]
    async fn get_dir_content() {
        let mocked_response: github_api_responses::repository_content::RepositoryContent =
            serde_json::from_str(include_str!("resources/repo_content_dir.json")).unwrap();
        let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
        let mock_server = setup_api(template, "/repos/octocat/Hello-World/contents/src").await;
        let client = setup_octocrab(&mock_server.uri());

        let github = GitHubApi::new("octocat".to_string(), client);
        let result = github.get_contents(
            &GitHubRepo {
                owner: "octocat".to_string(),
                name: "Hello-World".to_string(),
            },
            "src"
        ).await.unwrap();

        match result {
            github_api_responses::repository_content::RepositoryContent::File(_) => {
                panic!("Expected directory, got file.");
            }
            github_api_responses::repository_content::RepositoryContent::Dir(content) => {
                let entries = content.entries.unwrap();
                assert_eq!(entries.len(), 2);

                let entry1 = &entries[0];
                match entry1 {
                    github_api_responses::repository_content::RepositoryContent::File(content) => {
                        assert_eq!(content.size, 625);
                        assert_eq!(content.name, "app.js");
                    }
                    github_api_responses::repository_content::RepositoryContent::Dir(_) => {
                        panic!("Expected file, got directory.");
                    }
                }

                let entry2 = &entries[1];
                match entry2 {
                    github_api_responses::repository_content::RepositoryContent::File(_) => {
                        panic!("Expected directory, got file.");
                    }
                    github_api_responses::repository_content::RepositoryContent::Dir(content) => {
                        assert_eq!(content.path, "src/images");
                        assert_eq!(content.sha, "a84d88e7554fc1fa21bcbc4efae3c782a70d2b9d");
                    }
                }
            }
        }
    }
}
