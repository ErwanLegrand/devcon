use std::path::Path;
use std::path::PathBuf;

use devcont::devcontainers::Devcontainer;

/// Stop, remove, and restart the dev container in `dir` (or the current directory when `None`).
///
/// When `use_cache` is `true` the Docker layer cache is used; pass `false` to force a clean
/// image rebuild.
///
/// # Errors
/// Returns an error if the directory cannot be resolved, the devcontainer config cannot be
/// loaded, or any lifecycle operation fails.
pub fn run(
    dir: Option<&str>,
    use_cache: bool,
    trust: bool,
    no_root_check: bool,
) -> std::io::Result<()> {
    let directory = get_project_directory(dir)?;
    let devcontainer = Devcontainer::load(&directory)?;
    devcontainer.rebuild(use_cache, trust, no_root_check)?;

    Ok(())
}

fn get_project_directory(dir: Option<&str>) -> std::io::Result<PathBuf> {
    if let Some(path) = dir {
        let expanded = shellexpand::env(path).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Could not expand dir '{path}': {e}"),
            )
        })?;

        Path::new(expanded.as_ref()).canonicalize()
    } else {
        std::env::current_dir()
    }
}
