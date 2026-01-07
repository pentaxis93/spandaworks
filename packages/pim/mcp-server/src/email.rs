//! Email tools - wraps notmuch (search/read) and himalaya (send)

use crate::cli::{run_command_stdout, run_command_with_stdin};

#[derive(Clone)]
pub struct EmailTools {
    himalaya_path: String,
}

impl Default for EmailTools {
    fn default() -> Self {
        Self::new()
    }
}

impl EmailTools {
    pub fn new() -> Self {
        Self {
            // himalaya is installed via pipx, not in system PATH
            himalaya_path: format!(
                "{}/.local/bin/himalaya",
                std::env::var("HOME").unwrap_or_default()
            ),
        }
    }

    /// Normalize thread ID - accept with or without "thread:" prefix
    pub fn normalize_thread_id(thread_id: &str) -> String {
        if thread_id.starts_with("thread:") {
            thread_id.to_string()
        } else {
            format!("thread:{}", thread_id)
        }
    }

    /// Search emails using notmuch query syntax
    pub async fn search(&self, query: String, limit: Option<u32>) -> String {
        let limit = limit.unwrap_or(20);
        let limit_str = limit.to_string();

        // Use notmuch search with structured output
        let args = vec!["search", "--limit", &limit_str, "--format=text", &query];

        match run_command_stdout("notmuch", &args).await {
            Ok(output) => {
                if output.trim().is_empty() {
                    format!("No emails found matching: {}", query)
                } else {
                    let lines: Vec<&str> = output.lines().collect();
                    let count = lines.len();
                    format!(
                        "Found {} email thread(s) matching '{}':\n\n{}",
                        count, query, output
                    )
                }
            }
            Err(e) => format!("Error searching emails: {}", e),
        }
    }

    /// Read the full content of an email thread
    pub async fn read_thread(&self, thread_id: String) -> String {
        let query = Self::normalize_thread_id(&thread_id);

        match run_command_stdout("notmuch", &["show", "--format=text", &query]).await {
            Ok(output) => {
                if output.trim().is_empty() {
                    format!("No email found with ID: {}", query)
                } else {
                    output
                }
            }
            Err(e) => format!("Error reading email: {}", e),
        }
    }

    /// Send an email (requires explicit confirmation)
    pub async fn send(&self, to: String, subject: String, body: String, confirm: bool) -> String {
        if !confirm {
            return "Email NOT sent. Set confirm=true to actually send the email.\n\n\
                    Preview:\n\
                    To: {}\n\
                    Subject: {}\n\
                    Body:\n{}"
                .replace("{}", &to)
                .replacen("{}", &subject, 1)
                .replacen("{}", &body, 1);
        }

        // Use himalaya to send
        // himalaya message write --to X --subject Y < body
        let args = vec!["message", "write", "--to", &to, "--subject", &subject];

        match run_command_with_stdin(&self.himalaya_path, &args, &body).await {
            Ok(output) => {
                format!(
                    "Email sent successfully!\n\
                     To: {}\n\
                     Subject: {}\n\
                     {}",
                    to, subject, output
                )
            }
            Err(e) => format!("Error sending email: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_thread_id_with_prefix() {
        let result = EmailTools::normalize_thread_id("thread:00000000000012ab");
        assert_eq!(result, "thread:00000000000012ab");
    }

    #[test]
    fn test_normalize_thread_id_without_prefix() {
        let result = EmailTools::normalize_thread_id("00000000000012ab");
        assert_eq!(result, "thread:00000000000012ab");
    }

    #[tokio::test]
    async fn test_send_without_confirmation() {
        let tools = EmailTools::new();
        let result = tools
            .send(
                "test@example.com".to_string(),
                "Test Subject".to_string(),
                "Test body".to_string(),
                false,
            )
            .await;

        assert!(result.contains("Email NOT sent"));
        assert!(result.contains("confirm=true"));
    }
}
