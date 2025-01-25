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

    let mocked_path = "/repos/octocat/Hello-World";

    Mock::given(method("GET"))
        .and(path(mocked_path))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("GET on {mocked_path} was not received"),
    )
    .await;

    mock_server
}

// This function sets up the Octocrab client with the base URI of the mock server.
fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[cfg(test)]
mod tests {
    use octocrab::models::Repository;

    use fosscopetoolkit_core::apis::GitHubApi;
    use fosscopetoolkit_core::models::GitHubRepo;

    use super::*;

    #[tokio::test]
    async fn non_existent_repo_validation() {
        let mocked_response: Repository =
            serde_json::from_str(include_str!("resources/forked_repo.json")).unwrap();
        let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
        let mock_server = setup_api(template).await;
        let client = setup_octocrab(&mock_server.uri());

        let github = GitHubApi::new("octocat".to_string(), client);
        let result = github
            .validate_repo(
                GitHubRepo {
                    owner: "someone".to_string(),
                    name: "Hello-World".to_string(),
                },
                GitHubRepo {
                    owner: "upstream".to_string(),
                    name: "Hello-World".to_string(),
                },
            )
            .await;

        assert_eq!(result, Err("Repository does not exist"));
    }

    #[tokio::test]
    async fn not_forked_repo_validation() {
        let mocked_response: Repository =
            serde_json::from_str(include_str!("resources/not_forked_repo.json")).unwrap();
        let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
        let mock_server = setup_api(template).await;
        let client = setup_octocrab(&mock_server.uri());

        let github = GitHubApi::new("octocat".to_string(), client);
        let result = github
            .validate_repo(
                GitHubRepo {
                    owner: "octocat".to_string(),
                    name: "Hello-World".to_string(),
                },
                GitHubRepo {
                    owner: "upstream".to_string(),
                    name: "Hello-World".to_string(),
                },
            )
            .await;

        assert_eq!(result, Err("Repository is not a fork"));
    }

    #[tokio::test]
    async fn wrongly_forked_repo_validation() {
        let mocked_response: Repository =
            serde_json::from_str(include_str!("resources/wrongly_forked_repo.json")).unwrap();
        let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
        let mock_server = setup_api(template).await;
        let client = setup_octocrab(&mock_server.uri());

        let github = GitHubApi::new("octocat".to_string(), client);
        let result = github
            .validate_repo(
                GitHubRepo {
                    owner: "octocat".to_string(),
                    name: "Hello-World".to_string(),
                },
                GitHubRepo {
                    owner: "upstream".to_string(),
                    name: "Hello-World".to_string(),
                },
            )
            .await;

        assert_eq!(
            result,
            Err("Repository is not a fork of the upstream repository")
        );
    }

    #[tokio::test]
    async fn correctly_forked_repo_validation() {
        let mocked_response: Repository =
            serde_json::from_str(include_str!("resources/forked_repo.json")).unwrap();
        let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
        let mock_server = setup_api(template).await;
        let client = setup_octocrab(&mock_server.uri());

        let github = GitHubApi::new("octocat".to_string(), client);
        let result = github
            .validate_repo(
                GitHubRepo {
                    owner: "octocat".to_string(),
                    name: "Hello-World".to_string(),
                },
                GitHubRepo {
                    owner: "upstream".to_string(),
                    name: "Hello-World".to_string(),
                },
            )
            .await;

        assert_eq!(result, Ok(()));
    }
}
