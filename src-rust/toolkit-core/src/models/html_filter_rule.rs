use crate::models::websites::ItsFoss;

/// A representation of the set of rules to filter the HTML content of a website.
///
/// # Fields
/// - `tags`: The list of tags to filter out from the HTML content.
/// - `classes`: The list of classes to filter out from the HTML content.
pub struct HTMLFilterRule<'a> {
    pub tags: Vec<&'a str>,
    pub classes: Vec<&'a str>,
}

impl<'a> HTMLFilterRule<'a> {
    pub fn new(tags: Vec<&'a str>, classes: Vec<&'a str>) -> Self {
        Self {
            tags,
            classes,
        }
    }

    /// Get the filter rule for the given URL.
    ///
    /// # Current Supported Websites:
    /// - `itsfoss.com` (and its news subdomain `news.itsfoss.com`)
    pub fn get_filter_rule(url: &str) -> Self {
        match url {
            "itsfoss.com" | "news.itsfoss.com" => {
                ItsFoss::get_filter_rule()
            }
            _ => Self::new(vec![], vec![])
        }
    }
}
