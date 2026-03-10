use std::collections::HashMap;
use std::env;
use std::io::Result;
use std::process::Command;

use super::Provider;
use super::print_command;

const IMAGE_NAMESPACE: &str = "devcont";

/// Source used to obtain the container image for a provider.
#[derive(Debug)]
pub enum BuildSource {
    /// Build from a Dockerfile at the given path.
    Dockerfile(String),
    /// Pull a pre-built image by name.
    Image(String),
}

#[derive(Debug)]
pub struct Docker {
    pub build_args: HashMap<String, String>,
    pub build_source: BuildSource,
    pub command: String,
    pub directory: String,
    pub forward_ports: Vec<u16>,
    pub name: String,
    pub run_args: Vec<String>,
    pub mounts: Option<Vec<HashMap<String, String>>>,
    pub user: String,
    pub workspace_folder: String,
    pub override_command: bool,
}

impl Provider for Docker {
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

                command.arg(&self.directory);

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

    fn create(&self, args: Vec<String>) -> Result<bool> {
        let image = match &self.build_source {
            BuildSource::Dockerfile(_) => format!("{IMAGE_NAMESPACE}/{}", &self.name),
            BuildSource::Image(name) => name.clone(),
        };

        let mut command = Command::new(&self.command);
        command.arg("create");
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

        for arg in &args {
            command.arg(arg);
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
                .arg("/bin/sh")
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

    fn exec(&self, cmd: String) -> Result<bool> {
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

        Ok(command.status()?.success())
    }

    fn exec_raw(&self, prog: &str, args: &[&str]) -> Result<bool> {
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

        Ok(command.status()?.success())
    }
}

/// Returns true if any name in `output` (newline-separated) exactly matches `name`.
pub(crate) fn exact_name_match(output: Vec<u8>, name: &str) -> bool {
    String::from_utf8(output)
        .unwrap_or_default()
        .lines()
        .any(|line| line.trim() == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_name_match_matches_exact() {
        let output = b"foo\n".to_vec();
        assert!(exact_name_match(output, "foo"));
    }

    #[test]
    fn exact_name_match_no_prefix_match() {
        let output = b"foobar\n".to_vec();
        assert!(!exact_name_match(output, "foo"));
    }

    #[test]
    fn exact_name_match_no_suffix_match() {
        let output = b"barfoo\n".to_vec();
        assert!(!exact_name_match(output, "foo"));
    }

    #[test]
    fn exact_name_match_empty_output() {
        assert!(!exact_name_match(vec![], "foo"));
    }

    #[test]
    fn exact_name_match_multiple_names_one_matches() {
        let output = b"foobar\nfoo\nbaz\n".to_vec();
        assert!(exact_name_match(output, "foo"));
    }

    #[test]
    fn exact_name_match_multiple_names_none_match() {
        let output = b"foobar\nbaz\n".to_vec();
        assert!(!exact_name_match(output, "foo"));
    }
}
