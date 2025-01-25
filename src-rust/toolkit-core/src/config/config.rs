use crate::models::GitHubRepo;
use std::collections::HashMap;
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
    pub contributor_repos: HashMap<String, GitHubRepo>,
}

impl Config {
    pub fn new(editor: String) -> Self {
        Self {
            editor,
            contributor_repos: HashMap::new(),
        }
    }

    /// Get the contributor repository for an upstream repository.
    ///
    /// # Arguments
    /// - `config`: The configuration to get the contributor repository from.
    /// - `upstream`: The upstream repository to get the contributor repository for.
    ///
    /// # Returns
    /// - Option<GitHubRepo>: The contributor repository for the upstream repository, if it exists.
    pub fn get_contributor_repo(&self, upstream: &GitHubRepo) -> Option<GitHubRepo> {
        self.contributor_repos
            .get(&upstream.get_full_name())
            .cloned()
    }

    /// Set the contributor repository for an upstream repository, and update the configuration file.
    ///
    /// # Arguments
    /// - `config`: The configuration to update.
    /// - `upstream`: The upstream repository to set the contributor repository for.
    /// - `contributor`: The contributor repository to set.
    ///
    /// # Returns
    /// - Result<(), ConfigCreationLoadError>:
    ///     - Ok(()): The contributor repository was successfully set and the configuration updated.
    ///     - Err(ConfigCreationLoadError): Reason why the contributor repository could not be set.
    pub fn set_contributor_repo(
        &mut self,
        upstream: &GitHubRepo,
        contributor: GitHubRepo,
    ) -> Result<(), ConfigCreationLoadError> {
        self.contributor_repos
            .insert(upstream.get_full_name(), contributor);
        self.write_config()
    }

    /// Write the configuration to the config file (`.fosscope_toolkit/config.json`).
    ///
    /// # Arguments
    /// - `config`: The configuration to write to the file.
    ///
    /// # Returns
    /// - Result<(), ConfigCreationLoadError>:
    ///     - Ok(()): The configuration was successfully written to the file.
    ///     - Err(ConfigCreationLoadError): Reason why the configuration could not be written.
    pub fn write_config(&self) -> Result<(), ConfigCreationLoadError> {
        let config_json = serde_json::to_string(self);
        match config_json {
            Ok(config_json) => {
                let file = File::create(".fosscope_toolkit/config.json");
                if file.is_err() {
                    return Err(ConfigCreationLoadError::FileCreateError);
                }
                match file.unwrap().write_all(config_json.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(ConfigCreationLoadError::FileWriteError),
                }
            }
            Err(_) => Err(ConfigCreationLoadError::SerializeFailed),
        }
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
        Err(_) => Err(ConfigCreationLoadError::ParseError),
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
        match config.write_config() {
            Ok(_) => Ok(config),
            Err(e) => Err(e),
        }
    } else {
        // If the config file exists, load the configuration from it.
        get_config()
    }
}
