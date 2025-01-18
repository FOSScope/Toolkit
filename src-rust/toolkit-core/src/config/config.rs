use std::fs;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Write};
use std::path::Path;

/* ----------------- Configuration Data Structure ----------------- */

/// A structure that holds all configuration information for the toolkit.
///
/// # Fields
/// - `editor`: The editor to use for opening files. *(a terminal command)*
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub editor: String,
}

impl Config {
    pub fn new(editor: String) -> Self {
        Self { editor }
    }
}

/* ----------------- Other Utility Functions ----------------- */

/// Possible errors that can occur when creating or loading the configuration.
#[derive(Debug)]
pub enum ConfigCreationLoadError {
    /// Error when cannot create the configuration file.
    FileCreateError,
    /// Error when cannot open the configuration file.
    FileOpenError,
    /// Error when cannot write to the configuration file.
    FileWriteError,
    /// Error when cannot parse the configuration from stored JSON.
    ParseError,
    /// Error when serializing the configuration to JSON.
    SerializeFailed,
}

/// Load the configuration from the config file (`.fosscope_toolkit/config.json`).
///
/// # Returns
/// - Result<Config, ConfigCreationLoadError>:
///     - Ok(Config): The configuration read from the file.
///     - Err(ConfigCreationLoadError): Reason why the configuration could not be loaded.
pub fn get_config() -> Result<Config, ConfigCreationLoadError> {
    let config_file = File::open(".fosscope_toolkit/config.json");
    if config_file.is_err() {
        return Err(ConfigCreationLoadError::FileOpenError);
    }
    let reader = BufReader::new(config_file.unwrap());
    let config: Result<Config, _> = serde_json::from_reader(reader);
    match config {
        Ok(config) => Ok(config),
        Err(_) => Err(ConfigCreationLoadError::FileWriteError),
    }
}

/// The configuration process during initialization of the toolkit.
///
/// This function is responsible for creating the configuration for the toolkit.
/// When the configuration file (`.fosscope_toolkit/config.json`) exists, the config will be loaded from it.
/// Otherwise, a new configuration will be created through an interactive process.
///
/// # Returns
/// - Result<Config, ConfigCreationError>:
///     - Ok(Config): The configuration for the toolkit. Read from the file or newly created.
///     - Err(ConfigCreationLoadError): Reason why the configuration could not be loaded/created.
pub fn config_process() -> Result<Config, ConfigCreationLoadError> {
    let dir_path = Path::new(".fosscope_toolkit");
    if !dir_path.exists() {
        fs::create_dir_all(".fosscope_toolkit").unwrap();
    }
    let file_path = Path::new(".fosscope_toolkit/config.json");

    if !file_path.exists() {
        // If the config file does not exist, create a new configuration.
        let mut user_input = String::new();

        // Get user inputs for the configuration.
        print!(
            "Enter the terminal command that you use to open your text editor, \
            or enter nothing to not use any terminal text editor: "
        );
        let _ = stdout().flush();
        stdin().read_line(&mut user_input).unwrap_or(0);
        let editor = user_input.trim().to_string();

        // Create a new configuration.
        let config = Config::new(editor);
        let config_json = serde_json::to_string(&config);
        match config_json {
            Ok(config_json) => {
                // Write the configuration to the configuration file.
                let file = File::create(".fosscope_toolkit/config.json");
                if file.is_err() {
                    return Err(ConfigCreationLoadError::FileCreateError);
                }
                match file.unwrap().write_all(config_json.as_bytes()) {
                    Ok(_) => Ok(config), // Return the created configuration.
                    Err(_) => Err(ConfigCreationLoadError::FileCreateError),
                }
            },
            Err(_) => Err(ConfigCreationLoadError::SerializeFailed),
        }
    } else {
        // If the config file exists, load the configuration from it.
        get_config()
    }
}
