use std::collections::HashMap;
use std::env;
use std::io::Result;
use std::process::Command;

use super::Provider;
use super::options::ContainerOptions;
use super::print_command;
use crate::provider::docker::{BuildSource, exact_name_match};

const IMAGE_NAMESPACE: &str = "devcont";

/// Podman provider — uses the `podman` CLI to manage dev containers.
#[derive(Debug)]
pub struct Podman {
    pub build_args: HashMap<String, String>,
    /// Path sent to the Podman daemon as the build context.
    /// Defaults to the workspace root; overridden by `build.context` in devcontainer.json.
    pub build_context: String,
    pub build_source: BuildSource,
    pub command: String,
    pub directory: String,
    pub forward_ports: Vec<u16>,
    pub mounts: Option<Vec<HashMap<String, String>>>,
    pub name: String,
    pub run_args: Vec<String>,
    pub override_command: bool,
    pub user: String,
    pub workspace_folder: String,
}

impl Provider for Podman {
    fn build(&self, use_cache: bool) -> Result<bool> {
        match &self.build_source {
            BuildSource::Dockerfile(path) => {
                let tag = format!("{IMAGE_NAMESPACE}/{}", &self.name);

                let mut command = Command::new(&self.command);
                command.arg("build").arg("-t").arg(&tag).arg("-f").arg(path);

                if !use_cache {
                    command.arg("--no-cache");
                }

                for (key, value) in &self.build_args {
                    command.arg("--build-arg").arg(format!("{key}={value}"));
                }

                command.arg(&self.build_context);

                print_command(&command);

                Ok(command.status()?.success())
            }
            BuildSource::Image(image) => {
                let mut command = Command::new(&self.command);
                command.arg("pull").arg(image);

                print_command(&command);

                Ok(command.status()?.success())
            }
        }
    }

    fn create(&self, opts: &ContainerOptions) -> Result<bool> {
        let image = match &self.build_source {
            BuildSource::Dockerfile(_) => format!("{IMAGE_NAMESPACE}/{}", &self.name),
            BuildSource::Image(name) => name.clone(),
        };

        let mut command = Command::new(&self.command);
        command.arg("create");
        command.arg("--userns=keep-id");
        command.arg("--security-opt");
        command.arg("label=disable");
        command.arg("--mount");
        command.arg(format!(
            "type=bind,source={},target={}",
            &self.directory, &self.workspace_folder
        ));

        // Forwards the ssh-agent to the container
        if let Ok(ssh_auth_sock) = env::var("SSH_AUTH_SOCK") {
            command.arg("--volume");
            command.arg(format!("{ssh_auth_sock}:/ssh-agent"));
            command.arg("--env");
            command.arg("SSH_AUTH_SOCK=/ssh-agent");
        }

        for port in &self.forward_ports {
            command.arg("--publish").arg(format!("{port}:{port}"));
        }

        for (key, value) in &opts.remote_env {
            command.arg("--env").arg(format!("{key}={value}"));
        }

        for arg in &self.run_args {
            command.arg(arg);
        }

        if let Some(mounts) = &self.mounts {
            for mount in mounts {
                command.arg("--mount");
                let m = mount
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<String>>()
                    .join(",");
                command.arg(m);
            }
        }

        command.arg("-it");
        command.arg("--name");
        command.arg(&self.name);
        command.arg("-u");
        command.arg(&self.user);
        command.arg("-w");
        command.arg(&self.workspace_folder);
        command.arg(image);

        if self.override_command {
            command
                .arg("sh")
                .arg("-c")
                .arg("while sleep 1000; do :; done");
        }

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn start(&self) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command.arg("start").arg(&self.name);

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn stop(&self) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command.arg("stop").arg(&self.name);

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn restart(&self) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command.arg("restart").arg(&self.name);

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn attach(&self) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command.arg("attach").arg(&self.name);

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn rm(&self) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command.arg("rm").arg(&self.name);

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn exists(&self) -> Result<bool> {
        let output = Command::new(&self.command)
            .arg("ps")
            .arg("-a")
            .arg("--filter")
            .arg(format!("name={}", &self.name))
            .arg("--format")
            .arg("{{.Names}}")
            .output()?
            .stdout;

        Ok(exact_name_match(output, &self.name))
    }

    fn running(&self) -> Result<bool> {
        let output = Command::new(&self.command)
            .arg("ps")
            .arg("--filter")
            .arg(format!("name={}", &self.name))
            .arg("--format")
            .arg("{{.Names}}")
            .output()?
            .stdout;

        Ok(exact_name_match(output, &self.name))
    }

    fn cp(&self, source: String, destination: String) -> Result<bool> {
        let mut command = Command::new(&self.command);
        command
            .arg("cp")
            .arg(source)
            .arg(format!("{}:{}", &self.name, destination));

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn exec(&self, cmd: String) -> Result<()> {
        let mut command = Command::new(&self.command);
        command
            .arg("exec")
            .arg("-u")
            .arg(&self.user)
            .arg("-w")
            .arg(&self.workspace_folder)
            .arg(&self.name)
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
        let mut command = Command::new(&self.command);
        command
            .arg("exec")
            .arg("-u")
            .arg(&self.user)
            .arg("-w")
            .arg(&self.workspace_folder)
            .arg(&self.name)
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
