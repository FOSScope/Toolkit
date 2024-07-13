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

                let src_dir = std::path::Path::new(src);
                if src_dir.is_dir() {
                    let r = copy_dir_all(src, dest);
                    match r {
                        Ok(_) => Ok(()),
                        Err(_) => Err("Error copying directory"),
                    }
                } else {
                    let dest_dir = std::path::Path::new(dest);
                    if !dest_dir.exists() {
                        std::fs::create_dir_all(dest_dir.parent().unwrap()).unwrap();
                    }

                    std::fs::copy(src, dest).unwrap();
                    Ok(())
                }
            }
            // Write a content to a file
            "ECHO" => {
                let path = self.args.get(0).unwrap();
                let content = self.args.get(1).unwrap();
                std::fs::write(path, content).unwrap();
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

                let dest_dir = std::path::Path::new(dest);
                if !dest_dir.exists() {
                    std::fs::create_dir_all(dest_dir.parent().unwrap()).unwrap();
                }

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

fn copy_dir_all(src: impl AsRef<std::path::Path>, dst: impl AsRef<std::path::Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
