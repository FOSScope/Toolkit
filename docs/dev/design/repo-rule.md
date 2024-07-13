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
# The article template to use when creating a new source file for an article.
article_template = """---
title: {{title}}
date: <发布时间>
author:
  - fosscope-translation-team
  - {{translator}}
  - {{proofreader}}
banner: {{cover_image}}
cover: {{cover_image}}
categories:
  - 翻译
  - <类型>
tags:
  - <标签>
authorInfo: |
  via: {{via}}

  作者：[{{author}}]({{author_link}})
  选题：[{{selector}}](https://github.com/{{selector}})
  译者：[{{translator}}](https://github.com/{{translator}})
  校对：[{{proofreader}}](https://github.com/{{proofreader}})

  本文由 [FOSScope翻译组](https://github.com/FOSScope/TranslateProject) 原创编译，[开源观察](https://fosscope.com/) 荣誉推出
---

<!-- 所有在被 `<>` 标记的地方都需要填写对应信息 -->

{{summary}}

<!-- more -->

{{content}}
"""


[[articles]]
# Each `[[articles]]` block defines a type of article available to contribute to.
type = "news"   # The type of article.
description = "News Articles"   # The description of the article type.
directory = "{{step}}/news"   # The directory where the articles of this type are stored.
                            # `{step}` is the placeholder for the directory where the article will be moved from
                            # step to step (e.g. "source", "translated", "published", etc.)
# If needed, a specific article template can be defined for this article type.
# Otherwise, the default article template will be used.
# template = """
# """

# Multiple article types can be defined.
[[articles]]
type = "tech"
description = "Tech Articles"
directory = "{{step}}/tech"

# [[articles]]
# ...

[[actions]]
# Each `[[actions]]` block defines an action that can be made in the contribution process.
action = "select"   # The action name.
description = "Select an article to translate."  # The description of the action.
command = "TOUCH source/{{article_id}}.md"   # The command to execute when the action is made.
                                        # The command follows a *nix shell command syntax, but is defined, parsed, and executed by the core component of Toolkit software.
                                        # In this case, {article} is the placeholder for the article name.

# Multiple actions can be defined.
[[actions]]
action = "translate"
description = "Translate the article."
command = "MV source/{{article_id}}.md translated/{{article_id}}.md"

# [[actions]]
# ...

[git]
# This section defines how git conventions applies in different steps.
# `{action}`, `{type}`, and `{article}` are placeholders for the action's name, article type, and article name respectively.
# Other placeholders can be used as well.
branch_naming = "{{action_name}}/{{type_name}}/{{article_id}}"  # The branch naming rule.
commit_message = "[{{action_desc}}][{{type_desc}}]: {{article_title}}"  # The commit message rule.
```

> [!NOTE]
> 
> In general, placeholders like `{{title}}` can be used anywhere, and the template engine will replace them with the actual value when generating the file.
> 
> Place holder other than what's shown above may be defined and used.
