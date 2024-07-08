/// A representation of a GitHub repository, which includes the owner and the name of the repository.
///
/// Derived from the `serde::Serialize` and `serde::Deserialize` traits to allow serialization and deserialization from JSON.
///
/// Derived from the `Clone`, `PartialEq`, and `Eq` traits to allow cloning and comparison.
///
/// # Fields
/// - `owner`: The owner of the repository. e.g. `FOSScope`.
/// - `name`: The name of the repository. e.g. `Toolkit`.
#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GitHubRepo {
    /// The owner of the repository.
    pub owner: String,
    /// The name of the repository.
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
