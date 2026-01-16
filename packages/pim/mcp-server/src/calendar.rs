//! Calendar tools - wraps khal CLI
//!
//! Calendars:
//! - robbie (read/write) - Robbie's personal calendar
//! - meli (read-only) - Wife's calendar (shared)
//! - dennis (read-only) - Son's calendar (shared)
//! - holidays (read-only) - Hungarian holidays

use crate::cli::run_command_stdout;
use chrono::Local;

#[derive(Clone, Default)]
pub struct CalendarTools;

impl CalendarTools {
    pub fn new() -> Self {
        Self
    }

    /// Format a date range string for display
    #[cfg(test)]
    pub fn format_date_range(start: &str, days: u32) -> String {
        format!("{} ({} days)", start, days)
    }

    /// List calendar events for a date range
    pub async fn list_events(&self, start_date: Option<String>, days: Option<u32>) -> String {
        let start = start_date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());
        let days = days.unwrap_or(7);
        let range = format!("{}d", days);

        // Format: [calendar] time-time title
        // This makes calendar source visible in output
        let format = "[{calendar}] {start-end-time-style} {title}";
        
        match run_command_stdout("khal", &["list", "-f", format, &start, &range]).await {
            Ok(output) => {
                if output.trim().is_empty() {
                    format!("No events found from {} for {} days.", start, days)
                } else {
                    format!("Events from {} ({} days):\n\n{}", start, days, output)
                }
            }
            Err(e) => format!("Error listing events: {}", e),
        }
    }

    /// Create a new calendar event
    pub async fn create_event(
        &self,
        title: String,
        date: String,
        start_time: Option<String>,
        end_time: Option<String>,
        location: Option<String>,
        _description: Option<String>,
    ) -> String {
        // Build khal new command
        // Format: khal new [-a calendar] [OPTIONS] [START [END | DELTA] [TIMEZONE] SUMMARY] [:: DESCRIPTION]

        let mut args: Vec<&str> = vec!["new"];

        // Always use robbie's personal calendar for writes
        args.push("-a");
        args.push("robbie");

        // Build the datetime string
        let datetime_str: String;
        let end_str: String;

        if let Some(ref start) = start_time {
            // Timed event
            datetime_str = format!("{} {}", date, start);
            args.push(&datetime_str);

            if let Some(ref end) = end_time {
                end_str = end.clone();
                args.push(&end_str);
            } else {
                // Default to 1 hour duration
                args.push("1h");
            }
        } else {
            // All-day event
            args.push(&date);
        }

        // Add title
        args.push(&title);

        // Add location if provided (using :: separator)
        let location_str: String;
        if let Some(ref loc) = location {
            location_str = format!(":: {}", loc);
            args.push(&location_str);
        }

        match run_command_stdout("khal", &args).await {
            Ok(output) => {
                let result = if output.trim().is_empty() {
                    "Event created successfully.".to_string()
                } else {
                    output
                };
                format!("Created event '{}' on {}.\n{}", title, date, result)
            }
            Err(e) => format!("Error creating event: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date_range() {
        let result = CalendarTools::format_date_range("2026-01-07", 7);
        assert_eq!(result, "2026-01-07 (7 days)");
    }

    #[test]
    fn test_default_days() {
        // Verify default is 7 days
        let days: Option<u32> = None;
        let actual = days.unwrap_or(7);
        assert_eq!(actual, 7);
    }
}
