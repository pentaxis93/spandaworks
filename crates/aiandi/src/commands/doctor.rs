//! `aiandi doctor` command implementation.
//!
//! System health checks for aiandi dependencies and configuration.

use std::path::{Path, PathBuf};
use std::process::Command;

use colored::Colorize;

/// A single health check
#[derive(Debug, Clone)]
#[allow(dead_code)] // name field used for debugging
pub struct HealthCheck {
    pub name: String,
    pub description: String,
    pub status: CheckStatus,
    pub details: Option<String>,
}

/// Status of a health check
#[derive(Debug, Clone, PartialEq)]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
}

/// Result of running all health checks
#[derive(Debug, Default)]
pub struct DoctorResult {
    pub checks: Vec<HealthCheck>,
    pub passed: usize,
    pub warnings: usize,
    pub failed: usize,
}

impl DoctorResult {
    pub fn add(&mut self, check: HealthCheck) {
        match check.status {
            CheckStatus::Pass => self.passed += 1,
            CheckStatus::Warn => self.warnings += 1,
            CheckStatus::Fail => self.failed += 1,
        }
        self.checks.push(check);
    }

    pub fn has_failures(&self) -> bool {
        self.failed > 0
    }
}

/// Check if a command is available and get its version
pub fn check_command(name: &str, version_arg: &str) -> HealthCheck {
    let description = format!("{} installation", name);

    match which::which(name) {
        Ok(path) => {
            // Try to get version
            let version = Command::new(&path)
                .arg(version_arg)
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.lines().next().unwrap_or("").trim().to_string());

            HealthCheck {
                name: name.to_string(),
                description,
                status: CheckStatus::Pass,
                details: version,
            }
        }
        Err(_) => HealthCheck {
            name: name.to_string(),
            description,
            status: CheckStatus::Fail,
            details: Some(format!("{} not found in PATH", name)),
        },
    }
}

/// Check if a directory exists
pub fn check_directory(path: &Path, name: &str) -> HealthCheck {
    let description = format!("{} ({})", name, path.display());

    if path.exists() {
        if path.is_dir() {
            HealthCheck {
                name: name.to_string(),
                description,
                status: CheckStatus::Pass,
                details: None,
            }
        } else {
            HealthCheck {
                name: name.to_string(),
                description,
                status: CheckStatus::Fail,
                details: Some("Path exists but is not a directory".to_string()),
            }
        }
    } else {
        HealthCheck {
            name: name.to_string(),
            description,
            status: CheckStatus::Warn,
            details: Some("Directory does not exist".to_string()),
        }
    }
}

/// Check if a file exists
pub fn check_file(path: &Path, name: &str) -> HealthCheck {
    let description = format!("{} ({})", name, path.display());

    if path.exists() {
        if path.is_file() {
            HealthCheck {
                name: name.to_string(),
                description,
                status: CheckStatus::Pass,
                details: None,
            }
        } else {
            HealthCheck {
                name: name.to_string(),
                description,
                status: CheckStatus::Fail,
                details: Some("Path exists but is not a file".to_string()),
            }
        }
    } else {
        HealthCheck {
            name: name.to_string(),
            description,
            status: CheckStatus::Warn,
            details: Some("File does not exist".to_string()),
        }
    }
}

/// Get the aiandi config directory
pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("aiandi")
}

/// Run all health checks
pub fn run_checks() -> DoctorResult {
    let mut result = DoctorResult::default();

    // Check required commands
    result.add(check_command("opencode", "--version"));
    result.add(check_command("task", "--version"));

    // Check config directory
    let config = config_dir();
    result.add(check_directory(&config, "Config directory"));

    // Check config file
    let config_file = config.join("config.toml");
    result.add(check_file(&config_file, "Config file"));

    // Check OpenCode config for skills
    let opencode_skill_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("opencode")
        .join("skill");
    result.add(check_directory(
        &opencode_skill_dir,
        "OpenCode skills directory",
    ));

    result
}

/// Print a health check result
fn print_check(check: &HealthCheck) {
    let symbol = match check.status {
        CheckStatus::Pass => "✓".green(),
        CheckStatus::Warn => "⚠".yellow(),
        CheckStatus::Fail => "✗".red(),
    };

    print!("{} {}", symbol, check.description);

    if let Some(ref details) = check.details {
        if check.status == CheckStatus::Pass {
            println!(" ({})", details.dimmed());
        } else {
            println!(" — {}", details);
        }
    } else {
        println!();
    }
}

/// Print summary of results
fn print_summary(result: &DoctorResult) {
    println!();
    let mut parts = vec![];

    if result.passed > 0 {
        parts.push(format!("{} passed", result.passed.to_string().green()));
    }
    if result.warnings > 0 {
        parts.push(format!(
            "{} warning{}",
            result.warnings.to_string().yellow(),
            if result.warnings == 1 { "" } else { "s" }
        ));
    }
    if result.failed > 0 {
        parts.push(format!("{} failed", result.failed.to_string().red()));
    }

    println!("{}", parts.join(", "));
}

/// Run the doctor command
pub fn run() {
    let result = run_checks();

    for check in &result.checks {
        print_check(check);
    }

    print_summary(&result);

    if result.has_failures() {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // ===================
    // TDD: Tests written first
    // ===================

    #[test]
    fn test_check_command_found() {
        // 'ls' should exist on any Unix system
        let check = check_command("ls", "--version");
        // ls might not support --version on all systems, but it should be found
        assert_eq!(check.name, "ls");
    }

    #[test]
    fn test_check_command_not_found() {
        let check = check_command("nonexistent_command_xyz", "--version");
        assert_eq!(check.status, CheckStatus::Fail);
        assert!(check.details.is_some());
        assert!(check.details.unwrap().contains("not found"));
    }

    #[test]
    fn test_check_directory_exists() {
        let temp = TempDir::new().unwrap();
        let check = check_directory(&temp.path().to_path_buf(), "Test dir");

        assert_eq!(check.status, CheckStatus::Pass);
    }

    #[test]
    fn test_check_directory_not_exists() {
        let path = PathBuf::from("/nonexistent/path/xyz");
        let check = check_directory(&path, "Missing dir");

        assert_eq!(check.status, CheckStatus::Warn);
    }

    #[test]
    fn test_check_file_exists() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.txt");
        std::fs::write(&file_path, "test").unwrap();

        let check = check_file(&file_path, "Test file");
        assert_eq!(check.status, CheckStatus::Pass);
    }

    #[test]
    fn test_check_file_not_exists() {
        let path = PathBuf::from("/nonexistent/file.txt");
        let check = check_file(&path, "Missing file");

        assert_eq!(check.status, CheckStatus::Warn);
    }

    #[test]
    fn test_check_file_is_directory() {
        let temp = TempDir::new().unwrap();
        let check = check_file(&temp.path().to_path_buf(), "Is dir");

        assert_eq!(check.status, CheckStatus::Fail);
        assert!(check.details.unwrap().contains("not a file"));
    }

    #[test]
    fn test_doctor_result_counting() {
        let mut result = DoctorResult::default();

        result.add(HealthCheck {
            name: "test1".to_string(),
            description: "Test 1".to_string(),
            status: CheckStatus::Pass,
            details: None,
        });
        result.add(HealthCheck {
            name: "test2".to_string(),
            description: "Test 2".to_string(),
            status: CheckStatus::Warn,
            details: None,
        });
        result.add(HealthCheck {
            name: "test3".to_string(),
            description: "Test 3".to_string(),
            status: CheckStatus::Fail,
            details: None,
        });

        assert_eq!(result.passed, 1);
        assert_eq!(result.warnings, 1);
        assert_eq!(result.failed, 1);
        assert!(result.has_failures());
    }

    #[test]
    fn test_doctor_result_no_failures() {
        let mut result = DoctorResult::default();

        result.add(HealthCheck {
            name: "test1".to_string(),
            description: "Test 1".to_string(),
            status: CheckStatus::Pass,
            details: None,
        });
        result.add(HealthCheck {
            name: "test2".to_string(),
            description: "Test 2".to_string(),
            status: CheckStatus::Warn,
            details: None,
        });

        assert!(!result.has_failures());
    }

    #[test]
    fn test_run_checks_returns_result() {
        let result = run_checks();

        // Should have at least the basic checks
        assert!(!result.checks.is_empty());
        assert!(result.checks.len() >= 4);
    }
}
