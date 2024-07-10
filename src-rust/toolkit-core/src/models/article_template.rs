pub struct ArticleTemplate {
    pub header: Option<String>,
    pub footer: Option<String>,
}

impl ArticleTemplate {
    pub fn new(header: Option<String>, footer: Option<String>) -> Self {
        Self {
            header,
            footer,
        }
    }
}
