use std::fs;
use std::fs::File;
use std::io::{BufReader, stdin, stdout, Write};
use std::path::Path;

/// A structure representing all information in the configuration file.
///
/// # Fields
/// - `editor`: The terminal command that the user uses to open their text editor.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub editor: String,
}

impl Config {
    pub fn new(editor: String) -> Self {
        Self { editor }
    }
}

/// Get the configuration from the configuration file. (`.fosscope_toolkit/config.json`)
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

/// Initial Configuration Process
///
/// Create the configuration file if it does not exist, or read the configuration file if it exists.
///
/// # Returns
/// - `Result<Config, String>`:
///    - `Ok(Config)`: The newly created or existing configuration.
///    - `Err(String)`: An error message indicating why the process failed.
pub fn initial_configuration_process() -> Result<Config, String> {
    let dir_path = Path::new(".fosscope_toolkit");
    if !dir_path.exists() {
        fs::create_dir_all(".fosscope_toolkit").unwrap();
    }

    let file_path = Path::new(".fosscope_toolkit/config.json");

    if !file_path.exists() {
        // If the configuration file does not exist, create it.
        let mut user_input = String::new();

        // Ask the user to configure everything.
        print!(
            "Enter the terminal command that you use to open your text editor, \
        or enter nothing to not use any terminal text editor: "
        );
        let _ = stdout().flush();
        stdin().read_line(&mut user_input).unwrap_or(0);
        let user_input = user_input.trim().to_string();

        // Create the new configuration.
        let config = Config::new(user_input);
        let config_json = serde_json::to_string(&config);
        match config_json {
            Ok(config_json) => {
                // Write the configuration to the configuration file.
                let mut file = File::create(".fosscope_toolkit/config.json").unwrap();
                file.write_all(config_json.as_bytes()).unwrap();
                // Return the newly created configuration.
                Ok(config)
            }
            Err(_) => Err("Failed to create the configuration file.".to_string()),
        }
    } else {
        // If the configuration file exists, read it and return the configuration.
        Ok(get_config())
    }
}
