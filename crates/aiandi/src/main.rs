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
    Inbox {
        /// Item to capture
        item: String,
    },
    /// Initialize aiandi for OpenCode
    Init(init::Args),
    /// Start HTTP server for inbox capture
    Serve,
    /// Check aiandi installation and configuration
    Doctor,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Inbox { item } => inbox::run(item),
        Commands::Init(args) => init::run_from_args(args),
        Commands::Serve => serve::run(),
        Commands::Doctor => doctor::run(),
    }
}
