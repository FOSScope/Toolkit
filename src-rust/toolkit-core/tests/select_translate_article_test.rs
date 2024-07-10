mod tests {
    use fosscopetoolkit_core::models::article_template::ArticleTemplate;
    use fosscopetoolkit_core::models::html_filter_rule::HTMLFilterRule;

    #[test]
    fn select_translate_article() {
        let tags = vec!["script", "style", "link", "meta", "li", "desc", "title", "svg",
                        "path", "dialog", "select", "head", "header", "foot", "footer", "ul", "nav",
                        "button", "form", "input", "picture", "time", "h2", "h3", "h4", "i", "aside",
                        "FreeStarVideoAdContainer", "freestar-video-parent", "reestar-video-child"
        ].iter().map(|s| s.to_string()).collect();
        ;
        let classes = vec!["progress-bar", "js-menu", "social-share", "post-info__readtime",
                           "cta__description", "cta__inner", "cta__content", "hide-mobile", "js-toc",
                           "author-card", "related-posts"
        ].iter().map(|s| s.to_string()).collect();

        let url = "https://itsfoss.com/ollama/";
        let html_filter_rule = HTMLFilterRule::new(
            tags,
            classes
        );

        let content_template = r#"---
title: <标题>
date: <发布时间>
author:
  - fosscope-translation-team
  - <译者ID>
  - <校对者ID>
banner: <封面图片地址>
cover: <封面图片地址（与 banner 一致）>
categories:
  - 翻译
  - <类型>
tags:
  - <标签>
authorInfo: |
  via: <原文链接>

  作者：[<原作者>](<原作者信息页>)
  选题：[<选题者ID>](https://github.com/<选题者ID>)
  译者：[<译者ID>](https://github.com/<译者ID>)
  校对：[<校对者ID>](https://github.com/<校对者ID>)

  本文由 [FOSScope翻译组](https://github.com/FOSScope/TranslateProject) 原创编译，[开源观察](https://fosscope.com/) 荣誉推出
---

<!-- 所有在被 `<>` 标记的地方都需要被替换成对应的内容 -->"#;

        let article_template = ArticleTemplate::new(
            Some(content_template.to_string()), None
        );

        let content = fosscopetoolkit_core::workflow::translate::select::get_content(
            url,
            html_filter_rule,
            &article_template,
        );
        println!("Content:");
        println!("{}", content);
    }
}
