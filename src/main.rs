use clap::{Parser, Subcommand};
use anyhow::Result;

mod dump;

#[derive(Parser)]
#[command(name = "shtt")]
#[command(about = "Simple History Tracking Tool")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all changes in the current working directory
    Dump {
        /// Show only modified files (no untracked files)
        #[arg(short, long)]
        modified_only: bool,
        
        /// Use porcelain format for machine-readable output
        #[arg(short, long)]
        porcelain: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dump { modified_only, porcelain } => {
            dump::dump_changes(modified_only, porcelain)?;
        }
    }

    Ok(())
}