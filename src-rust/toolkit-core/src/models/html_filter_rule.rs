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

    pub fn get_filter_rule(url: &str) -> Self {
        match url {
            "itsfoss.com" | "news.itsfoss.com" => {
                Self::new(
                    vec![
                        "script", "style", "link", "meta", "li", "desc", "title", "svg", "path",
                        "dialog", "select", "head", "header", "foot", "footer", "ul", "nav", "button",
                        "form", "input", "picture", "time", "h2", "h3", "h4", "i", "aside",
                        "FreeStarVideoAdContainer", "freestar-video-parent", "reestar-video-child",
                    ],
                    vec![
                        "progress-bar", "js-menu", "social-share", "post-info__readtime",
                        "cta__description", "cta__inner", "cta__content", "hide-mobile", "js-toc",
                        "author-card", "related-posts",
                    ],
                )
            }
            _ => Self::new(vec![], vec![])
        }
    }
}
