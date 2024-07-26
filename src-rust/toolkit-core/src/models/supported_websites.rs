/// An enum representing all the websites supported by the toolkit.
pub enum SupportedWebsites {
    Itsfoss,
    NewsItsfoss,
    NotSupported,
}

impl SupportedWebsites {
    /// Get the website from a URL.
    ///
    /// # Arguments
    /// - `url`: The URL of the website.
    ///
    /// # Returns
    /// - `SupportedWebsites`: The website supported by the toolkit.
    pub fn get_website(url: &str) -> Result<SupportedWebsites, String> {
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

        match host {
            "itsfoss.com" => Ok(SupportedWebsites::Itsfoss),
            "news.itsfoss.com" => Ok(SupportedWebsites::NewsItsfoss),
            _ => Ok(SupportedWebsites::NotSupported),
        }
    }
}
