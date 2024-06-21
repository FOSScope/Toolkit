#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GitHubRepo {
    pub owner: String,
    pub name: String,
}

impl GitHubRepo {
    pub fn new(owner: String, name: String) -> Self {
        Self { owner, name }
    }

    pub fn get_full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}
