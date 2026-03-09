use std::collections::HashMap;
use std::io::Result;
use std::process::Command;

use super::Provider;
use super::print_command;
use super::utils::create_compose_override;

#[derive(Debug)]
pub struct PodmanCompose {
    pub build_args: HashMap<String, String>,
    pub command: String,
    pub env_vars: Vec<(String, String)>,
    pub podman_command: String,
    pub file: String,
    pub name: String,
    pub service: String,
    pub shell: String,
    pub user: String,
    pub workspace_folder: String,
}

impl PodmanCompose {
    fn create_docker_compose(&self) -> Result<String> {
        create_compose_override(&self.service, &self.env_vars)
    }
}

impl Provider for PodmanCompose {
    fn build(&self, use_cache: bool) -> Result<bool> {
        let docker_override = self.create_docker_compose()?;

        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&docker_override)
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
        let docker_override = self.create_docker_compose()?;

        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&docker_override)
            .arg("-p")
            .arg(&self.name)
            .arg("up")
            .arg("--detach");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn stop(&self) -> Result<bool> {
        let docker_override = self.create_docker_compose()?;

        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&docker_override)
            .arg("-p")
            .arg(&self.name)
            .arg("stop");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn restart(&self) -> Result<bool> {
        let docker_override = self.create_docker_compose()?;

        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&docker_override)
            .arg("-p")
            .arg(&self.name)
            .arg("restart");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn attach(&self) -> Result<bool> {
        let docker_override = self.create_docker_compose()?;

        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&docker_override)
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
        let docker_override = self.create_docker_compose()?;

        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&docker_override)
            .arg("-p")
            .arg(&self.name)
            .arg("down")
            .arg("--remove-orphans");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn exists(&self) -> Result<bool> {
        // Use `podman ps -aq` with a project label filter so this works even when
        // podman-compose is not on the PATH and also catches stopped containers.
        let output = Command::new(&self.podman_command)
            .args([
                "ps",
                "-aq",
                "--filter",
                &format!("label=io.podman.compose.project={}", &self.name),
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
            .arg("ps")
            .arg("-q")
            .arg("--filter")
            .arg("status=running")
            .arg("--filter")
            .arg(format!("label=io.podman.compose.project={}", &self.name))
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
        let container_id = raw
            .lines()
            .find(|l| !l.trim().is_empty())
            .unwrap_or("")
            .trim()
            .to_string();

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

    fn exec(&self, cmd: String) -> Result<bool> {
        let docker_override = self.create_docker_compose()?;

        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&docker_override)
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

        Ok(command.status()?.success())
    }

    fn exec_raw(&self, prog: &str, args: &[&str]) -> Result<bool> {
        let docker_override = self.create_docker_compose()?;

        let mut command = Command::new(&self.command);
        command
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&docker_override)
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

        Ok(command.status()?.success())
    }
}
