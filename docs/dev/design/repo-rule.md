# Rule Definition in Repositories

FOSScope has two repositories (as of writing) for contributors to contribute. Which are:

- Articles, that holds the original articles written by contributors, and reposts where the source are already in Chinese.
- TranslationProject, that holds the source, work-in-progress, and finished translations.

These repositories have different types of work involved, and thus have different rules to follow when contributing.

For example, there are more steps involved into translation contribution, where "topic selection" is a crucial step, that is simply non-existent, on the other hand, in the original article project.

So, for the rule definition, it has to be well written to:

1. Define available types of articles to contribute to.
2. Define the available action to make in every step of contribution.
3. How git conventions (i.e. commit message rules, branch naming rules, etc.) applies in different steps.

Here's how the exact rule file is defined.

The repository rule file will be written to a TOML file, following the structure below:

```toml
[[articles]]
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

# [[articles]]
# ...

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

# [[actions]]
# ...

[git]
# This section defines how git conventions applies in different steps.
# `{action}`, `{type}`, and `{article}` are placeholders for the action's name, article type, and article name respectively.
# Other placeholders can be used as well.
branch-naming = "{action}/{type}/{article}"  # The branch naming rule.
commit-message = "[{action.desc}][{type.desc}]: {article.title}"  # The commit message rule.
```

> [!NOTE]
> In general, placeholders like `{step}` can be used anywhere, and placerholders other than what's shown in the example above can be defined and used as well.
