use clap::{Parser, Subcommand};
use anyhow::Result;

mod dump;
mod wipe;
mod save;

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
    Dump,
    /// Remove everything from the git repository including history (DESTRUCTIVE)
    Wipe {
        /// Also remove untracked files (only ignored files will remain)
        #[arg(short, long)]
        include_untracked: bool,
    },
    /// Save changes by committing and pushing to origin
    Save {
        /// Commit message (if not provided, will prompt for one)
        #[arg(short, long)]
        message: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dump => {
            dump::dump_changes()?;
        }
        Commands::Wipe { include_untracked } => {
            wipe::wipe_repository(include_untracked)?;
        }
        Commands::Save { message } => {
            save::save_changes(message)?;
        }
    }

    Ok(())
}