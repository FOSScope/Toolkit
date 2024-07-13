pub struct HTMLFilterRule {
    pub tags: Vec<&str>,
    pub classes: Vec<&str>,
}

impl HTMLFilterRule {
    pub fn new(tags: Vec<&str>, classes: Vec<&str>) -> Self {
        Self {
            tags,
            classes,
        }
    }
}
