use std::collections::HashMap;
use std::io::Result;
use std::path::Path;
use std::process::Command;

use super::Provider;
use super::print_command;
use super::utils::{ComposeOverrideGuard, create_compose_override};

/// Podman Compose provider — manages containers via `podman-compose`.
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

    /// Build the base compose args: `-f <file> -f <override> -p <name>`.
    fn compose_base_args(&self, override_file: &Path) -> Vec<String> {
        vec![
            "-f".to_string(),
            self.file.clone(),
            "-f".to_string(),
            override_file.to_string_lossy().into_owned(),
            "-p".to_string(),
            self.name.clone(),
        ]
    }

    fn build_command_args(&self, override_file: &Path, use_cache: bool) -> Vec<String> {
        let mut args = self.compose_base_args(override_file);
        args.push("build".to_string());
        if !use_cache {
            args.push("--no-cache".to_string());
        }
        for (key, value) in &self.build_args {
            args.push("--build-arg".to_string());
            args.push(format!("{key}={value}"));
        }
        args
    }

    fn start_command_args(&self, override_file: &Path) -> Vec<String> {
        let mut args = self.compose_base_args(override_file);
        args.extend(["up".to_string(), "--detach".to_string()]);
        args
    }

    fn stop_command_args(&self, override_file: &Path) -> Vec<String> {
        let mut args = self.compose_base_args(override_file);
        args.push("stop".to_string());
        args
    }

    fn restart_command_args(&self, override_file: &Path) -> Vec<String> {
        let mut args = self.compose_base_args(override_file);
        args.push("restart".to_string());
        args
    }

    fn attach_command_args(&self, override_file: &Path) -> Vec<String> {
        let mut args = self.compose_base_args(override_file);
        args.extend([
            "exec".to_string(),
            "-u".to_string(),
            self.user.clone(),
            "-w".to_string(),
            self.workspace_folder.clone(),
            self.service.clone(),
            self.shell.clone(),
        ]);
        args
    }

    fn exec_command_args(&self, override_file: &Path, cmd: &str) -> Vec<String> {
        let mut args = self.compose_base_args(override_file);
        args.extend([
            "exec".to_string(),
            "-u".to_string(),
            self.user.clone(),
            "-w".to_string(),
            self.workspace_folder.clone(),
            self.service.clone(),
            "sh".to_string(),
            "-c".to_string(),
            cmd.to_string(),
        ]);
        args
    }

    fn exec_raw_command_args(
        &self,
        override_file: &Path,
        prog: &str,
        extra: &[&str],
    ) -> Vec<String> {
        let mut args = self.compose_base_args(override_file);
        args.extend([
            "exec".to_string(),
            "-u".to_string(),
            self.user.clone(),
            "-w".to_string(),
            self.workspace_folder.clone(),
            self.service.clone(),
            prog.to_string(),
        ]);
        args.extend(extra.iter().map(|s| (*s).to_string()));
        args
    }
}

impl Provider for PodmanCompose {
    fn build(&self, use_cache: bool) -> Result<bool> {
        let guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command.args(self.build_command_args(&guard.0, use_cache));
        print_command(&command);
        Ok(command.status()?.success())
    }

    fn create(&self, _args: Vec<String>) -> Result<bool> {
        Ok(true)
    }

    fn start(&self) -> Result<bool> {
        let guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command.args(self.start_command_args(&guard.0));
        print_command(&command);
        Ok(command.status()?.success())
    }

    fn stop(&self) -> Result<bool> {
        let guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command.args(self.stop_command_args(&guard.0));
        print_command(&command);
        Ok(command.status()?.success())
    }

    fn restart(&self) -> Result<bool> {
        let guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command.args(self.restart_command_args(&guard.0));
        print_command(&command);
        Ok(command.status()?.success())
    }

    fn attach(&self) -> Result<bool> {
        let guard = self.create_docker_compose()?;
        let mut command = Command::new(&self.command);
        command.args(self.attach_command_args(&guard.0));
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
        command.args(self.exec_command_args(&guard.0, &cmd));
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
        command.args(self.exec_raw_command_args(&guard.0, prog, args));
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

    // --- compose_base_args ---

    #[test]
    fn compose_base_args_includes_compose_file() {
        let p = make_provider("proj", "svc");
        let args = p.compose_base_args(Path::new("/override.yml"));
        let idx = args.iter().position(|a| a == "-f").expect("-f missing");
        assert_eq!(args[idx + 1], "docker-compose.yml");
    }

    #[test]
    fn compose_base_args_includes_override_file() {
        let p = make_provider("proj", "svc");
        let args = p.compose_base_args(Path::new("/my/override.yml"));
        let positions: Vec<usize> = args
            .iter()
            .enumerate()
            .filter(|(_, a)| *a == "-f")
            .map(|(i, _)| i)
            .collect();
        assert_eq!(positions.len(), 2, "must have two -f flags");
        assert_eq!(args[positions[1] + 1], "/my/override.yml");
    }

    #[test]
    fn compose_base_args_includes_project_name() {
        let p = make_provider("myproj", "svc");
        let args = p.compose_base_args(Path::new("/override.yml"));
        let idx = args.iter().position(|a| a == "-p").expect("-p missing");
        assert_eq!(args[idx + 1], "myproj");
    }

    // --- build_command_args ---

    #[test]
    fn build_command_args_includes_build_subcommand() {
        let p = make_provider("proj", "svc");
        let args = p.build_command_args(Path::new("/override.yml"), true);
        assert!(args.iter().any(|a| a == "build"), "must include 'build'");
    }

    #[test]
    fn build_command_args_no_cache_flag() {
        let p = make_provider("proj", "svc");
        let args = p.build_command_args(Path::new("/override.yml"), false);
        assert!(
            args.iter().any(|a| a == "--no-cache"),
            "must include --no-cache"
        );
    }

    #[test]
    fn build_command_args_with_cache_no_no_cache_flag() {
        let p = make_provider("proj", "svc");
        let args = p.build_command_args(Path::new("/override.yml"), true);
        assert!(
            !args.iter().any(|a| a == "--no-cache"),
            "must not include --no-cache"
        );
    }

    #[test]
    fn build_command_args_includes_build_arg_flags() {
        let mut p = make_provider("proj", "svc");
        p.build_args.insert("FOO".to_string(), "bar".to_string());
        let args = p.build_command_args(Path::new("/override.yml"), true);
        let idx = args
            .iter()
            .position(|a| a == "--build-arg")
            .expect("--build-arg missing");
        assert_eq!(args[idx + 1], "FOO=bar");
    }

    // --- start_command_args ---

    #[test]
    fn start_command_args_includes_up_detach() {
        let p = make_provider("proj", "svc");
        let args = p.start_command_args(Path::new("/override.yml"));
        assert!(args.iter().any(|a| a == "up"), "must include 'up'");
        assert!(
            args.iter().any(|a| a == "--detach"),
            "must include '--detach'"
        );
    }

    // --- stop_command_args ---

    #[test]
    fn stop_command_args_includes_stop_subcommand() {
        let p = make_provider("proj", "svc");
        let args = p.stop_command_args(Path::new("/override.yml"));
        assert!(args.iter().any(|a| a == "stop"), "must include 'stop'");
    }

    // --- restart_command_args ---

    #[test]
    fn restart_command_args_includes_restart_subcommand() {
        let p = make_provider("proj", "svc");
        let args = p.restart_command_args(Path::new("/override.yml"));
        assert!(
            args.iter().any(|a| a == "restart"),
            "must include 'restart'"
        );
    }

    // --- attach_command_args ---

    #[test]
    fn attach_command_args_includes_exec_and_service() {
        let p = make_provider("proj", "app");
        let args = p.attach_command_args(Path::new("/override.yml"));
        assert!(args.iter().any(|a| a == "exec"), "must include 'exec'");
        assert!(args.iter().any(|a| a == "app"), "must include service name");
    }

    #[test]
    fn attach_command_args_includes_shell() {
        let p = make_provider("proj", "svc");
        let args = p.attach_command_args(Path::new("/override.yml"));
        assert!(args.iter().any(|a| a == "/bin/bash"), "must include shell");
    }

    #[test]
    fn attach_command_args_includes_user_and_workspace() {
        let p = make_provider("proj", "svc");
        let args = p.attach_command_args(Path::new("/override.yml"));
        let u_idx = args.iter().position(|a| a == "-u").expect("-u missing");
        assert_eq!(args[u_idx + 1], "vscode");
        let w_idx = args.iter().position(|a| a == "-w").expect("-w missing");
        assert_eq!(args[w_idx + 1], "/workspace");
    }

    // --- exec_command_args ---

    #[test]
    fn exec_command_args_passes_cmd_via_sh_c() {
        let p = make_provider("proj", "svc");
        let args = p.exec_command_args(Path::new("/override.yml"), "npm install");
        // Must end with sh -c <cmd>
        let len = args.len();
        assert!(len >= 3);
        assert_eq!(args[len - 3], "sh");
        assert_eq!(args[len - 2], "-c");
        assert_eq!(args[len - 1], "npm install");
    }

    // --- exec_raw_command_args ---

    #[test]
    fn exec_raw_command_args_places_prog_after_service() {
        let p = make_provider("proj", "svc");
        let args = p.exec_raw_command_args(Path::new("/override.yml"), "myprogram", &["--flag"]);
        // service comes before prog
        let svc_idx = args
            .iter()
            .position(|a| a == "svc")
            .expect("service missing");
        assert_eq!(args[svc_idx + 1], "myprogram");
        assert_eq!(args[svc_idx + 2], "--flag");
    }

    #[test]
    fn exec_raw_command_args_no_sh_wrapper() {
        let p = make_provider("proj", "svc");
        let args = p.exec_raw_command_args(Path::new("/override.yml"), "myprog", &[]);
        assert!(!args.iter().any(|a| a == "sh"), "exec_raw must not use sh");
        assert!(!args.iter().any(|a| a == "-c"), "exec_raw must not use -c");
    }
}
