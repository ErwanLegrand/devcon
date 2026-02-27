pub(crate) mod docker;
pub(crate) mod docker_compose;
pub(crate) mod podman;
pub(crate) mod podman_compose;

use colored::Colorize;
use std::io::Result;

/// Abstraction over container engine backends (Docker, Podman, Compose variants).
pub(crate) trait Provider {
    /// Build the container image. Pass `use_cache = false` to add `--no-cache`.
    fn build(&self, use_cache: bool) -> Result<bool>;
    /// Create the container with the given extra arguments.
    fn create(&self, args: Vec<String>) -> Result<bool>;
    /// Start a stopped container.
    fn start(&self) -> Result<bool>;
    /// Stop a running container.
    fn stop(&self) -> Result<bool>;
    /// Restart the container.
    fn restart(&self) -> Result<bool>;
    /// Attach an interactive shell session to the container.
    fn attach(&self) -> Result<bool>;
    /// Remove the container.
    fn rm(&self) -> Result<bool>;
    /// Return `true` if the container exists (running or stopped).
    fn exists(&self) -> Result<bool>;
    /// Return `true` if the container is currently running.
    fn running(&self) -> Result<bool>;
    /// Copy `source` (host path) into the container at `destination`.
    fn cp(&self, source: String, destination: String) -> Result<bool>;
    /// Execute a shell command inside the container via `sh -c`.
    ///
    /// Commands from `devcontainer.json` lifecycle hooks are passed here and
    /// are expected to be shell syntax (pipes, redirects, etc. are supported).
    /// Callers constructing commands programmatically should shell-quote any
    /// path arguments to prevent word-splitting.
    fn exec(&self, cmd: String) -> Result<bool>;
}

pub(crate) fn print_command(command: &std::process::Command) {
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
