use regex::Regex;

/// Get the HTML content from a given URL.
/// Only the content between `<body>` and `</body>` tags will be returned.
///
/// # Arguments
/// - `url`: The URL to get the HTML content from.
///
/// # Returns
/// - `Result<String, Box<dyn std::error::Error>>`: The result of getting the HTML content.
///     - `Ok(String)`: The HTML content between `<body>` and `</body>` tags.
///     - `Err(Box<dyn std::error::Error>)`: The error message indicating the failure to get the HTML content.
async fn get_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::get(url).await?;
    if !client.status().is_success() {
        return Err("Failed to get the HTML content.".into());
    }

    let text = client.text().await?;

    // Get only content between <body> and </body> tags
    let body_start_regex = Regex::new(r#"<body[^>]*>"#).unwrap();
    let body_start_tag = body_start_regex.find(&text).unwrap();
    let body_start = body_start_tag.end();
    let body_end = text.find("</body>").unwrap();

    Ok(text[body_start..body_end].to_string())
}

/// Convert a webpage's HTML content to Markdown.
/// Each line of the HTML content will be trimmed before converting to Markdown.
/// The converted Markdown content will be returned.
///
/// # Arguments
/// - `url`: The URL of the webpage to convert.
///
/// # Returns
/// - `Result<String, Box<dyn std::error::Error>>`: The result of converting the webpage content to Markdown.
///     - `Ok(String)`: The Markdown content converted from the HTML content.
///     - `Box<dyn std::error::Error>`: The error message indicating the failure to convert the HTML content to Markdown.
pub async fn html_to_markdown(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let html = get_html(url).await?;

    // We go through the html content line by line and trim the lines.
    // And then we convert the trimmed html content to markdown.
    let mut content = String::new();
    for line in html.lines() {
        content.push_str(line.trim());
        content.push('\n');
    }

    Ok(html2md::parse_html(&content).trim().to_string())
}
