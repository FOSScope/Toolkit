use crate::contributor_repo_init;
use crate::workflow::translate_project::action::{
    show_translate_project_action_menu, TranslateProjectAction,
};
use fosscopetoolkit_core::apis::GitHubApi;
use fosscopetoolkit_core::config::config::Config;
use fosscopetoolkit_core::models::GitHubRepo;
use std::io::stdin;

pub enum Project {
    Articles, // TODO: Implement Article Project Related Functions
    TranslateProject,
    NONE,
}

impl Project {
    /// Start to work on a project following the workflow.
    ///
    /// # Returns
    /// - bool: Whether the program should exit after the workflow is finished.
    pub async fn start(&self, config: &mut Config, github: &GitHubApi) {
        match self {
            Project::Articles => {
                eprintln!("Not implemented yet. Please select another option.");
            }
            Project::TranslateProject => {
                let upstream_repo =
                    GitHubRepo::new("FOSScope".to_string(), "TranslateProject".to_string());

                let contributor_repo = match config.get_contributor_repo(&upstream_repo) {
                    Some(contributor_repo) => contributor_repo,
                    None => {
                        let contributor_repo = contributor_repo_init(&github, &upstream_repo).await;
                        match config.set_contributor_repo(&upstream_repo, contributor_repo.clone())
                        {
                            Ok(_) => contributor_repo,
                            Err(e) => {
                                eprintln!("Error setting contributor repo: {:?}", e);
                                std::process::exit(1);
                            }
                        }
                    }
                };

                println!("\n========================================");
                let mut user_input = String::new();
                let mut action: TranslateProjectAction;
                loop {
                    show_translate_project_action_menu();
                    stdin().read_line(&mut user_input).unwrap_or(0);
                    action = match user_input.to_uppercase().trim() {
                        // Project Selection
                        "1" => TranslateProjectAction::Select,
                        "2" => TranslateProjectAction::Translate,
                        "3" => TranslateProjectAction::Review,
                        "4" => TranslateProjectAction::Publish,
                        // Commands Selection
                        "B" => TranslateProjectAction::Back,
                        _ => TranslateProjectAction::Invalid,
                    };
                    user_input.clear();
                    if action.perform().await {
                        break;
                    }
                }
                println!("========================================\n");
            }
            _ => {
                // This should be unreachable.
                eprintln!("Reached Unreachable Code. Please report this issue.");
                eprintln!("ID: toolkit-cli::workflow::workflow::Project::start");
                std::process::exit(1);
            }
        }
    }
}
