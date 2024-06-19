mod mock_error;

use mock_error::setup_error_handler;
use fosscopetoolkit_core::apis::GitHubApi;
use octocrab::models::{UserProfile};
use octocrab::{Octocrab};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

// This is a fake page that we can use to deserialize the response from the GitHub API.
#[derive(Serialize, Deserialize)]
struct FakePage<T> {
    items: Vec<T>,
}

// This function sets up the mock server with the given template (response).
async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    let mocked_path = "/user";

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
    use super::*;

    #[tokio::test]
    async fn user_verify_successful() {
        let mocked_response: UserProfile =
            serde_json::from_str(include_str!("resources/user.json")).unwrap();
        let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
        let mock_server = setup_api(template).await;
        let client = setup_octocrab(&mock_server.uri());
        // let client = Octocrab::builder().personal_token(token).build().unwrap();

        let github = GitHubApi::new("octocat".to_string(), client);
        // The username and the username from the mocked response should match.
        assert!(github.verify_user().await);
    }

    #[tokio::test]
    async fn user_verify_unsuccessful() {
        let mocked_response: UserProfile =
            serde_json::from_str(include_str!("resources/user.json")).unwrap();
        let template = ResponseTemplate::new(200).set_body_json(&mocked_response);
        let mock_server = setup_api(template).await;
        let client = setup_octocrab(&mock_server.uri());

        let github = GitHubApi::new("random-user-123".to_string(), client);
        // The username and the username from the mocked response should NOT match.
        assert!(!github.verify_user().await);
    }
}
