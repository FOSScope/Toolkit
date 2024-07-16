use std::fs;
use std::fs::File;
use std::io::{BufReader, stdin, stdout, Write};
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub editor: String,
}

impl Config {
    pub fn new(editor: String) -> Self {
        Self {
            editor,
        }
    }
}

pub fn get_config() -> Config {
    let config_file = File::open(".fosscope_toolkit/config.json").unwrap();
    let reader = BufReader::new(config_file);
    let config: Result<Config, _> = serde_json::from_reader(reader);
    match config {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Failed to read the configuration file.");
            std::process::exit(1);
        }
    }
}

pub fn initial_configuration_process() -> Result<Config, String> {
    let dir_path = Path::new(".fosscope_toolkit");
    if !dir_path.exists() {
        fs::create_dir_all(".fosscope_toolkit").unwrap();
    }

    let file_path = Path::new(".fosscope_toolkit/config.json");
    if !file_path.exists() {
        let mut user_input = String::new();
        print!("Enter the terminal command that you use to open your text editor, \
        or enter nothing to not use any terminal text editor: ");
        let _= stdout().flush();
        stdin().read_line(&mut user_input).unwrap_or(0);
        let user_input = user_input.trim().to_string();
        let config = Config::new(user_input);
        let config_json = serde_json::to_string(&config);
        match config_json {
            Ok(config_json) => {
                let mut file = File::create(".fosscope_toolkit/config.json").unwrap();
                file.write_all(config_json.as_bytes()).unwrap();
                Ok(config)
            },
            Err(_) => Err("Failed to create the configuration file.".to_string())
        }

    } else {
        Ok(get_config())
    }
}
