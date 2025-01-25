use crate::models::article_metadata::AuthorMetadata;
use crate::models::SupportedWebsite;
use scraper::{Html, Selector};

pub fn get_author_metadata(website: &SupportedWebsite, html: &str) -> Option<AuthorMetadata> {
    let document = Html::parse_document(&*html);

    let element = document
        .select(&Selector::parse("span.post-info__authors").unwrap())
        .next();
    let element = match element {
        Some(element) => element,
        None => return None,
    };

    // Search for the link (`<a>`) element, which contains author information
    let element = element.select(&Selector::parse("a").unwrap()).next();
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

    Some(AuthorMetadata::new(text.to_string(), link.to_string()))
}
