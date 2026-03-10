#![warn(clippy::pedantic)]

use clap::Parser;
use clap::Subcommand;

pub(crate) mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Rebuild {
        dir: Option<String>,
        #[arg(short, long)]
        no_cache: bool,
        /// Trust the devcontainer and skip the initializeCommand confirmation prompt.
        #[arg(long)]
        trust: bool,
    },
    Start {
        dir: Option<String>,
        /// Trust the devcontainer and skip the initializeCommand confirmation prompt.
        #[arg(long)]
        trust: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Start { dir, trust }) => {
            commands::start::run(dir.as_deref(), *trust)?;
        }
        Some(Commands::Rebuild {
            dir,
            no_cache,
            trust,
        }) => {
            commands::rebuild::run(dir.as_deref(), !no_cache, *trust)?;
        }
        None => {
            commands::start::run(None, false)?;
        }
    }

    Ok(())
}
