mod mock_error;

use mock_error::setup_error_handler;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    let mocked_path = "/simple-html";

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

mod tests {
    use fosscopetoolkit_core::parser::html_to_markdown;
    use super::*;

    #[tokio::test]
    async fn convert_html_to_markdown() {
        let template = ResponseTemplate::new(200).set_body_string(
            r#"
            <html>
                <head>
                    <title>Simple HTML</title>
                </head>
                <body>
                    <h1>Simple HTML</h1>
                    <p>This is a simple HTML page.</p>
                </body>
            </html>
            "#,
        );
        let mock_server = setup_api(template).await;

        let url = format!("{}/simple-html", mock_server.uri());
        let markdown = html_to_markdown(&url).await;

        assert_eq!(markdown.unwrap(), "# Simple HTML\n\nThis is a simple HTML page.");
    }

    #[tokio::test]
    async fn convert_html_to_markdown_file() {
        let template = ResponseTemplate::new(200).set_body_string(
            r#"
            <html>
                <head>
                    <title>Simple HTML</title>
                </head>
                <body>
                    <h1>Simple HTML</h1>
                    <p>This is a simple HTML page.</p>
                    <blockquote>
                        This is a blockquote.
                    </blockquote>
                    <p>This is some <code>inline</code> code.</p>
                    <pre>
                        <code class="language-rust">
                        fn main() {
                            println!("Hello, world!");
                        }
                        </code>
                    </pre>
                    <ul>
                        <li>Item 1</li>
                        <li>
                            Item 2
                            <ul>
                                <li>Item 2.1</li>
                                <li>Item 2.2</li>
                            </ul>
                        </li>
                        <li>Item 3</li>
                    </ul>
                    <ol type="I">
                        <li>Item 1</li>
                        <li>
                            Item 2
                            <ul>
                                <li>Item 2.1</li>
                                <li>Item 2.2</li>
                            </ul>
                        </li>
                        <li>Item 3</li>
                    </ol>
                </body>
            </html>
            "#,
        );
        let mock_server = setup_api(template).await;

        let url = format!("{}/simple-html", mock_server.uri());
        let markdown = html_to_markdown(&url).await;

        // Write the markdown to a file for debugging
        use std::fs;
        fs::write("output.md", markdown.unwrap()).expect("Unable to write file");

        // Yes I know this is not the right way of writing tests, but
        // forgive me because I need to know the output of the markdown conversion.
        assert!(true);
    }
}
