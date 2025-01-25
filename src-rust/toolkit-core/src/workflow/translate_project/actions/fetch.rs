use crate::models::repo_rule::Article;
use crate::models::supported_website::get_supported_website;
use crate::models::{supported_website, HTMLFilterRule, RepoRule};
use crate::utils;
use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
use std::collections::HashMap;

/// Possible errors that can occur when fetching the content of the selected article.
pub enum ArticleContentFetchError {
    HtmlFetchError(utils::html::HtmlGetError),
    WebsiteGettingError(supported_website::WebsiteGettingError),
}

/// Get the content of an HTML page in Markdown format.
///
/// The HTML content is filtered first based on the host of the URL using [`HTMLFilterRule`](struct.HTMLFilterRule.html)
/// before being converted to Markdown format.
///
/// # Arguments
/// - `url`: The URL of the HTML page to get the content from.
///
/// # Returns
/// - Result<(String, String), MarkdownContentGettingError>:
///     - `Ok(String, String)`: The content of the HTML page in Markdown format (first element) and the title of the page (second element).
///     - `Err(MarkdownContentGettingError)`: The error that occurred when getting the content of the HTML page.
async fn get_markdown_content(url: &str, html_filter_rule: HTMLFilterRule) -> (String, String) {
    // Filter the HTML content
    let filtered_html = libhtmlfilter::get_filtered_html_fullurl_removeref(
        url,
        &*html_filter_rule.tags,
        &*html_filter_rule.classes,
    )
    .await;

    // Convert the filtered HTML content to Markdown format
    let content = html2md::parse_html(&filtered_html).trim().to_string();

    // The title is the only H1 header in the content, with the `# ` prefix removed.
    let title = &content
        .split("\n")
        .find(|line| line.starts_with("# "))
        .unwrap();
    // Remove the line from the content
    let content = content.replace(title, "");
    // Remove the `# ` prefix from the title
    let title = title.replace("# ", "");

    (content, title.to_string())
}

/// Fetch the content of an article to translate.
///
/// # Arguments
/// - `url`: The URL of the article to fetch.
/// - `repo_rule`: The repository rule of the repository to contribute to.
/// - `article_type`: The type of the article to fetch.
///
/// # Returns
/// - Result<(String, String), ArticleContentFetchError>:
///     - `Ok((String, String))`: The content of the article, in Markdown format, rendered with provided variables (first element) and the title of the article (second element).
///     - `Err(ArticleContentFetchError)`: The error that occurred when fetching the content of the article.
pub async fn fetch(
    url: String,
    user: String,
    repo_rule: &RepoRule,
    article_type: &Article,
) -> Result<(String, String), ArticleContentFetchError> {
    let website = match get_supported_website(url.as_str()) {
        Ok(website) => website,
        Err(e) => return Err(ArticleContentFetchError::WebsiteGettingError(e)),
    };
    let html_filter_rule = HTMLFilterRule::get_filter_rule(&website);

    let html_content = match utils::html::get_html_content(url.as_str()).await {
        Ok(html) => html,
        Err(e) => return Err(ArticleContentFetchError::HtmlFetchError(e)),
    };
    let markdown_content = get_markdown_content(url.as_str(), html_filter_rule).await;

    // Add information to the variables to be used in the Handlebars template rendering.
    let mut vars = HashMap::new();
    vars.insert("via", url.clone()); // The URL of the article
    vars.insert("selector", user.clone()); // The username of the currently signed in GitHub user (as the article selector)
    vars.insert("type_name", article_type.r#type.clone());
    vars.insert("type_desc", article_type.description.clone());
    vars.insert("title", markdown_content.1.clone());
    vars.insert("content", markdown_content.0.clone());
    let author = website.get_author_metadata(&html_content);
    if author.is_some() {
        let author = author.unwrap();
        vars.insert("author", author.name);
        vars.insert("author_link", author.link);
    }

    // Get Article Template
    let article_template = repo_rule.get_article_template(article_type);

    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(false);
    handlebars.register_helper(
        "helperMissing",
        Box::new(
            |h: &Helper<'_>,
             _: &Handlebars<'_>,
             _: &Context,
             _: &mut RenderContext<'_, '_>,
             out: &mut dyn Output|
             -> Result<(), RenderError> {
                out.write(&format!("{{{{{}}}}}", h.name()))?;
                Ok(())
            },
        ),
    );
    handlebars.register_escape_fn(handlebars::no_escape);

    let rendered = handlebars
        .render_template(&*article_template, &vars)
        .unwrap();

    Ok((rendered, markdown_content.1))
}
