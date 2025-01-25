use crate::models::article_metadata::AuthorMetadata;
use crate::models::supported_websites;

/// An enum representing all the websites supported by the toolkit.
///
/// Each supported variant contains two strings:
/// - The host part of the URL.
/// - The path part of the URL.
pub enum SupportedWebsite {
    Itsfoss(String, String),
    NewsItsfoss(String, String),
    NotSupported,
}

/// An enum representing all the errors that can occur when getting a website from URL.
pub enum WebsiteGettingError {
    ParseFailed,
    HostIsNone,
}

impl SupportedWebsite {
    pub fn get_host(&self) -> Option<String> {
        match self {
            SupportedWebsite::Itsfoss(host, _) => Some(host.to_string()),
            SupportedWebsite::NewsItsfoss(host, _) => Some(host.to_string()),
            SupportedWebsite::NotSupported => None,
        }
    }

    pub fn get_path(&self) -> Option<String> {
        match self {
            SupportedWebsite::Itsfoss(_, path) => Some(path.to_string()),
            SupportedWebsite::NewsItsfoss(_, path) => Some(path.to_string()),
            SupportedWebsite::NotSupported => None,
        }
    }

    /// Get the author metadata by extracting it from the HTML content of the given URL.
    ///
    /// # Current Supported Websites:
    /// - `itsfoss.com`
    /// - `news.itsfoss.com`
    ///
    /// # Arguments
    /// - `html`: The HTML content of the page to get the author metadata from.
    ///
    /// # Returns
    /// - Option<AuthorMetadata>: The metadata of the author of the article.
    ///     `None` if the website is not supported or the author metadata cannot be extracted.
    pub fn get_author_metadata(&self, html: &str) -> Option<AuthorMetadata> {
        match self {
            SupportedWebsite::Itsfoss(_, _) => supported_websites::Itsfoss::get_author_metadata(self, html),
            SupportedWebsite::NewsItsfoss(_, _) => supported_websites::NewsItsfoss::get_author_metadata(self, html),
            SupportedWebsite::NotSupported => None,
        }
    }
}

/// Get the website instance from a URL.
///
/// # Arguments
/// - `url`: The URL of the website.
///
/// # Returns
/// - Result<SupportedWebsites, WebsiteGettingError>:
///     - Ok(SupportedWebsites): The supported website instance.
///     - Err(WebsiteGettingError): The error that occurred when getting the website.
pub fn get_supported_website(url: &str) -> Result<SupportedWebsite, WebsiteGettingError> {
    let website = match url::Url::parse(url) {
        Ok(parsed_url) => parsed_url,
        Err(_) => return Err(WebsiteGettingError::ParseFailed),
    };

    let host = match website.host_str() {
        Some(host) => host,
        None => return Err(WebsiteGettingError::HostIsNone),
    };

    match host {
        "itsfoss.com" => Ok(SupportedWebsite::Itsfoss(
            host.to_string(),
            website.path().to_string(),
        )),
        "news.itsfoss.com" => Ok(SupportedWebsite::NewsItsfoss(
            host.to_string(),
            website.path().to_string(),
        )),
        _ => Ok(SupportedWebsite::NotSupported),
    }
}
