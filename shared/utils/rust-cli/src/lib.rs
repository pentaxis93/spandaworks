//! # spanda-cli
//!
//! Shared CLI command execution utilities for Spanda Works Rust packages.
//!
//! Provides consistent error handling, output capture, and stdin piping
//! for wrapping external CLI tools in async Rust code.
//!
//! ## Features
//!
//! - **Async execution** via tokio::process::Command
//! - **Structured error handling** with anyhow context
//! - **Stdout/stderr capture** with UTF-8 conversion
//! - **Stdin piping** for interactive commands
//! - **Lossy execution** for commands that use stderr for info
//! - **Environment variable support**
//! - **Working directory control**
//!
//! ## Example
//!
//! ```rust,no_run
//! use spanda_cli::{run_command_stdout, CommandBuilder};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Simple execution
//!     let output = run_command_stdout("ls", &["-la"]).await?;
//!     println!("{}", output);
//!
//!     // Advanced usage with builder
//!     let result = CommandBuilder::new("grep")
//!         .args(&["pattern", "file.txt"])
//!         .env("GREP_COLOR", "always")
//!         .current_dir("/tmp")
//!         .run_stdout()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

use anyhow::{Context, Result};
use std::process::Output;
use tokio::process::Command;

/// Execute a CLI command and return its full output (stdout, stderr, status).
///
/// Does NOT check exit status - returns raw Output for manual handling.
///
/// # Example
///
/// ```rust,no_run
/// # use spanda_cli::run_command;
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let output = run_command("git", &["status"]).await?;
/// if output.status.success() {
///     println!("{}", String::from_utf8_lossy(&output.stdout));
/// }
/// # Ok(())
/// # }
/// ```
pub async fn run_command(program: &str, args: &[&str]) -> Result<Output> {
    Command::new(program)
        .args(args)
        .output()
        .await
        .with_context(|| format!("Failed to execute: {} {}", program, args.join(" ")))
}

/// Execute a CLI command and return stdout as a String.
///
/// Returns an error if:
/// - Command fails to execute
/// - Exit code is non-zero
/// - Stdout is not valid UTF-8
///
/// # Example
///
/// ```rust,no_run
/// # use spanda_cli::run_command_stdout;
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let files = run_command_stdout("ls", &["-1"]).await?;
/// for file in files.lines() {
///     println!("Found: {}", file);
/// }
/// # Ok(())
/// # }
/// ```
pub async fn run_command_stdout(program: &str, args: &[&str]) -> Result<String> {
    let output = run_command(program, args).await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "Command failed: {} {}\nExit code: {:?}\nStderr: {}",
            program,
            args.join(" "),
            output.status.code(),
            stderr
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Execute a CLI command, returning stdout even if it fails.
///
/// Useful for commands that write informational messages to stderr
/// but still produce valid stdout.
///
/// On error, returns a formatted error string instead of propagating.
///
/// # Example
///
/// ```rust,no_run
/// # use spanda_cli::run_command_stdout_lossy;
/// # #[tokio::main]
/// # async fn main() {
/// // grep returns exit code 1 when no matches found, but that's not an error
/// let result = run_command_stdout_lossy("grep", &["pattern", "file.txt"]).await;
/// println!("{}", result);
/// # }
/// ```
pub async fn run_command_stdout_lossy(program: &str, args: &[&str]) -> String {
    match run_command(program, args).await {
        Ok(output) => String::from_utf8_lossy(&output.stdout).into_owned(),
        Err(e) => format!("Error: {}", e),
    }
}

/// Pipe input to a command's stdin and return stdout.
///
/// Spawns the command with piped stdin/stdout/stderr, writes `stdin_data`,
/// closes stdin, then waits for completion.
///
/// Returns an error if:
/// - Command fails to spawn
/// - Writing to stdin fails
/// - Exit code is non-zero
/// - Stdout is not valid UTF-8
///
/// # Example
///
/// ```rust,no_run
/// # use spanda_cli::run_command_with_stdin;
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let email_body = "Hello from Spanda Works!";
/// let result = run_command_with_stdin(
///     "mail",
///     &["-s", "Test Subject", "user@example.com"],
///     email_body
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub async fn run_command_with_stdin(
    program: &str,
    args: &[&str],
    stdin_data: &str,
) -> Result<String> {
    use tokio::io::AsyncWriteExt;

    let mut child = Command::new(program)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to spawn: {} {}", program, args.join(" ")))?;

    // Write to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(stdin_data.as_bytes())
            .await
            .context("Failed to write to stdin")?;
        // stdin is dropped here, closing the pipe
    }

    let output = child
        .wait_with_output()
        .await
        .context("Failed to wait for command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "Command failed: {} {}\nStderr: {}",
            program,
            args.join(" "),
            stderr
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Builder for advanced command execution with environment variables and working directory.
///
/// # Example
///
/// ```rust,no_run
/// # use spanda_cli::CommandBuilder;
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let output = CommandBuilder::new("cargo")
///     .args(&["build", "--release"])
///     .env("RUSTFLAGS", "-C target-cpu=native")
///     .current_dir("./my-project")
///     .run_stdout()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct CommandBuilder {
    program: String,
    args: Vec<String>,
    envs: Vec<(String, String)>,
    cwd: Option<String>,
}

impl CommandBuilder {
    /// Create a new CommandBuilder for the given program.
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            envs: Vec::new(),
            cwd: None,
        }
    }

    /// Add command-line arguments.
    pub fn args(mut self, args: &[&str]) -> Self {
        self.args.extend(args.iter().map(|s| s.to_string()));
        self
    }

    /// Add a single argument.
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Set an environment variable.
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.envs.push((key.into(), value.into()));
        self
    }

    /// Set the current working directory.
    pub fn current_dir(mut self, dir: impl Into<String>) -> Self {
        self.cwd = Some(dir.into());
        self
    }

    /// Execute and return stdout as String (fails on non-zero exit).
    pub async fn run_stdout(self) -> Result<String> {
        let program = self.program.clone();
        let args = self.args.clone();
        let output = self.run().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!(
                "Command failed: {} {}\nExit code: {:?}\nStderr: {}",
                program,
                args.join(" "),
                output.status.code(),
                stderr
            );
        }

        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// Execute and return full Output (does not check exit status).
    pub async fn run(self) -> Result<Output> {
        let mut cmd = Command::new(&self.program);
        cmd.args(&self.args);

        for (key, value) in &self.envs {
            cmd.env(key, value);
        }

        if let Some(cwd) = &self.cwd {
            cmd.current_dir(cwd);
        }

        cmd.output()
            .await
            .with_context(|| format!("Failed to execute: {} {}", self.program, self.args.join(" ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_command_success() {
        let output = run_command("echo", &["hello"]).await;
        assert!(output.is_ok());
        let output = output.unwrap();
        assert!(output.status.success());
    }

    #[tokio::test]
    async fn test_run_command_stdout() {
        let result = run_command_stdout("echo", &["hello"]).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "hello");
    }

    #[tokio::test]
    async fn test_run_command_stdout_failure() {
        // Try to run a command that doesn't exist
        let result = run_command_stdout("nonexistent_command_xyz", &[]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_run_command_with_stdin() {
        let result = run_command_with_stdin("cat", &[], "test input").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test input");
    }

    #[tokio::test]
    async fn test_command_builder() {
        let result = CommandBuilder::new("echo")
            .args(&["builder", "test"])
            .run_stdout()
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "builder test");
    }

    #[tokio::test]
    async fn test_command_builder_with_env() {
        let result = CommandBuilder::new("sh")
            .args(&["-c", "echo $TEST_VAR"])
            .env("TEST_VAR", "environment_value")
            .run_stdout()
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "environment_value");
    }
}
