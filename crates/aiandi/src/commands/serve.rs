//! `aiandi serve` command implementation.
//!
//! MCP server for OpenCode integration.
//! Currently a scaffold that will be expanded with actual MCP protocol support.

use colored::Colorize;

/// Default port for the server
pub const DEFAULT_PORT: u16 = 3000;

/// Default host to bind to
pub const DEFAULT_HOST: &str = "127.0.0.1";

/// CLI arguments for the serve command
#[derive(Debug, Clone, clap::Args)]
pub struct Args {
    /// Port to listen on
    #[arg(long, short = 'p', default_value_t = DEFAULT_PORT)]
    pub port: u16,

    /// Host to bind to
    #[arg(long, short = 'H', default_value = DEFAULT_HOST)]
    pub host: String,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            port: DEFAULT_PORT,
            host: DEFAULT_HOST.to_string(),
        }
    }
}

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl From<&Args> for ServerConfig {
    fn from(args: &Args) -> Self {
        Self {
            host: args.host.clone(),
            port: args.port,
        }
    }
}

impl ServerConfig {
    /// Get the bind address
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Start the MCP server (currently a scaffold)
pub fn start_server(config: &ServerConfig) {
    println!(
        "{} Starting MCP server on {}",
        "→".blue(),
        config.bind_address().cyan()
    );
    println!();
    println!("MCP server scaffold ready.");
    println!("Full protocol support coming in future release.");
    println!();
    println!("Press {} to stop.", "Ctrl+C".bold());

    // For now, just wait indefinitely
    // In the future, this will start an actual MCP server
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

/// Run the serve command from CLI arguments
pub fn run_from_args(args: &Args) {
    let config = ServerConfig::from(args);
    start_server(&config);
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===================
    // TDD: Tests for serve command
    // ===================

    #[test]
    fn test_default_port() {
        let args = Args::default();
        assert_eq!(args.port, 3000);
    }

    #[test]
    fn test_default_host() {
        let args = Args::default();
        assert_eq!(args.host, "127.0.0.1");
    }

    #[test]
    fn test_server_config_from_args() {
        let args = Args {
            port: 8080,
            host: "0.0.0.0".to_string(),
        };
        let config = ServerConfig::from(&args);

        assert_eq!(config.port, 8080);
        assert_eq!(config.host, "0.0.0.0");
    }

    #[test]
    fn test_bind_address_format() {
        let config = ServerConfig {
            host: "localhost".to_string(),
            port: 9000,
        };

        assert_eq!(config.bind_address(), "localhost:9000");
    }

    #[test]
    fn test_bind_address_default() {
        let args = Args::default();
        let config = ServerConfig::from(&args);

        assert_eq!(config.bind_address(), "127.0.0.1:3000");
    }
}
