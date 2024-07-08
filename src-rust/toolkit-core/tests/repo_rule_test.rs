#[cfg(test)]
mod tests {
    use fosscopetoolkit_core::get_repo_rule;
    use fosscopetoolkit_core::models::{Article, Action, GitRule, RepoRule};

    #[test]
    fn deserialize() {
        let rule = r#"[[articles]]
# Each `[[articles]]` block defines a type of article available to contribute to.
type = "news"   # The type of article.
description = "News Articles"   # The description of the article type.
directory = "{step}/news"   # The directory where the articles of this type are stored.
                            # `{step}` is the placeholder for the directory where the article will be moved from
                            # step to step (e.g. "source", "translated", "published", etc.)

# Multiple article types can be defined.
[[articles]]
type = "tech"
description = "Tech Articles"
directory = "{step}/tech"

[[actions]]
# Each `[[actions]]` block defines an action that can be made in the contribution process.
action = "select"   # The action name.
description = "Select an article to translate."  # The description of the action.
command = "TOUCH source/{article}.md"   # The command to execute when the action is made.
                                        # The command follows a *nix shell command syntax, but is defined, parsed, and executed by the core component of Toolkit software.
                                        # In this case, {article} is the placeholder for the article name.

# Multiple actions can be defined.
[[actions]]
action = "translate"
description = "Translate the article."
command = "MV source/{article}.md translated/{article}.md"

[git]
# This section defines how git conventions applies in different steps.
# `{action}`, `{type}`, and `{article}` are placeholders for the action's name, article type, and article name respectively.
# Other placeholders can be used as well.
branch_naming = "{action}/{type}/{article}"  # The branch naming rule.
commit_message = "[{action.desc}][{type.desc}]: {article.title}"  # The commit message rule."#;

        let deserialized = get_repo_rule(rule).unwrap();

        let news: Article = Article::new("news".to_string(), "News Articles".to_string(), "{step}/news".to_string());
        let tech: Article = Article::new("tech".to_string(), "Tech Articles".to_string(), "{step}/tech".to_string());
        let select: Action = Action::new("select".to_string(), "Select an article to translate.".to_string(), "TOUCH source/{article}.md".to_string());
        let translate: Action = Action::new("translate".to_string(), "Translate the article.".to_string(), "MV source/{article}.md translated/{article}.md".to_string());
        let git_rule: GitRule = GitRule::new("{action}/{type}/{article}".to_string(), "[{action.desc}][{type.desc}]: {article.title}".to_string());

        let expected = RepoRule::new(vec![news, tech], vec![select, translate], git_rule);

        assert_eq!(deserialized, expected);
    }
}
