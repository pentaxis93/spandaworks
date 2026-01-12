//! Integration tests for aiandi CLI
//!
//! These tests verify the full command-line interface works correctly.
//! Tests marked with #[ignore] require external dependencies (TaskWarrior, etc.)

use std::process::Command;

/// Helper to run aiandi command
fn aiandi(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_aiandi"))
        .args(args)
        .output()
        .expect("Failed to execute aiandi")
}

/// Helper to get stdout as string
fn stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Helper to get stderr as string  
fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

mod cli_help {
    use super::*;

    #[test]
    fn test_main_help() {
        let output = aiandi(&["--help"]);
        assert!(output.status.success());

        let out = stdout(&output);
        assert!(out.contains("aiandi"));
        assert!(out.contains("inbox"));
        assert!(out.contains("init"));
        assert!(out.contains("serve"));
        assert!(out.contains("doctor"));
    }

    #[test]
    fn test_init_help() {
        let output = aiandi(&["init", "--help"]);
        assert!(output.status.success());

        let out = stdout(&output);
        assert!(out.contains("--skills"));
        assert!(out.contains("--no-skills"));
        assert!(out.contains("--force"));
        assert!(out.contains("--dry-run"));
    }

    #[test]
    fn test_inbox_help() {
        let output = aiandi(&["inbox", "--help"]);
        assert!(output.status.success());

        let out = stdout(&output);
        assert!(out.contains("--tag"));
        assert!(out.contains("--project"));
        assert!(out.contains("--dry-run"));
    }

    #[test]
    fn test_serve_help() {
        let output = aiandi(&["serve", "--help"]);
        assert!(output.status.success());

        let out = stdout(&output);
        // MCP server runs on stdio, not HTTP
        assert!(out.contains("Start MCP server") || out.contains("serve"));
    }

    #[test]
    fn test_version() {
        let output = aiandi(&["--version"]);
        assert!(output.status.success());

        let out = stdout(&output);
        assert!(out.contains("aiandi"));
    }
}

mod init_command {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_creates_structure() {
        let temp = TempDir::new().unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_aiandi"))
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .expect("Failed to execute aiandi init");

        assert!(output.status.success());
        assert!(temp.path().join(".opencode").exists());
        assert!(temp
            .path()
            .join(".opencode/skill/transmission/SKILL.md")
            .exists());
        assert!(temp.path().join(".opencode/skill/gtd/SKILL.md").exists());
    }

    #[test]
    fn test_init_dry_run() {
        let temp = TempDir::new().unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_aiandi"))
            .args(["init", "--dry-run"])
            .current_dir(temp.path())
            .output()
            .expect("Failed to execute aiandi init");

        assert!(output.status.success());
        // Nothing should be created
        assert!(!temp.path().join(".opencode").exists());

        let out = stdout(&output);
        assert!(out.contains("dry-run"));
    }

    #[test]
    fn test_init_selective_skills() {
        let temp = TempDir::new().unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_aiandi"))
            .args(["init", "--skills", "gtd"])
            .current_dir(temp.path())
            .output()
            .expect("Failed to execute aiandi init");

        assert!(output.status.success());
        assert!(temp.path().join(".opencode/skill/gtd/SKILL.md").exists());
        assert!(!temp.path().join(".opencode/skill/transmission").exists());
    }
}

mod doctor_command {
    use super::*;

    #[test]
    fn test_doctor_runs() {
        let output = aiandi(&["doctor"]);
        // Doctor should run even if some checks fail
        // It will exit 1 if any check fails, but should still produce output

        let out = stdout(&output);
        // Should have some output about checks
        assert!(
            out.contains("OpenCode") || out.contains("TaskWarrior") || out.contains("Config"),
            "Doctor should report on system checks"
        );
    }
}

mod inbox_command {
    use super::*;

    #[test]
    fn test_inbox_empty_text_fails() {
        // This should fail because empty text is not allowed
        let output = aiandi(&["inbox", ""]);

        assert!(!output.status.success());
        let err = stderr(&output);
        assert!(err.contains("empty") || err.contains("Error"));
    }

    #[test]
    #[ignore] // Requires TaskWarrior
    fn test_inbox_captures_task() {
        let output = aiandi(&["inbox", "Integration test task"]);

        if output.status.success() {
            let out = stdout(&output);
            assert!(out.contains("Captured") || out.contains("✓"));
        } else {
            // If TaskWarrior isn't installed, this is expected to fail
            let err = stderr(&output);
            assert!(err.contains("TaskWarrior") || err.contains("not found"));
        }
    }

    #[test]
    #[ignore] // Requires TaskWarrior
    fn test_inbox_dry_run() {
        let output = aiandi(&["inbox", "--dry-run", "Dry run test"]);

        // Dry run should succeed if TaskWarrior is installed
        if output.status.success() {
            let out = stdout(&output);
            assert!(out.contains("dry-run"));
            assert!(out.contains("Would run"));
        }
    }
}
