mod tests {
    #[test]
    fn select_translate_article() {
        let url = "https://itsfoss.com/ollama/";

        let content = fosscopetoolkit_core::workflow::translate::select::get_content(
            url
        );

        match content {
            Ok(content) => {
                println!("Content:\n{}", content);
            },
            Err(e) => {
                eprintln!("Error:\n{}", e);
            }
        }
    }
}
