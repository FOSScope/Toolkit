/// The metadata of the author of an article.
///
/// # Fields
/// - `name`: The name of the author.
/// - `link`: The link to the author's profile.
pub struct AuthorMetadata {
    pub name: String,
    pub link: String,
}

impl AuthorMetadata {
    pub fn new(name: String, link: String) -> Self {
        Self { name, link }
    }
}
