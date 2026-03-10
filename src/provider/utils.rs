use serde::Serialize;
use std::env;
use std::io::Result;
use std::path::PathBuf;
use tinytemplate::TinyTemplate;

#[derive(Serialize, Debug)]
struct TemplateContext {
    service: String,
    envs: Vec<TemplateEntry>,
    volumes: Vec<TemplateEntry>,
}

#[derive(Serialize, Debug)]
struct TemplateEntry {
    source: String,
    dest: String,
}

static TEMPLATE: &str = include_str!("../../templates/docker-compose.yml");

/// RAII guard that deletes the compose override file when dropped.
pub(crate) struct ComposeOverrideGuard(pub(crate) PathBuf);

impl Drop for ComposeOverrideGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// Write a temporary docker-compose override file that forwards the SSH agent
/// socket into the named `service` container and injects any additional
/// environment variables, returning a guard that holds the path and deletes
/// the file on drop.
///
/// The file is created with mode 0o600 (owner read/write only).
///
/// # Errors
/// Returns an error if the template cannot be rendered or the file cannot be written.
pub(crate) fn create_compose_override(
    service: &str,
    env_vars: &[(String, String)],
) -> Result<ComposeOverrideGuard> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    use std::os::unix::fs::PermissionsExt;

    let dir = env::temp_dir();
    let path = dir.join("docker-compose.yml");
    let mut volumes = vec![];
    let mut envs: Vec<TemplateEntry> = env_vars
        .iter()
        .map(|(k, v)| TemplateEntry {
            source: k.clone(),
            dest: v.clone(),
        })
        .collect();

    if let Ok(ssh_auth_sock) = env::var("SSH_AUTH_SOCK") {
        volumes.push(TemplateEntry {
            source: ssh_auth_sock,
            dest: "/ssh-agent".to_string(),
        });
        envs.push(TemplateEntry {
            source: "SSH_AUTH_SOCK".to_string(),
            dest: "/ssh-agent".to_string(),
        });
    }

    let context = TemplateContext {
        service: service.to_string(),
        envs,
        volumes,
    };

    let mut tt = TinyTemplate::new();
    tt.add_template("docker-compose.yml", TEMPLATE)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let rendered = tt
        .render("docker-compose.yml", &context)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(&path)?;
    file.write_all(rendered.as_bytes())?;
    drop(file);

    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;

    Ok(ComposeOverrideGuard(path))
}

/// Resolve the Dockerfile path from config, relative to `context` if provided,
/// otherwise relative to `workspace`.
///
/// Absolute paths are returned unchanged.
///
/// # Errors
/// Returns an error if the resulting path would escape the workspace root (validated
/// by the caller, e.g. via `validate_within_root`).
pub(crate) fn resolve_dockerfile_path(
    workspace: &std::path::Path,
    dockerfile: &str,
    context: Option<&str>,
) -> std::path::PathBuf {
    let dockerfile_path = std::path::Path::new(dockerfile);
    if dockerfile_path.is_absolute() {
        return dockerfile_path.to_path_buf();
    }
    let base = if let Some(ctx) = context {
        let ctx_path = std::path::Path::new(ctx);
        if ctx_path.is_absolute() {
            ctx_path.to_path_buf()
        } else {
            workspace.join(ctx_path)
        }
    } else {
        workspace.to_path_buf()
    };
    base.join(dockerfile_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn resolve_dockerfile_no_context_relative_to_workspace() {
        let ws = std::path::Path::new("/ws");
        let result = resolve_dockerfile_path(ws, "Dockerfile", None);
        assert_eq!(result, std::path::PathBuf::from("/ws/Dockerfile"));
    }

    #[test]
    fn resolve_dockerfile_with_relative_context() {
        let ws = std::path::Path::new("/ws");
        let result = resolve_dockerfile_path(ws, "Dockerfile", Some("subdir"));
        assert_eq!(result, std::path::PathBuf::from("/ws/subdir/Dockerfile"));
    }

    #[test]
    fn resolve_dockerfile_with_absolute_context() {
        let ws = std::path::Path::new("/ws");
        let result = resolve_dockerfile_path(ws, "Dockerfile", Some("/other/ctx"));
        assert_eq!(result, std::path::PathBuf::from("/other/ctx/Dockerfile"));
    }

    #[test]
    fn resolve_dockerfile_absolute_path_returned_unchanged() {
        let ws = std::path::Path::new("/ws");
        let result = resolve_dockerfile_path(ws, "/abs/Dockerfile", Some("ctx"));
        assert_eq!(result, std::path::PathBuf::from("/abs/Dockerfile"));
    }

    #[test]
    fn compose_override_file_has_mode_0o600() {
        let guard = create_compose_override("test-service", &[])
            .expect("create_compose_override should succeed");
        let mode = std::fs::metadata(&guard.0)
            .expect("metadata should be readable")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o600, "expected 0o600, got 0o{mode:o}");
    }

    #[test]
    fn compose_override_file_removed_after_guard_drop() {
        let path = {
            let guard = create_compose_override("test-service", &[])
                .expect("create_compose_override should succeed");
            guard.0.clone()
        };
        assert!(
            !path.exists(),
            "file should be deleted after guard is dropped"
        );
    }
}
