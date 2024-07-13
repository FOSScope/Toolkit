pub struct HTMLFilterRule<'a> {
    pub tags: Vec<&'a str>,
    pub classes: Vec<&'a str>,
}

impl HTMLFilterRule {
    pub fn new(tags: Vec<&str>, classes: Vec<&str>) -> Self {
        Self {
            tags,
            classes,
        }
    }
}
