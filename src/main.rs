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
    },
    Start {
        dir: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Start { dir }) => {
            commands::start::run(dir.as_deref())?;
        }
        Some(Commands::Rebuild { dir, no_cache }) => {
            commands::rebuild::run(dir.as_deref(), !no_cache)?;
        }
        None => {
            commands::start::run(None)?;
        }
    }

    Ok(())
}
