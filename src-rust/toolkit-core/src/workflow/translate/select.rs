use std::collections::HashMap;
use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
use crate::models::html_filter_rule::HTMLFilterRule;
use crate::models::repo_rule::{Article, RepoRule};

pub async fn get_content(url: &str) -> Result<String, String> {
    let website = url::Url::parse(url);
    let website = match website {
        Ok(website) => website,
        Err(_) => {
            return Err("Failed to parse the URL.".to_string());
        }
    };
    let host = website.host_str();
    let host = if host.is_none() {
        return Err("Failed to get the host of the URL.".to_string());
    } else {
        host.unwrap()
    };

    let html_filter_rule = HTMLFilterRule::get_filter_rule(host);

    // Filter the HTML content
    let filtered_html = libhtmlfilter::get_filtered_html_fullurl_removeref(
        url,
        &*html_filter_rule.tags,
        &*html_filter_rule.classes
    ).await;

    Ok(html2md::parse_html(&filtered_html).trim().to_string())
}

pub async fn select_article(
    repo_rule: &RepoRule,
    article_type: &Article,
    vars: &HashMap<&str, &str>
) -> Result<String, String> {
    let mut local_vars = vars.clone();

    let url = local_vars.get("via").unwrap();

    let content = get_content(url).await;
    if content.is_err() {
        return content;
    }
    let content = content.unwrap();

    // The title is the only H1 header in the content
    let title = content.split("\n").find(|line| line.starts_with("# ")).unwrap();

    // The content is the content without the title
    let content = content.replace(title, "");

    // Add Info To Vars
    local_vars.insert("title", title);
    local_vars.insert("content", &content);

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

    Ok(
        handlebars.render_template(&*article_template, &local_vars).unwrap()
    )
}
