use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::io::Result;
use std::process::Command;
use tinytemplate::TinyTemplate;

use super::Provider;
use super::print_command;

#[derive(Debug)]
pub struct PodmanCompose {
    pub build_args: HashMap<String, String>,
    pub command: String,
    pub podman_command: String,
    pub file: String,
    pub name: String,
    pub service: String,
    pub user: String,
    pub workspace_folder: String,
}

#[derive(Serialize)]
struct TemplateContext {
    service: String,
    envs: Vec<TemplateVolumeContext>,
    volumes: Vec<TemplateVolumeContext>,
}

#[derive(Serialize)]
struct TemplateVolumeContext {
    source: String,
    dest: String,
}

static TEMPLATE: &str = include_str!("../../templates/docker-compose.yml");
impl PodmanCompose {
    fn create_docker_compose(&self) -> Result<String> {
        let dir = env::temp_dir();
        let file = dir.join("docker-compose.yml");
        let mut volumes = vec![];
        let mut envs = vec![];

        // Forwards the ssh-agent to the container
        if let Ok(ssh_auth_sock) = env::var("SSH_AUTH_SOCK") {
            volumes.push(TemplateVolumeContext {
                source: ssh_auth_sock,
                dest: "/ssh-agent".to_string(),
            });
            envs.push(TemplateVolumeContext {
                source: "SSH_AUTH_SOCK".to_string(),
                dest: "/ssh-agent".to_string(),
            });
        };

        let context = TemplateContext {
            service: self.service.clone(),
            envs,
            volumes,
        };

        let mut tt = TinyTemplate::new();
        tt.add_template("docker-compose.yml", TEMPLATE)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        let rendered = tt
            .render("docker-compose.yml", &context)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        std::fs::write(&file, rendered)?;

        Ok(file.to_string_lossy().to_string())
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
            .arg("zsh");

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
            .arg("--remove-orphans")
            .arg("--rmi")
            .arg("all");

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
            .arg("--format")
            .arg("{{.ID}}")
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

        let container_id = String::from_utf8(output)
            .unwrap_or_default()
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
}
