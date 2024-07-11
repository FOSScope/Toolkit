#[derive(PartialEq, Eq, Debug, serde::Deserialize)]
pub struct HTMLFilterRule {
    pub tags: Vec<String>,
    pub classes: Vec<String>,
}

impl HTMLFilterRule {
    pub fn new(tags: Vec<String>, classes: Vec<String>) -> Self {
        Self {
            tags,
            classes,
        }
    }
}
