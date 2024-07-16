use std::collections::HashMap;
use std::fs;
use std::path::Path;

use handlebars::Handlebars;

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

    pub fn execute(&self, vars: Option<&HashMap<&str, &str>>) -> Result<(), &str> {
        let mut args = self.args.clone();
        if vars.is_some() {
            let handlebars = Handlebars::new();
            args = args.iter().map(
                |arg| handlebars.render_template(&*arg, &vars).unwrap()
            ).collect();
        }

        match self.command.as_str() {
            // Copy a file or a directory
            "CP" => {
                let src = args.get(0).unwrap();
                let dest = args.get(1).unwrap();

                let src_dir = Path::new(&src);
                if src_dir.is_dir() {
                    let r = copy_dir_all(src, dest);
                    match r {
                        Ok(_) => Ok(()),
                        Err(_) => Err("Error copying directory"),
                    }
                } else {
                    let dest_dir = Path::new(&dest);
                    if !dest_dir.exists() {
                        fs::create_dir_all(dest_dir.parent().unwrap()).unwrap();
                    }

                    fs::copy(src, dest).unwrap();
                    Ok(())
                }
            }
            // Write a content to a file
            "ECHO" => {
                let path = args.get(0).unwrap();
                let content = args.get(1).unwrap();
                fs::write(path, content).unwrap();
                Ok(())
            }
            // Create a directory
            "MKDIR" => {
                let path = args.get(0).unwrap();
                fs::create_dir_all(path).unwrap();
                Ok(())
            }
            // Move a file or a directory
            "MV" => {
                let src = args.get(0).unwrap();
                let dest = args.get(1).unwrap();

                let dest_dir = Path::new(dest);
                if !dest_dir.exists() {
                    fs::create_dir_all(dest_dir.parent().unwrap()).unwrap();
                }

                fs::rename(src, dest).unwrap();
                Ok(())
            }
            // Remove a file or a directory
            "RM" => {
                let path = args.get(0).unwrap();
                if fs::metadata(path).unwrap().is_dir() {
                    fs::remove_dir_all(path).unwrap();
                } else {
                    fs::remove_file(path).unwrap();
                }
                Ok(())
            }
            // Create a file
            "TOUCH" => {
                let path = args.get(0).unwrap();
                fs::File::create(path).unwrap();
                Ok(())
            }
            _ => Err("Unknown command"),
        }
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
