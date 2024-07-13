#[derive(PartialEq, Eq, Debug)]
pub struct ActionCommand {
    pub command: String,
    pub args: Vec<String>,
}

impl<'de> serde::Deserialize<'de> for ActionCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let content = String::deserialize(deserializer)?;
        let mut parts = content.split_whitespace();

        let command = parts.next().unwrap().to_string();
        let args = parts.map(|s| s.to_string()).collect();

        Ok(Self {
            command,
            args,
        })
    }
}

impl ActionCommand {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self {
            command,
            args,
        }
    }
}
