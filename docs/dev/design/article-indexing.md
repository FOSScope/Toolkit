# Article Indexing design in FOSScope Toolkit

This article describes the design of the article indexing feature in the FOSScope Toolkit.

## Objective

The objective of the article indexing feature is to provide a way to efficiently track all articles that is being worked on by the team.

The index should contain the following information:

- Current Status of the Corresponding Article
- Title
- Author
- Type of the Corresponding Article
- Selector (if applicable)
- Translator (if applicable)
- Proofreader (if applicable)

## Design

### Indexing File

The article index file should be a JSON file, stored in the root of the repository with the name `.article-index.json`.

The JSON file contains several lists, each representing the status of the corresponding articles.

Taking the translation project as the example, the lists are as follows:

- `source`, which contains the information of the source articles, which are articles that got selected by a selector and are yet to be worked on.
- `translated`, which contains the information of the translated articles, which are articles that got translated by a translator and are yet to be proofread.
- `published`, which contains the information of the published articles, which are articles that are translated and proofread and are published on the website.

And in each list, the information of each article is stored as an object.

### File Object

The object representing an article should contain the following fields:

- `title`, which is the title of the article.
- `author`, which is the author of the article.
- `selector`, which is the selector of the article (if applicable).
- `translator`, which is the translator of the article (if applicable).
- `proofreader`, which is the proofreader of the article (if applicable).
- `type`, which is the type of the article.
