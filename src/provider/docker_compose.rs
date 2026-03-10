use std::collections::HashMap;
use std::io::Result;
use std::process::Command;

use super::Provider;
use super::print_command;
use super::utils::{ComposeOverrideGuard, create_compose_override};

/// Docker Compose provider — manages containers via `docker compose`.
#[derive(Debug)]
pub struct DockerCompose {
    pub build_args: HashMap<String, String>,
    pub command: String,
    pub env_vars: Vec<(String, String)>,
    pub file: String,
    pub name: String,
    pub service: String,
    pub shell: String,
    pub user: String,
    pub workspace_folder: String,
}

impl DockerCompose {
    fn create_docker_compose(&self) -> Result<ComposeOverrideGuard> {
        create_compose_override(&self.service, &self.env_vars, false, &self.build_args)
    }
}

impl Provider for DockerCompose {
    fn build(&self, use_cache: bool) -> Result<bool> {
        let guard = self.create_docker_compose()?;

        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&guard.0)
            .arg("-p")
            .arg(&self.name)
            .arg("build");

        if !use_cache {
            command.arg("--no-cache");
        }

        for (key, value) in &self.build_args {
            command.arg("--build-arg").arg(format!("{key}={value}"));
        }

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn create(&self, _args: Vec<String>) -> Result<bool> {
        Ok(true)
    }

    fn start(&self) -> Result<bool> {
        let guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&guard.0)
            .arg("-p")
            .arg(&self.name)
            .arg("up")
            .arg("--detach");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn stop(&self) -> Result<bool> {
        let guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&guard.0)
            .arg("-p")
            .arg(&self.name)
            .arg("stop");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn restart(&self) -> Result<bool> {
        let guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&guard.0)
            .arg("-p")
            .arg(&self.name)
            .arg("restart");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn attach(&self) -> Result<bool> {
        let guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&guard.0)
            .arg("-p")
            .arg(&self.name)
            .arg("exec")
            .arg("-u")
            .arg(&self.user)
            .arg("-w")
            .arg(&self.workspace_folder);

        command.arg(&self.service).arg(&self.shell);

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn rm(&self) -> Result<bool> {
        let guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&guard.0)
            .arg("-p")
            .arg(&self.name)
            .arg("down")
            .arg("--remove-orphans")
            .arg("--rmi")
            .arg("all");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn exists(&self) -> Result<bool> {
        // Scope to the specific service to avoid false-positives from sibling services.
        let output = Command::new(&self.command)
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-p")
            .arg(&self.name)
            .arg("ps")
            .arg("-aq")
            .arg("--format")
            .arg("json")
            .arg(&self.service)
            .output()?
            .stdout;

        Ok(compose_service_exists(output))
    }

    fn running(&self) -> Result<bool> {
        let output = Command::new(&self.command)
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-p")
            .arg(&self.name)
            .arg("ps")
            .arg("-q")
            .arg("--status=running")
            .output()?
            .stdout;

        let value = String::from_utf8(output)
            .unwrap_or_default()
            .trim()
            .to_string();

        Ok(!value.is_empty())
    }

    fn cp(&self, source: String, destination: String) -> Result<bool> {
        let guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&guard.0)
            .arg("-p")
            .arg(&self.name)
            .arg("cp")
            .arg(source)
            .arg(format!("{}:{}", &self.service, destination));

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn exec(&self, cmd: String) -> Result<()> {
        let guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&guard.0)
            .arg("-p")
            .arg(&self.name)
            .arg("exec")
            .arg("-u")
            .arg(&self.user)
            .arg("-w")
            .arg(&self.workspace_folder)
            .arg(&self.service)
            .arg("sh")
            .arg("-c")
            .arg(cmd);

        print_command(&command);

        let status = command.status()?;
        if status.success() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("exec failed with exit code {}", status.code().unwrap_or(-1)),
            ))
        }
    }

    fn exec_raw(&self, prog: &str, args: &[&str]) -> Result<()> {
        let guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&guard.0)
            .arg("-p")
            .arg(&self.name)
            .arg("exec")
            .arg("-u")
            .arg(&self.user)
            .arg("-w")
            .arg(&self.workspace_folder)
            .arg(&self.service)
            .arg(prog)
            .args(args);

        print_command(&command);

        let status = command.status()?;
        if status.success() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "exec_raw failed with exit code {}",
                    status.code().unwrap_or(-1)
                ),
            ))
        }
    }
}

/// Returns true if the `docker compose ps --format json` output indicates at least one container.
///
/// Handles empty output, JSON `[]`, and `null` as "no container". Any other non-empty content
/// means a container record was returned. Also handles newline-delimited JSON (one object per line).
pub(crate) fn compose_service_exists(output: Vec<u8>) -> bool {
    let text = String::from_utf8(output).unwrap_or_default();
    let trimmed = text.trim();
    !matches!(trimmed, "" | "[]" | "null")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_service_exists_empty_output_is_false() {
        assert!(!compose_service_exists(vec![]));
    }

    #[test]
    fn compose_service_exists_empty_array_is_false() {
        assert!(!compose_service_exists(b"[]".to_vec()));
    }

    #[test]
    fn compose_service_exists_null_is_false() {
        assert!(!compose_service_exists(b"null".to_vec()));
    }

    #[test]
    fn compose_service_exists_populated_array_is_true() {
        let json = br#"[{"ID":"abc","Name":"proj-svc-1","State":"running"}]"#;
        assert!(compose_service_exists(json.to_vec()));
    }

    #[test]
    fn compose_service_exists_newline_delimited_object_is_true() {
        let json = br#"{"ID":"abc","Name":"proj-svc-1","State":"running"}"#;
        assert!(compose_service_exists(json.to_vec()));
    }
}
