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

    pub fn execute(&self) -> Result<(), &str> {
        match self.command.as_str() {
            // Copy a file or a directory
            "CP" => {
                let src = self.args.get(0).unwrap();
                let dest = self.args.get(1).unwrap();
                std::fs::copy(src, dest).unwrap();
                Ok(())
            }
            // Create a directory
            "MKDIR" => {
                let path = self.args.get(0).unwrap();
                std::fs::create_dir_all(path).unwrap();
                Ok(())
            }
            // Move a file or a directory
            "MV" => {
                let src = self.args.get(0).unwrap();
                let dest = self.args.get(1).unwrap();
                std::fs::rename(src, dest).unwrap();
                Ok(())
            }
            // Remove a file or a directory
            "RM" => {
                let path = self.args.get(0).unwrap();
                if std::fs::metadata(path).unwrap().is_dir() {
                    std::fs::remove_dir_all(path).unwrap();
                } else {
                    std::fs::remove_file(path).unwrap();
                }
                Ok(())
            }
            // Create a file
            "TOUCH" => {
                let path = self.args.get(0).unwrap();
                std::fs::File::create(path).unwrap();
                Ok(())
            }
            _ => Err("Unknown command"),
        }
    }
}
