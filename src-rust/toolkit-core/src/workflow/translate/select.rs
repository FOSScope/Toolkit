use crate::models::article_template::ArticleTemplate;
use crate::models::html_filter_rule::HTMLFilterRule;

pub fn get_content(
    url: &str,
    html_filter_rule: HTMLFilterRule,
    article_template: &ArticleTemplate,
) -> String {
    // Filter the HTML content
    let filtered_html = libhtmlfilter::get_filtered_html_fullurl_removeref(
        url,
        html_filter_rule.tags.iter().map(|s| s.as_str()).collect::<Vec<&str>>().as_slice(),
        html_filter_rule.classes.iter().map(|s| s.as_str()).collect::<Vec<&str>>().as_slice()
    );

    // Parse HTML to markdown
    let md = html2md::parse_html(&filtered_html).trim().to_string();

    // Add header and footer
    let mut content = String::new();
    if let Some(header) = &article_template.header {
        content.push_str(header);
    }
    content.push_str(&md);
    if let Some(footer) = &article_template.footer {
        content.push_str(footer);
    }

    content
}
