use std::collections::HashMap;
use std::io::Result;
use std::process::Command;

use super::Provider;
use super::print_command;
use super::utils::{ComposeOverrideGuard, create_compose_override};

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
        create_compose_override(&self.service, &self.env_vars)
    }
}

impl Provider for DockerCompose {
    fn build(&self, use_cache: bool) -> Result<bool> {
        let _guard = self.create_docker_compose()?;

        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&_guard.0)
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
        let _guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&_guard.0)
            .arg("-p")
            .arg(&self.name)
            .arg("up")
            .arg("--detach");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn stop(&self) -> Result<bool> {
        let _guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&_guard.0)
            .arg("-p")
            .arg(&self.name)
            .arg("stop");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn restart(&self) -> Result<bool> {
        let _guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&_guard.0)
            .arg("-p")
            .arg(&self.name)
            .arg("restart");

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn attach(&self) -> Result<bool> {
        let _guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&_guard.0)
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
        let _guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&_guard.0)
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
        let output = Command::new(&self.command)
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-p")
            .arg(&self.name)
            .arg("ps")
            .arg("-aq")
            .output()?
            .stdout;

        let value = String::from_utf8(output)
            .unwrap_or_default()
            .trim()
            .to_string();

        Ok(!value.is_empty())
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
        let _guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&_guard.0)
            .arg("-p")
            .arg(&self.name)
            .arg("cp")
            .arg(source)
            .arg(format!("{}:{}", &self.service, destination));

        print_command(&command);

        Ok(command.status()?.success())
    }

    fn exec(&self, cmd: String) -> Result<bool> {
        let _guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&_guard.0)
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
        let _guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command
            .arg("compose")
            .arg("-f")
            .arg(&self.file)
            .arg("-f")
            .arg(&_guard.0)
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
