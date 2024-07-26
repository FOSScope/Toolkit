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
pub async fn get_html_content(url: &str) -> Result<String, reqwest::Error> {
    let response = get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
