use reqwest::get;

/// Get the HTML content of a website.
///
/// # Arguments
/// - `url`: The URL of the website.
///
/// # Returns
/// - `Result<String, reqwest::Error>`:
///     - `Ok(String)`: The HTML content of the website.
///    - `Err(reqwest::Error)`: An error indicating why the content could not be fetched.
pub async fn get_html_content(url: &str) -> Result<String, String> {
    let response = get(url).await;
    let response = match response {
        Ok(response) => response,
        Err(e) => return Err(format!("Failed to fetch the content of the website: {:?}", e)),
    };

    let body = response.text().await;
    let body = match body {
        Ok(body) => body,
        Err(e) => return Err(format!("Failed to get the text content of the website: {:?}", e)),
    };

    Ok(body)
}
