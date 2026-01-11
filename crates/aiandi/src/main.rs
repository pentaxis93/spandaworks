use clap::{Parser, Subcommand};

mod commands;
mod skills;

use commands::{doctor, inbox, init, serve};

#[derive(Parser)]
#[command(name = "aiandi")]
#[command(about = "Infrastructure for AI-human collaboration", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Capture items to GTD inbox
    Inbox(inbox::Args),
    /// Initialize aiandi for OpenCode
    Init(init::Args),
    /// Start MCP server for OpenCode integration
    Serve(serve::Args),
    /// Check aiandi installation and configuration
    Doctor,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Inbox(args) => inbox::run_from_args(args),
        Commands::Init(args) => init::run_from_args(args),
        Commands::Serve(args) => {
            if let Err(e) = serve::run_from_args(args) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Doctor => doctor::run(),
    }
}
