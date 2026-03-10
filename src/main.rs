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
        /// Suppress the warning when the container will run as root with no remoteUser configured.
        #[arg(long)]
        no_root_check: bool,
        /// Disable the structured audit log for this invocation.
        #[arg(long)]
        no_audit_log: bool,
        /// Override the per-hook timeout in seconds (overrides hookTimeoutSeconds in config).
        #[arg(long)]
        hook_timeout: Option<u32>,
    },
    Start {
        dir: Option<String>,
        /// Trust the devcontainer and skip the initializeCommand confirmation prompt.
        #[arg(long)]
        trust: bool,
        /// Suppress the warning when the container will run as root with no remoteUser configured.
        #[arg(long)]
        no_root_check: bool,
        /// Disable the structured audit log for this invocation.
        #[arg(long)]
        no_audit_log: bool,
        /// Override the per-hook timeout in seconds (overrides hookTimeoutSeconds in config).
        #[arg(long)]
        hook_timeout: Option<u32>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Start {
            dir,
            trust,
            no_root_check,
            no_audit_log,
            hook_timeout,
        }) => {
            commands::start::run(
                dir.as_deref(),
                *trust,
                *no_root_check,
                *no_audit_log,
                *hook_timeout,
            )?;
        }
        Some(Commands::Rebuild {
            dir,
            no_cache,
            trust,
            no_root_check,
            no_audit_log,
            hook_timeout,
        }) => {
            commands::rebuild::run(
                dir.as_deref(),
                !no_cache,
                *trust,
                *no_root_check,
                *no_audit_log,
                *hook_timeout,
            )?;
        }
        None => {
            commands::start::run(None, false, false, false, None)?;
        }
    }

    Ok(())
}
