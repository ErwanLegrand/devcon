use std::path::Path;
use std::path::PathBuf;

use devcont::devcontainers::Devcontainer;

pub fn run(dir: Option<&str>, trust: bool) -> std::io::Result<()> {
    let directory = get_project_directory(dir)?;
    let devcontainer = Devcontainer::load(&directory)?;
    devcontainer.run(true, trust)?;

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
