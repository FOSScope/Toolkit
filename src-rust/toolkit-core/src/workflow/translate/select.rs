use crate::models::html_filter_rule::HTMLFilterRule;

pub fn get_content(url: &str) -> Result<String, String> {
    let article_template = std::fs::read_to_string("rsc/translate_article_template.md");
    let article_template = match article_template {
        Ok(article_template) => article_template,
        Err(_) => {
            return Err("Failed to read the translate article template.".to_string());
        }
    };

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
    );

    // Parse HTML to markdown
    let md = html2md::parse_html(&filtered_html).trim().to_string();

    // Add header and footer
    let mut content = String::new();
    content.push_str(&article_template);
    content.push_str(&md);

    Ok(content)
}
