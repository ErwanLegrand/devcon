pub mod docker;
pub mod docker_compose;
pub mod podman;
pub mod podman_compose;
pub(crate) mod utils;

use colored::Colorize;
use std::io::Result;

/// Abstraction over container engine backends (Docker, Podman, Compose variants).
pub trait Provider {
    /// Build the container image. Pass `use_cache = false` to add `--no-cache`.
    ///
    /// # Errors
    /// Returns an error if the underlying build command fails to spawn or exits with an error.
    fn build(&self, use_cache: bool) -> Result<bool>;
    /// Create the container with the given extra arguments.
    ///
    /// # Errors
    /// Returns an error if the underlying create command fails to spawn or exits with an error.
    fn create(&self, args: Vec<String>) -> Result<bool>;
    /// Start a stopped container.
    ///
    /// # Errors
    /// Returns an error if the underlying start command fails to spawn or exits with an error.
    fn start(&self) -> Result<bool>;
    /// Stop a running container.
    ///
    /// # Errors
    /// Returns an error if the underlying stop command fails to spawn or exits with an error.
    fn stop(&self) -> Result<bool>;
    /// Restart the container.
    ///
    /// # Errors
    /// Returns an error if the underlying restart command fails to spawn or exits with an error.
    fn restart(&self) -> Result<bool>;
    /// Attach an interactive shell session to the container.
    ///
    /// # Errors
    /// Returns an error if the underlying attach command fails to spawn or exits with an error.
    fn attach(&self) -> Result<bool>;
    /// Remove the container.
    ///
    /// # Errors
    /// Returns an error if the underlying remove command fails to spawn or exits with an error.
    fn rm(&self) -> Result<bool>;
    /// Return `true` if the container exists (running or stopped).
    ///
    /// # Errors
    /// Returns an error if the underlying inspect command fails to spawn.
    fn exists(&self) -> Result<bool>;
    /// Return `true` if the container is currently running.
    ///
    /// # Errors
    /// Returns an error if the underlying inspect command fails to spawn.
    fn running(&self) -> Result<bool>;
    /// Copy `source` (host path) into the container at `destination`.
    ///
    /// # Errors
    /// Returns an error if the underlying copy command fails to spawn or exits with an error.
    fn cp(&self, source: String, destination: String) -> Result<bool>;
    /// Execute a shell command inside the container via `sh -c`.
    ///
    /// Commands from `devcontainer.json` lifecycle hooks are passed here and
    /// are expected to be shell syntax (pipes, redirects, etc. are supported).
    /// Callers constructing commands programmatically should shell-quote any
    /// path arguments to prevent word-splitting.
    ///
    /// # Errors
    /// Returns an error if the underlying exec command fails to spawn or exits with a non-zero
    /// status. The exit code is included in the error message.
    fn exec(&self, cmd: String) -> Result<()>;
    /// Execute a program directly inside the container without a shell wrapper.
    ///
    /// `prog` is the executable to run; `args` are its arguments passed as
    /// separate tokens (no shell interpretation, no word-splitting, no injection).
    /// Use this for the array form of lifecycle hooks (`Many` variant).
    ///
    /// # Errors
    /// Returns an error if the underlying exec command fails to spawn or exits with a non-zero
    /// status. The exit code is included in the error message.
    fn exec_raw(&self, prog: &str, args: &[&str]) -> Result<()>;
}

/// Redact the values of `--env` / `-e` arguments in a list of command-line tokens.
///
/// For two-token forms (`--env KEY=VALUE` or `-e KEY=VALUE`), the token following
/// the flag is replaced with `KEY=***`. For single-token forms (`--env=KEY=VALUE`),
/// the value after `=` is replaced with `***`.
pub(crate) fn redact_env_args(args: &[&str]) -> Vec<String> {
    let mut redacted: Vec<String> = Vec::with_capacity(args.len());
    let mut redact_next = false;

    for arg in args {
        if redact_next {
            // Previous arg was --env or -e; redact value in KEY=VALUE.
            let display = if let Some(eq) = arg.find('=') {
                format!("{}=***", &arg[..eq])
            } else {
                "***".to_string()
            };
            redacted.push(display);
            redact_next = false;
        } else if *arg == "--env" || *arg == "-e" {
            redacted.push((*arg).to_string());
            redact_next = true;
        } else if let Some(suffix) = arg.strip_prefix("--env=") {
            // --env=KEY=VALUE single-token form.
            let display = if let Some(eq) = suffix.find('=') {
                format!("--env={}=***", &suffix[..eq])
            } else {
                format!("--env={suffix}")
            };
            redacted.push(display);
        } else if let Some(suffix) = arg.strip_prefix("-e=") {
            let display = if let Some(eq) = suffix.find('=') {
                format!("-e={}=***", &suffix[..eq])
            } else {
                format!("-e={suffix}")
            };
            redacted.push(display);
        } else {
            redacted.push((*arg).to_string());
        }
    }

    redacted
}

/// Print a formatted, redacted command line to stdout.
///
/// `--env KEY=VALUE` values are replaced with `KEY=***` to avoid leaking secrets in terminal
/// output. The actual subprocess arguments are not modified.
pub(crate) fn print_command(command: &std::process::Command) {
    let exec = command.get_program();
    let raw_args: Vec<&str> = command
        .get_args()
        .map(|arg| arg.to_str().unwrap_or("<non-utf8>"))
        .collect();

    let redacted = redact_env_args(&raw_args);

    let output = format!(
        "{} {}",
        exec.to_str().unwrap_or("<non-utf8>"),
        redacted.join(" ")
    );
    println!("{}", output.bold().blue());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redact_two_token_env() {
        let args = vec!["run", "-e", "SECRET=hunter2", "--name", "c"];
        let result = redact_env_args(&args);
        assert_eq!(result, vec!["run", "-e", "SECRET=***", "--name", "c"]);
    }

    #[test]
    fn redact_two_token_env_long() {
        let args = vec!["run", "--env", "MY_TOKEN=abc123"];
        let result = redact_env_args(&args);
        assert_eq!(result, vec!["run", "--env", "MY_TOKEN=***"]);
    }

    #[test]
    fn redact_single_token_env_equals() {
        let args = vec!["run", "--env=MY_TOKEN=abc123"];
        let result = redact_env_args(&args);
        assert_eq!(result, vec!["run", "--env=MY_TOKEN=***"]);
    }

    #[test]
    fn non_env_args_unchanged() {
        let args = vec!["run", "--name", "foo", "--network", "host"];
        let result = redact_env_args(&args);
        assert_eq!(result, vec!["run", "--name", "foo", "--network", "host"]);
    }

    #[test]
    fn multiple_env_args_all_redacted() {
        let args = vec!["run", "-e", "A=1", "-e", "B=2"];
        let result = redact_env_args(&args);
        assert_eq!(result, vec!["run", "-e", "A=***", "-e", "B=***"]);
    }
}
