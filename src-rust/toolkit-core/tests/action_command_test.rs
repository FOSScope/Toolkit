mod tests {
    use fosscopetoolkit_core::models::action_command::ActionCommand;

    #[test]
    fn cp() {
        // Create Test Directory
        let _ = std::fs::create_dir(".test");

        // Create Test Directory
        let _ = std::fs::create_dir(".test/source");
        // Create Test File
        let _ = std::fs::write(".test/source/test.md", "Test File");

        // Test The Command
        let command = ActionCommand::new(
            "CP".to_string(),
            vec![
                ".test/source/test.md".to_string(),
                ".test/copied/test.md".to_string(),
            ],
        );

        let r = command.execute();
        assert!(r.is_ok());

        let _ = std::fs::create_dir(".test/source_dir");
        let _ = std::fs::write(".test/source_dir/test.md", "Dir Test File");

        let command = ActionCommand::new(
            "CP".to_string(),
            vec![
                ".test/source_dir".to_string(),
                ".test/copied_dir".to_string(),
            ],
        );

        let r = command.execute();
        assert!(r.is_ok());
    }
}
