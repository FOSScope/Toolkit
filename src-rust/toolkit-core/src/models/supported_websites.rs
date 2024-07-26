/// An enum representing all the websites supported by the toolkit.
/// Each supported variant contains two strings:
/// - The host part of the URL.
/// - The path part of the URL.
pub enum SupportedWebsites {
    Itsfoss(String, String),
    NewsItsfoss(String, String),
    NotSupported,
}

impl SupportedWebsites {
    pub fn get_host(&self) -> Option<String> {
        match self {
            SupportedWebsites::Itsfoss(host, _) => Some(host.to_string()),
            SupportedWebsites::NewsItsfoss(host, _) => Some(host.to_string()),
            SupportedWebsites::NotSupported => None,
        }
    }

    pub fn get_path(&self) -> Option<String> {
        match self {
            SupportedWebsites::Itsfoss(_, path) => Some(path.to_string()),
            SupportedWebsites::NewsItsfoss(_, path) => Some(path.to_string()),
            SupportedWebsites::NotSupported => None,
        }
    }

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
            "itsfoss.com" => Ok(SupportedWebsites::Itsfoss(
                host.to_string(), website.path().to_string()
            )),
            "news.itsfoss.com" => Ok(SupportedWebsites::NewsItsfoss(
                host.to_string(), website.path().to_string()
            )),
            _ => Ok(SupportedWebsites::NotSupported),
        }
    }
}
