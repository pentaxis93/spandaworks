//! CLI command execution utilities
//!
//! Wraps tokio::process::Command with consistent error handling and output capture.

use anyhow::{Context, Result};
use std::process::Output;
use tokio::process::Command;

/// Execute a CLI command and return its output
pub async fn run_command(program: &str, args: &[&str]) -> Result<Output> {
    Command::new(program)
        .args(args)
        .output()
        .await
        .with_context(|| format!("Failed to execute: {} {}", program, args.join(" ")))
}

/// Execute a CLI command and return stdout as a String
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

/// Execute a CLI command, returning stdout even if it fails (for commands that use stderr for info)
#[allow(dead_code)]
pub async fn run_command_stdout_lossy(program: &str, args: &[&str]) -> String {
    match run_command(program, args).await {
        Ok(output) => String::from_utf8_lossy(&output.stdout).into_owned(),
        Err(e) => format!("Error: {}", e),
    }
}

/// Pipe input to a command's stdin
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
        stdin.write_all(stdin_data.as_bytes()).await?;
        // stdin is dropped here, closing the pipe
    }

    let output = child.wait_with_output().await?;

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
