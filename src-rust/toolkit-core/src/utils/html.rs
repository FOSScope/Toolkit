use reqwest::get;

/// An error that can occur when getting the HTML content of a website.
pub enum HtmlGetError {
    FetchError(String),
    TextError(String),
}

/// Get the HTML content of a website.
///
/// # Arguments
/// - `url`: The URL of the website.
///
/// # Returns
/// - `Result<String, HtmlGetError>`:
///     - `Ok(String)`: The HTML content of the website.
///     - `Err(HtmlGetError)`: The error that occurred when getting the HTML content.
pub async fn get_html_content(url: &str) -> Result<String, HtmlGetError> {
    let response = match get(url).await {
        Ok(response) => response,
        Err(e) => return Err(HtmlGetError::FetchError(e.to_string())),
    };
    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => return Err(HtmlGetError::TextError(e.to_string())),
    };
    Ok(body)
}
