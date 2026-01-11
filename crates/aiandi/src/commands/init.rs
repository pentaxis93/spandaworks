//! `aiandi init` command implementation.
//!
//! Initializes a project for aiandi/OpenCode workflows by creating the
//! .opencode/ directory structure and extracting bundled skills.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use colored::Colorize;

use crate::skills::{bundled_skills, get_skill, skill_names, BundledSkill};

/// Standard .opencode/.gitignore content
const OPENCODE_GITIGNORE: &str = r#"node_modules
package.json
bun.lock
.gitignore
"#;

/// Options for the init command
#[derive(Debug, Clone)]
pub struct InitOptions {
    /// Comma-separated list of skills to install (None = all)
    pub skills: Option<Vec<String>>,
    /// Skip skill installation entirely
    pub no_skills: bool,
    /// Overwrite existing files
    pub force: bool,
    /// Show what would be created without creating
    pub dry_run: bool,
}

impl Default for InitOptions {
    fn default() -> Self {
        Self {
            skills: None,
            no_skills: false,
            force: false,
            dry_run: false,
        }
    }
}

/// Result of the init operation
#[derive(Debug, Default)]
pub struct InitResult {
    /// Number of items created
    pub created: usize,
    /// Number of items skipped (already exist)
    pub skipped: usize,
    /// Skills that were installed
    pub skills_installed: Vec<String>,
    /// Skills that were skipped
    pub skills_skipped: Vec<String>,
}

/// Run the init command with the given options
pub fn run_with_options(options: &InitOptions) -> Result<InitResult> {
    run_in_directory(Path::new("."), options)
}

/// Run the init command in a specific directory (for testing)
pub fn run_in_directory(base_path: &Path, options: &InitOptions) -> Result<InitResult> {
    let mut result = InitResult::default();

    let opencode_dir = base_path.join(".opencode");
    let gitignore_path = opencode_dir.join(".gitignore");
    let skill_dir = opencode_dir.join("skill");

    // Create .opencode/ directory
    if !opencode_dir.exists() {
        if options.dry_run {
            println!("{} Would create .opencode/", "[dry-run]".blue());
        } else {
            fs::create_dir_all(&opencode_dir)
                .with_context(|| format!("Failed to create {}", opencode_dir.display()))?;
            println!("Creating .opencode/ directory");
            result.created += 1;
        }
    } else {
        println!("{} .opencode/ already exists", "→".yellow());
        result.skipped += 1;
    }

    // Create .opencode/.gitignore
    if !gitignore_path.exists() {
        if options.dry_run {
            println!("{} Would create .opencode/.gitignore", "[dry-run]".blue());
        } else {
            fs::write(&gitignore_path, OPENCODE_GITIGNORE)
                .with_context(|| format!("Failed to create {}", gitignore_path.display()))?;
            println!("Creating .opencode/.gitignore");
            result.created += 1;
        }
    } else {
        println!("{} .opencode/.gitignore already exists", "→".yellow());
        result.skipped += 1;
    }

    // Handle skills
    if options.no_skills {
        println!("Skipping skill installation (--no-skills)");
    } else {
        // Create .opencode/skill/ directory
        if !skill_dir.exists() {
            if options.dry_run {
                println!("{} Would create .opencode/skill/", "[dry-run]".blue());
            } else {
                fs::create_dir_all(&skill_dir)
                    .with_context(|| format!("Failed to create {}", skill_dir.display()))?;
                println!("Creating .opencode/skill/ directory");
                result.created += 1;
            }
        } else {
            println!("{} .opencode/skill/ already exists", "→".yellow());
            result.skipped += 1;
        }

        // Determine which skills to install
        let skills_to_install = get_skills_to_install(options)?;

        // Extract each skill
        for skill in skills_to_install {
            let skill_path = skill_dir.join(&skill.name);
            let skill_file = skill_path.join("SKILL.md");

            let should_write = if skill_file.exists() {
                if options.force {
                    true
                } else {
                    println!(
                        "{} Skipped skill: {} (already exists, use --force to overwrite)",
                        "→".yellow(),
                        skill.name
                    );
                    result.skills_skipped.push(skill.name.to_string());
                    false
                }
            } else {
                true
            };

            if should_write {
                if options.dry_run {
                    println!(
                        "{} Would extract skill: {}",
                        "[dry-run]".blue(),
                        skill.name
                    );
                } else {
                    // Create skill directory
                    if !skill_path.exists() {
                        fs::create_dir_all(&skill_path).with_context(|| {
                            format!("Failed to create {}", skill_path.display())
                        })?;
                    }

                    // Write skill file
                    fs::write(&skill_file, skill.content)
                        .with_context(|| format!("Failed to write {}", skill_file.display()))?;

                    println!("{} Extracted skill: {}", "✓".green(), skill.name);
                    result.skills_installed.push(skill.name.to_string());
                    result.created += 1;
                }
            }
        }
    }

    // Print summary
    println!();
    if options.dry_run {
        println!("Dry run complete. No files created.");
    } else {
        let skill_summary = if options.no_skills {
            String::new()
        } else {
            let installed = result.skills_installed.len();
            let skipped = result.skills_skipped.len();
            if skipped > 0 {
                format!(
                    " {} skill{} installed, {} skipped.",
                    installed,
                    if installed == 1 { "" } else { "s" },
                    skipped
                )
            } else {
                format!(
                    " {} skill{} installed.",
                    installed,
                    if installed == 1 { "" } else { "s" }
                )
            }
        };
        println!("aiandi initialized.{}", skill_summary);
    }

    Ok(result)
}

/// Get the list of skills to install based on options
fn get_skills_to_install(options: &InitOptions) -> Result<Vec<BundledSkill>> {
    match &options.skills {
        Some(requested) => {
            let mut skills = Vec::new();
            let available = skill_names();

            for name in requested {
                let name = name.trim();
                if let Some(skill) = get_skill(name) {
                    skills.push(skill);
                } else {
                    anyhow::bail!(
                        "Unknown skill: '{}'. Available skills: {}",
                        name,
                        available.join(", ")
                    );
                }
            }
            Ok(skills)
        }
        None => Ok(bundled_skills()),
    }
}

/// CLI arguments for the init command
#[derive(Debug, Clone, clap::Args)]
pub struct Args {
    /// Comma-separated skills to install (default: all bundled)
    #[arg(long, value_delimiter = ',')]
    pub skills: Option<Vec<String>>,

    /// Skip skill installation
    #[arg(long)]
    pub no_skills: bool,

    /// Overwrite existing files
    #[arg(long)]
    pub force: bool,

    /// Show what would be created without creating
    #[arg(long)]
    pub dry_run: bool,
}

impl From<&Args> for InitOptions {
    fn from(args: &Args) -> Self {
        Self {
            skills: args.skills.clone(),
            no_skills: args.no_skills,
            force: args.force,
            dry_run: args.dry_run,
        }
    }
}

/// Run the init command from CLI arguments
pub fn run_from_args(args: &Args) {
    let options = InitOptions::from(args);
    if let Err(e) = run_with_options(&options) {
        eprintln!("{} {}", "Error:".red(), e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_temp_dir() -> TempDir {
        tempfile::tempdir().expect("Failed to create temp directory")
    }

    #[test]
    fn test_init_fresh_directory() {
        let temp = create_temp_dir();
        let options = InitOptions::default();

        let result = run_in_directory(temp.path(), &options).expect("Init should succeed");

        // Check directories exist
        assert!(temp.path().join(".opencode").exists());
        assert!(temp.path().join(".opencode/skill").exists());
        assert!(temp.path().join(".opencode/.gitignore").exists());

        // Check skills extracted
        assert!(temp.path().join(".opencode/skill/transmission/SKILL.md").exists());
        assert!(temp.path().join(".opencode/skill/gtd/SKILL.md").exists());

        // Check result
        assert!(result.skills_installed.contains(&"transmission".to_string()));
        assert!(result.skills_installed.contains(&"gtd".to_string()));
        assert!(result.skills_skipped.is_empty());
    }

    #[test]
    fn test_init_idempotent() {
        let temp = create_temp_dir();
        let options = InitOptions::default();

        // First run
        let result1 = run_in_directory(temp.path(), &options).expect("First init should succeed");
        assert_eq!(result1.skills_installed.len(), 2);

        // Second run (should skip existing)
        let result2 = run_in_directory(temp.path(), &options).expect("Second init should succeed");
        assert_eq!(result2.skills_installed.len(), 0);
        assert_eq!(result2.skills_skipped.len(), 2);
    }

    #[test]
    fn test_init_force_overwrites() {
        let temp = create_temp_dir();

        // First run without force
        let options1 = InitOptions::default();
        run_in_directory(temp.path(), &options1).expect("First init should succeed");

        // Modify a skill file
        let skill_file = temp.path().join(".opencode/skill/transmission/SKILL.md");
        fs::write(&skill_file, "modified content").expect("Should write");

        // Second run with force
        let options2 = InitOptions {
            force: true,
            ..Default::default()
        };
        let result = run_in_directory(temp.path(), &options2).expect("Force init should succeed");

        // Should have reinstalled both skills
        assert_eq!(result.skills_installed.len(), 2);
        assert!(result.skills_skipped.is_empty());

        // Content should be restored
        let content = fs::read_to_string(&skill_file).expect("Should read");
        assert!(content.contains("transmission"), "Should have original content");
    }

    #[test]
    fn test_init_no_skills() {
        let temp = create_temp_dir();
        let options = InitOptions {
            no_skills: true,
            ..Default::default()
        };

        let result = run_in_directory(temp.path(), &options).expect("Init should succeed");

        // .opencode should exist but not skill directory content
        assert!(temp.path().join(".opencode").exists());
        assert!(temp.path().join(".opencode/.gitignore").exists());
        // skill directory shouldn't be created when --no-skills
        assert!(!temp.path().join(".opencode/skill/transmission").exists());

        assert!(result.skills_installed.is_empty());
    }

    #[test]
    fn test_init_selective_skills() {
        let temp = create_temp_dir();
        let options = InitOptions {
            skills: Some(vec!["gtd".to_string()]),
            ..Default::default()
        };

        let result = run_in_directory(temp.path(), &options).expect("Init should succeed");

        // Only gtd should be installed
        assert!(temp.path().join(".opencode/skill/gtd/SKILL.md").exists());
        assert!(!temp.path().join(".opencode/skill/transmission/SKILL.md").exists());

        assert_eq!(result.skills_installed, vec!["gtd".to_string()]);
    }

    #[test]
    fn test_init_dry_run() {
        let temp = create_temp_dir();
        let options = InitOptions {
            dry_run: true,
            ..Default::default()
        };

        let _result = run_in_directory(temp.path(), &options).expect("Dry run should succeed");

        // Nothing should be created
        assert!(!temp.path().join(".opencode").exists());
    }

    #[test]
    fn test_init_invalid_skill() {
        let temp = create_temp_dir();
        let options = InitOptions {
            skills: Some(vec!["nonexistent".to_string()]),
            ..Default::default()
        };

        let result = run_in_directory(temp.path(), &options);
        assert!(result.is_err(), "Should fail with invalid skill");
        assert!(result.unwrap_err().to_string().contains("Unknown skill"));
    }
}
