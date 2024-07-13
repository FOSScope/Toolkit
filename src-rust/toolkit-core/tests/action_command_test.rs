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

    #[test]
    fn echo() {
        // Create Test Directory
        let _ = std::fs::create_dir(".test");

        // Test The Command
        let command = ActionCommand::new(
            "ECHO".to_string(),
            vec![
                ".test/echo.md".to_string(),
                "Echo Test".to_string(),
            ],
        );

        let r = command.execute();
        assert!(r.is_ok());
    }

    #[test]
    fn mkdir() {
        // Create Test Directory
        let _ = std::fs::create_dir(".test");

        // Test The Command
        let command = ActionCommand::new(
            "MKDIR".to_string(),
            vec![
                ".test/mkdir".to_string(),
            ],
        );

        let r = command.execute();
        assert!(r.is_ok());
    }

    #[test]
    fn mv() {
        // Create Test Directory
        let _ = std::fs::create_dir(".test");

        // Create Test Directory
        let _ = std::fs::create_dir(".test/mv-src");
        // Create Test File
        let _ = std::fs::write(".test/mv-src/test.md", "Test File");

        // Test The Command
        let command = ActionCommand::new(
            "MV".to_string(),
            vec![
                ".test/mv-src/test.md".to_string(),
                ".test/moved/test.md".to_string(),
            ],
        );

        let r = command.execute();
        assert!(r.is_ok());

        let _ = std::fs::write(".test/mv-src/test.md", "Dir Test File");

        let command = ActionCommand::new(
            "MV".to_string(),
            vec![
                ".test/mv-src".to_string(),
                ".test/moved_dir".to_string(),
            ],
        );

        let r = command.execute();
        assert!(r.is_ok());
    }

    #[test]
    fn touch() {
        // Create Test Directory
        let _ = std::fs::create_dir(".test");

        // Test The Command
        let command = ActionCommand::new(
            "TOUCH".to_string(),
            vec![
                ".test/touch.md".to_string(),
            ],
        );

        let r = command.execute();
        assert!(r.is_ok());
    }

    #[test]
    fn unknown() {
        // Test The Command
        let command = ActionCommand::new(
            "UNKNOWN".to_string(),
            vec![],
        );

        let r = command.execute();
        assert!(r.is_err());
    }
}