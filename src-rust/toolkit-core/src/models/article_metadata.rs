use scraper::{Html, Selector};

use crate::models::SupportedWebsites;

pub struct AuthorMetadata {
    pub name: String,
    pub link: String,
}

impl AuthorMetadata {
    pub fn new(name: String, link: String) -> Self {
        Self { name, link }
    }

    /// Get the author metadata by extracting it from the HTML content of the given URL.
    ///
    /// # Current Supported Websites:
    /// - `itsfoss.com`
    /// - `news.itsfoss.com`
    ///
    /// # Arguments
    /// - `website`: The website to get the author metadata from.
    /// - `html`: The HTML content of the page to get the author metadata from.
    pub async fn get(website: &SupportedWebsites, html: &str) -> Option<Self> {
        match website {
            SupportedWebsites::Itsfoss(_, _) => {
                let document = Html::parse_document(&*html);

                let element = document.select(
                    &Selector::parse("span.post-info__author").unwrap()
                ).next();
                let element = match element {
                    Some(element) => element,
                    None => return None,
                };

                let link = element.child_elements().next().unwrap().attr("href");
                let link = match link {
                    Some(link) => link,
                    None => return None,
                };
                let text = element.text().collect::<Vec<_>>()[0];

                // Append the host to the link if it is a relative link
                let link = if link.starts_with("/") {
                    format!("https://{}{}", website.get_host().unwrap(), link)
                } else {
                    link.to_string()
                };

                Some(Self::new(text.to_string(), link.to_string()))
            }

            SupportedWebsites::NewsItsfoss(_, _) => {
                let document = Html::parse_document(&*html);

                let element = document.select(
                    &Selector::parse("div.c-byline.c-topper__byline").unwrap()
                ).next();
                let element = match element {
                    Some(element) => element,
                    None => return None,
                };

                // Search for the link (`<a>`) element, which contains author information
                let element = element.select(
                    &Selector::parse("a").unwrap()
                ).next();
                let element = match element {
                    Some(element) => element,
                    None => return None,
                };

                let link = element.attr("href");
                let link = match link {
                    Some(link) => link,
                    None => return None,
                };
                let text = element.text().collect::<Vec<_>>()[0];

                // Append the host to the link if it is a relative link
                let link = if link.starts_with("/") {
                    format!("https://{}{}", website.get_host().unwrap(), link)
                } else {
                    link.to_string()
                };

                Some(Self::new(text.to_string(), link.to_string()))
            }

            _ => None
        }
    }
}
