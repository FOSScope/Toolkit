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

    let html_filter_rule_path = format!("rsc/html_filter_rules/{}.toml", host);
    let html_filter_rule_str = std::fs::read_to_string(html_filter_rule_path);
    let html_filter_rule_str = match html_filter_rule_str {
        Ok(html_filter_rule_str) => html_filter_rule_str,
        Err(_) => {
            let error_msg = format!(
                "Failed to read the HTML filter rule for the website: {}", host
            );
            return Err(error_msg);
        }
    };

    let html_filter_rule: Result<HTMLFilterRule, _> = toml::from_str(&html_filter_rule_str);
    let html_filter_rule = match html_filter_rule {
        Ok(html_filter_rule) => html_filter_rule,
        Err(_) => {
            let error_msg = format!(
                "Failed to parse the HTML filter rule for the website: {}", host
            );
            return Err(error_msg);
        }
    };

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
    content.push_str(&article_template);
    content.push_str(&md);

    Ok(content)
}
