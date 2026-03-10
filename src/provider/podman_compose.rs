use std::collections::HashMap;
use std::io::Result;
use std::path::Path;
use std::process::Command;

use super::Provider;
use super::print_command;
use super::utils::{ComposeOverrideGuard, create_compose_override};

#[derive(Debug)]
pub struct PodmanCompose {
    pub build_args: HashMap<String, String>,
    pub command: String,
    pub env_vars: Vec<(String, String)>,
    pub podman_command: String,
    pub file: String,
    pub name: String,
    /// When `true`, appends `:z` to the SSH agent socket volume mount so that
    /// `SELinux` allows the container to access it.
    pub selinux_relabel: bool,
    pub service: String,
    pub shell: String,
    pub user: String,
    pub workspace_folder: String,
}

impl PodmanCompose {
    fn create_docker_compose(&self) -> Result<ComposeOverrideGuard> {
        create_compose_override(
            &self.service,
            &self.env_vars,
            self.selinux_relabel,
            &self.build_args,
        )
    }

    fn extract_container_id(output: &str) -> &str {
        output
            .lines()
            .find(|l| !l.trim().is_empty())
            .map_or("", str::trim)
    }

    fn running_args(&self) -> Vec<String> {
        vec![
            "ps".to_string(),
            "-q".to_string(),
            "--filter".to_string(),
            "status=running".to_string(),
            "--filter".to_string(),
            format!("label=io.podman.compose.project={}", &self.name),
        ]
    }

    fn rm_args(&self, override_file: &Path) -> Vec<String> {
        vec![
            "-f".to_string(),
            self.file.clone(),
            "-f".to_string(),
            override_file.to_string_lossy().into_owned(),
            "-p".to_string(),
            self.name.clone(),
            "down".to_string(),
            "--remove-orphans".to_string(),
        ]
    }
}

impl Provider for PodmanCompose {
    fn build(&self, use_cache: bool) -> Result<bool> {
        let guard = self.create_docker_compose()?;

        let mut command = Command::new(&self.command);
        command
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
            .arg(&self.shell);

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn rm(&self) -> Result<bool> {
        let guard = self.create_docker_compose()?;

        let mut command = Command::new(&self.command);
        command.args(self.rm_args(&guard.0));

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn exists(&self) -> Result<bool> {
        // Use `podman ps -aq` with project + service label filters so this works even when
        // podman-compose is not on the PATH, catches stopped containers, and is scoped to
        // the specific service (not sibling services in the same project).
        let output = Command::new(&self.podman_command)
            .args([
                "ps",
                "-aq",
                "--filter",
                &format!("label=io.podman.compose.project={}", &self.name),
                "--filter",
                &format!("label=com.docker.compose.service={}", &self.service),
            ])
            .output()?
            .stdout;

        let value = String::from_utf8(output)
            .unwrap_or_default()
            .trim()
            .to_string();

        Ok(!value.is_empty())
    }

    fn running(&self) -> Result<bool> {
        let output = Command::new(&self.podman_command)
            .args(self.running_args())
            .output()?
            .stdout;

        let value = String::from_utf8(output)
            .unwrap_or_default()
            .trim()
            .to_string();

        Ok(!value.is_empty())
    }

    fn cp(&self, source: String, destination: String) -> Result<bool> {
        // podman-compose has no native cp; find the service container via project label
        // and delegate to `podman cp`.
        let output = Command::new(&self.podman_command)
            .args([
                "ps",
                "-q",
                "--filter",
                &format!("label=io.podman.compose.project={}", &self.name),
                "--filter",
                &format!("label=io.podman.compose.service={}", &self.service),
            ])
            .output()?
            .stdout;

        let raw = String::from_utf8(output).unwrap_or_default();
        let container_id = Self::extract_container_id(&raw).to_string();

        if container_id.is_empty() {
            return Ok(false);
        }

        let mut command = Command::new(&self.podman_command);
        command
            .arg("cp")
            .arg(source)
            .arg(format!("{container_id}:{destination}"));

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn exec(&self, cmd: String) -> Result<()> {
        let guard = self.create_docker_compose()?;

        let mut command = Command::new(&self.command);
        command
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_provider(name: &str, service: &str) -> PodmanCompose {
        PodmanCompose {
            build_args: HashMap::new(),
            command: "podman-compose".to_string(),
            env_vars: vec![],
            podman_command: "podman".to_string(),
            file: "docker-compose.yml".to_string(),
            name: name.to_string(),
            selinux_relabel: false,
            service: service.to_string(),
            shell: "/bin/bash".to_string(),
            user: "vscode".to_string(),
            workspace_folder: "/workspace".to_string(),
        }
    }

    // --- extract_container_id ---

    #[test]
    fn extract_container_id_single_line() {
        let output = "abc123\n";
        assert_eq!(PodmanCompose::extract_container_id(output), "abc123");
    }

    #[test]
    fn extract_container_id_blank_leading_lines() {
        let output = "\n\n  \ndeadbeef\n";
        assert_eq!(PodmanCompose::extract_container_id(output), "deadbeef");
    }

    #[test]
    fn extract_container_id_empty() {
        assert_eq!(PodmanCompose::extract_container_id(""), "");
        assert_eq!(PodmanCompose::extract_container_id("   \n  \n"), "");
    }

    // --- running_args ---

    #[test]
    fn running_args_includes_status_running_filter() {
        let p = make_provider("myproject", "app");
        let args = p.running_args();
        // Must contain --filter status=running
        let status_idx = args
            .iter()
            .position(|a| a == "--filter")
            .expect("--filter missing");
        assert_eq!(args[status_idx + 1], "status=running");
    }

    #[test]
    fn running_args_does_not_include_format_flag() {
        let p = make_provider("myproject", "app");
        let args = p.running_args();
        assert!(
            !args.iter().any(|a| a.contains("--format")),
            "running_args must not contain --format, got: {args:?}"
        );
    }

    #[test]
    fn running_args_includes_project_label_filter() {
        let p = make_provider("testproj", "svc");
        let args = p.running_args();
        assert!(
            args.iter()
                .any(|a| a.contains("io.podman.compose.project=testproj")),
            "running_args must filter by project label, got: {args:?}"
        );
    }

    // --- rm_args ---

    #[test]
    fn rm_args_does_not_include_rmi() {
        let p = make_provider("myproject", "app");
        let args = p.rm_args(Path::new("/tmp/override.yml"));
        assert!(
            !args.iter().any(|a| a.contains("--rmi")),
            "rm_args must not contain --rmi, got: {args:?}"
        );
    }

    #[test]
    fn rm_args_includes_down_and_remove_orphans() {
        let p = make_provider("myproject", "app");
        let args = p.rm_args(Path::new("/tmp/override.yml"));
        assert!(
            args.iter().any(|a| a == "down"),
            "rm_args must include 'down'"
        );
        assert!(
            args.iter().any(|a| a == "--remove-orphans"),
            "rm_args must include '--remove-orphans'"
        );
    }

    #[test]
    fn rm_args_includes_project_name() {
        let p = make_provider("projname", "app");
        let args = p.rm_args(Path::new("/override.yml"));
        let p_idx = args.iter().position(|a| a == "-p").expect("-p missing");
        assert_eq!(args[p_idx + 1], "projname");
    }
}
