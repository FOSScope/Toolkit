/**
  * Get the HTML source content from the given URL.
  */
async fn get_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::get(url).await?;
    if !client.status().is_success() {
        return Err("Failed to get the HTML content".into());
    }
    Ok(client.text().await?)
}

/**
 * Convert the HTML content from a given URL to Markdown.
 * The converted Markdown content will be trimmed.
 */
pub async fn html_to_markdown(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let html = get_html(url).await?;

    // We go through the html content line by line and trim the lines.
    // And then we convert the trimmed html content to markdown.
    let mut content = String::new();
    for line in html.lines() {
        content.push_str(line.trim());
        content.push('\n');
    }

    Ok(mdka::from_html(&content).trim().to_string())
}
