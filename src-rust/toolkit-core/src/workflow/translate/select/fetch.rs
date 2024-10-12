use std::collections::HashMap;

use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};

use crate::models::{article_metadata, HTMLFilterRule};
use crate::models::repo_rule::Article;
use crate::models::RepoRule;
use crate::models::SupportedWebsites;
use crate::utils;

/// Get the content of an HTML page in Markdown format.
///
/// The HTML content is filtered first based on the host of the URL using [`HTMLFilterRule`](struct.HTMLFilterRule.html)
/// before being converted to Markdown format.
///
/// # Arguments
/// - `url`: The URL of the HTML page to get the content from.
///
/// # Returns
/// - `Result<(String, String), String>`:
///     - `Ok(String, String)`: The content of the HTML page in Markdown format (first element) and the title of the page (second element).
///     - `Err(String)`: An error message indicating why the content could not be fetched.
pub async fn get_content(url: &str) -> Result<(String, String), String> {
    let website = SupportedWebsites::get_website(url)?;

    let html_filter_rule = HTMLFilterRule::get_filter_rule(&website);

    // Filter the HTML content
    let filtered_html = libhtmlfilter::get_filtered_html_fullurl_removeref(
        url,
        &*html_filter_rule.tags,
        &*html_filter_rule.classes,
    ).await;

    // Convert the filtered HTML content to Markdown format
    let content = html2md::parse_html(&filtered_html).trim().to_string();

    // The title is the only H1 header in the content, with the `# ` prefix removed.
    let title = &content.split("\n").find(
        |line| line.starts_with("# ")
    ).unwrap()[2..];

    // The content is the content without the title
    let content = content.replace(title, "");

    Ok((content, title.to_string()))
}

/// Fetch the content of an article to translate.
///
/// # Arguments
/// - `repo_rule`: The rule of the repository to contribute to.
/// - `article_type`: The type of the article to translate.
/// - `vars`: The variables to use when rendering the Handlebars template.
///
/// # Returns
/// - `Result<(String, String), String>`:
///     - `Ok((String, String))`: The content of the article, in Markdown format, rendered with provided variables (first element) and the title of the article (second element).
///     - `Err(String)`: An error message if the content could not be fetched.
pub async fn fetch(
    repo_rule: &RepoRule,
    article_type: &Article,
    vars: &HashMap<&str, String>,
) -> Result<(String, String), String> {
    let url = vars.get("via").unwrap();

    let mut local_vars = vars.clone();

    let html = utils::html::get_html_content(url).await?;

    let content = get_content(url).await;
    if content.is_err() {
        return content
    }
    let content = content.unwrap();

    // Add Info To Vars
    local_vars.insert("title", content.1.clone());
    local_vars.insert("content", content.0);

    // Add Fetchable Metadata
    let website = SupportedWebsites::get_website(url)?;
    // Author
    let author = article_metadata::AuthorMetadata::get(&website, &html).await;
    if author.is_some() {
        let author = author.unwrap();
        local_vars.insert("author", author.name);
        local_vars.insert("author_link", author.link);
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

    let rendered = handlebars.render_template(&*article_template, &local_vars).unwrap();

    Ok((rendered, content.1))
}
