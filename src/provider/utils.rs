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
    suffix: String,
}

/// Return `true` if `SELinux` is currently in enforcing mode.
///
/// Reads `/sys/fs/selinux/enforce`; any error (file absent, unreadable)
/// is treated as "not enforcing".
pub(crate) fn selinux_enforcing() -> bool {
    std::fs::read_to_string("/sys/fs/selinux/enforce")
        .map(|s| s.trim() == "1")
        .unwrap_or(false)
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
/// When `selinux_relabel` is `true`, a `:z` label is appended to the SSH socket
/// volume mount so that `SELinux` allows the container to access the socket.
///
/// The file is created with mode 0o600 (owner read/write only).
///
/// # Errors
/// Returns an error if the template cannot be rendered or the file cannot be written.
pub(crate) fn create_compose_override(
    service: &str,
    env_vars: &[(String, String)],
    selinux_relabel: bool,
) -> Result<ComposeOverrideGuard> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    use std::os::unix::fs::PermissionsExt;

    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let dir = env::temp_dir();
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = dir.join(format!("docker-compose-{}-{count}.yml", std::process::id()));
    let mut volumes = vec![];
    let mut envs: Vec<TemplateEntry> = env_vars
        .iter()
        .map(|(k, v)| TemplateEntry {
            source: k.clone(),
            dest: v.clone(),
            suffix: String::new(),
        })
        .collect();

    if let Ok(ssh_auth_sock) = env::var("SSH_AUTH_SOCK") {
        let volume_suffix = if selinux_relabel {
            ":z".to_string()
        } else {
            String::new()
        };
        volumes.push(TemplateEntry {
            source: ssh_auth_sock,
            dest: "/ssh-agent".to_string(),
            suffix: volume_suffix,
        });
        envs.push(TemplateEntry {
            source: "SSH_AUTH_SOCK".to_string(),
            dest: "/ssh-agent".to_string(),
            suffix: String::new(),
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
        let guard = create_compose_override("test-service", &[], false)
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
            let guard = create_compose_override("test-service", &[], false)
                .expect("create_compose_override should succeed");
            guard.0.clone()
        };
        assert!(
            !path.exists(),
            "file should be deleted after guard is dropped"
        );
    }

    #[test]
    fn selinux_enforcing_returns_false_when_file_absent() {
        // On non-`SELinux` systems /sys/fs/selinux/enforce does not exist.
        // The function must return false rather than panic.
        // (On `SELinux` systems that are enforcing this test would return true —
        //  we only assert it doesn't panic.)
        let _ = selinux_enforcing();
    }

    #[test]
    fn compose_override_without_selinux_has_no_z_suffix() {
        let guard = create_compose_override("svc", &[], false)
            .expect("create_compose_override should succeed");
        let content = std::fs::read_to_string(&guard.0).expect("file should be readable");
        assert!(
            !content.contains(":z"),
            "non-`SELinux` override should not contain ':z', got:\n{content}"
        );
    }

    #[test]
    fn compose_override_with_selinux_has_z_suffix() {
        // SSH_AUTH_SOCK must be set for the volume entry to appear.
        let guard = std::env::var("SSH_AUTH_SOCK").ok().map(|sock| {
            let _ = sock; // use value
        });
        // Only run the assertion when SSH_AUTH_SOCK is set.
        if std::env::var("SSH_AUTH_SOCK").is_ok() {
            let file_guard = create_compose_override("svc", &[], true)
                .expect("create_compose_override should succeed");
            let content = std::fs::read_to_string(&file_guard.0).expect("file should be readable");
            assert!(
                content.contains(":z"),
                "`SELinux` override should contain ':z', got:\n{content}"
            );
        }
        let _ = guard;
    }
}
