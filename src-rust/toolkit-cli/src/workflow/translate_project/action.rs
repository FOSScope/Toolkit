use crate::workflow::translate_project::actions;
use fosscopetoolkit_core::apis::GitHubApi;
use fosscopetoolkit_core::models::GitHubRepo;
use std::io::Write;

pub enum TranslateProjectAction {
    Select,
    Translate,
    Review,
    Publish,
    Back,
    Invalid,
}

impl TranslateProjectAction {
    /// Perform the selected action.
    ///
    /// # Arguments
    /// - `github`: The GitHub API to use for performing the GitHub related actions.
    /// - `upstream_repo`: The upstream repository of the project.
    /// - `contributor_repo`: The contributor repository for contributing to the project.
    ///
    /// # Returns
    /// - bool: Whether the program should be back to the previous menu after action is performed.
    pub async fn perform(
        &self,
        github: &GitHubApi,
        upstream_repo: &GitHubRepo,
        contributor_repo: &GitHubRepo,
    ) -> bool {
        match self {
            TranslateProjectAction::Select => {
                println!("Selecting Article...");
            }
            TranslateProjectAction::Translate => {
                // TODO: Implement this Action
                println!("Translating Article...");
                eprintln!("Not implemented yet. Please select another option.");
            }
            TranslateProjectAction::Review => {
                // TODO: Implement this Action
                println!("Reviewing Translation...");
                eprintln!("Not implemented yet. Please select another option.");
            }
            TranslateProjectAction::Publish => {
                // TODO: Implement this Action
                println!("Publishing Article...");
                eprintln!("Not implemented yet. Please select another option.");
            }
            TranslateProjectAction::Back => return true,
            TranslateProjectAction::Invalid => {
                println!("Invalid input. Please select an option from the list.");
            }
        }
        false
    }
}

pub fn show_translate_project_action_menu() {
    println!(
        r#"Please select an action to perform for the Translate Project,
    1. Select Article
    2. Translate Article
    3. Review Translation
    4. Publish Article
or enter the following letters to execute the corresponding action.
    B: Back to Main Menu"#
    );
    print!("IN: ");
    std::io::stdout().flush().unwrap()
}
