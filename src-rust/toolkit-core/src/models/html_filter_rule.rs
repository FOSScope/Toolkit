use crate::models::SupportedWebsite;

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
    fn new(tags: Vec<&'a str>, classes: Vec<&'a str>) -> Self {
        Self {
            tags,
            classes,
        }
    }

    /// Get the filter rule for the given URL.
    ///
    /// # Current Supported Websites:
    /// - `itsfoss.com` (and its news subdomain `news.itsfoss.com`)
    pub fn get_filter_rule(website: &SupportedWebsite) -> Self {
        match website {
            SupportedWebsite::Itsfoss(_, _) | SupportedWebsite::NewsItsfoss(_, _) => {
                Self::new(
                    vec![
                        "script", "style", "link", "meta", "desc", "title", "svg", "path",
                        "dialog", "select", "head", "header", "foot", "footer", "nav", "button",
                        "form", "input", "picture", "time", "aside",
                        "FreeStarVideoAdContainer", "freestar-video-parent", "reestar-video-child",
                    ],
                    vec![
                        "progress-bar", "js-menu", "social-share", "post-info__readtime",
                        "cta__description", "cta__inner", "cta__content", "hide-mobile", "js-toc",
                        "author-card", "related-posts", "post-info", "post-tags", "author-info",
                    ],
                )
            }
            _ => Self::new(vec![], vec![])
        }
    }
}
