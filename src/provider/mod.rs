pub mod docker;
pub mod docker_compose;
pub mod podman;
pub mod podman_compose;

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
    /// Returns an error if the underlying exec command fails to spawn or exits with an error.
    fn exec(&self, cmd: String) -> Result<bool>;
}

pub fn print_command(command: &std::process::Command) {
    let exec = command.get_program();
    let args: Vec<&str> = command
        .get_args()
        .map(|arg| arg.to_str().unwrap_or("<non-utf8>"))
        .collect();

    let output = format!(
        "{} {}",
        exec.to_str().unwrap_or("<non-utf8>"),
        args.join(" ")
    );
    println!("{}", output.bold().blue());
}
