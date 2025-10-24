use journey::cli::{Cli, Commands};
use clap::Parser;
use std::path::PathBuf;

#[test]
fn test_cli_parse_default_behavior() {
    let cli = Cli::try_parse_from(&["journey", "test note"]).unwrap();
    assert_eq!(cli.note_content, vec!["test note"]);
    assert!(cli.command.is_none());
    assert!(!cli.list);
    assert!(!cli.edit);
}

#[test]
fn test_cli_parse_list_command() {
    let cli = Cli::try_parse_from(&["journey", "--list"]).unwrap();
    assert!(cli.list);
    assert!(cli.command.is_none());
}

#[test]
fn test_cli_parse_edit_command() {
    let cli = Cli::try_parse_from(&["journey", "--edit"]).unwrap();
    assert!(cli.edit);
    assert!(cli.command.is_none());
}

#[test]
fn test_cli_parse_with_vault() {
    let cli = Cli::try_parse_from(&["journey", "--vault", "test", "note"]).unwrap();
    assert_eq!(cli.vault, Some("test".to_string()));
    assert_eq!(cli.note_content, vec!["note"]);
}

#[test]
fn test_cli_parse_with_date() {
    let cli = Cli::try_parse_from(&["journey", "--date", "2025-10-24", "note"]).unwrap();
    assert_eq!(cli.date, Some("2025-10-24".to_string()));
    assert_eq!(cli.note_content, vec!["note"]);
}

#[test]
fn test_cli_parse_with_relative_date() {
    let cli = Cli::try_parse_from(&["journey", "--relative-date", "1", "note"]).unwrap();
    assert_eq!(cli.relative_date, Some(1));
    assert_eq!(cli.note_content, vec!["note"]);
}

#[test]
fn test_cli_parse_with_time() {
    let cli = Cli::try_parse_from(&["journey", "--time", "14:30", "note"]).unwrap();
    assert_eq!(cli.time, Some("14:30".to_string()));
    assert_eq!(cli.note_content, vec!["note"]);
}

#[test]
fn test_cli_parse_init_command() {
    let cli = Cli::try_parse_from(&["journey", "init", "--path", "/tmp/test", "--name", "test"]).unwrap();
    match cli.command {
        Some(Commands::Init { path, name, vault_type: _ }) => {
            assert_eq!(path, PathBuf::from("/tmp/test"));
            assert_eq!(name, Some("test".to_string()));
        }
        _ => panic!("Expected Init command"),
    }
}

#[test]
fn test_cli_parse_add_command() {
    let cli = Cli::try_parse_from(&["journey", "add", "test note"]).unwrap();
    match cli.command {
        Some(Commands::Add { content }) => {
            assert_eq!(content, "test note");
        }
        _ => panic!("Expected Add command"),
    }
}

#[test]
fn test_cli_parse_list_command_subcommand() {
    let cli = Cli::try_parse_from(&["journey", "list"]).unwrap();
    match cli.command {
        Some(Commands::List) => {
            // Expected
        }
        _ => panic!("Expected List command"),
    }
}

#[test]
fn test_cli_parse_edit_command_subcommand() {
    let cli = Cli::try_parse_from(&["journey", "edit"]).unwrap();
    match cli.command {
        Some(Commands::Edit) => {
            // Expected
        }
        _ => panic!("Expected Edit command"),
    }
}

#[test]
fn test_cli_parse_stdin_flag() {
    let cli = Cli::try_parse_from(&["journey", "--stdin"]).unwrap();
    assert!(cli.stdin);
    assert!(cli.command.is_none());
    assert!(!cli.list);
    assert!(!cli.edit);
}

#[test]
fn test_cli_parse_stdin_with_time() {
    let cli = Cli::try_parse_from(&["journey", "--stdin", "--time", "14:30"]).unwrap();
    assert!(cli.stdin);
    assert_eq!(cli.time, Some("14:30".to_string()));
}

#[test]
fn test_cli_parse_stdin_with_date() {
    let cli = Cli::try_parse_from(&["journey", "--stdin", "--date", "2025-10-24"]).unwrap();
    assert!(cli.stdin);
    assert_eq!(cli.date, Some("2025-10-24".to_string()));
}

#[test]
fn test_cli_parse_stdin_with_vault() {
    let cli = Cli::try_parse_from(&["journey", "--stdin", "--vault", "test-vault"]).unwrap();
    assert!(cli.stdin);
    assert_eq!(cli.vault, Some("test-vault".to_string()));
}
