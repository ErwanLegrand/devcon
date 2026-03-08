pub mod config;
pub mod one_or_many;

use crate::provider::Provider;
use crate::provider::docker::{BuildSource, Docker};
use crate::provider::docker_compose::DockerCompose;
use crate::provider::podman::Podman;
use crate::provider::podman_compose::PodmanCompose;
use crate::settings::Settings;
use config::Config;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

/// A loaded and configured dev container instance.
pub struct Devcontainer {
    config: Config,
    provider: Box<dyn Provider>,
    settings: Settings,
}

impl Devcontainer {
    /// Load a dev container from `directory`, resolving the config file and
    /// selecting the appropriate container provider based on user settings.
    ///
    /// Looks for `.devcontainer/devcontainer.json` first, then `.devcontainer.json`.
    ///
    /// # Errors
    /// Returns an error if the config file is missing, cannot be read, or fails to parse.
    pub fn load(directory: &Path) -> Result<Self, std::io::Error> {
        let file = directory.join(".devcontainer").join("devcontainer.json");
        let file = if file.is_file() {
            file
        } else {
            directory.join(".devcontainer.json")
        };

        if file.exists() {
            let config = Config::parse(&file).map_err(|e| {
                std::io::Error::new(
                    ErrorKind::InvalidData,
                    format!("could not parse {}: {}", file.display(), e),
                )
            })?;
            let settings = Settings::load();
            let provider = build_provider(directory, &settings, &config)?;

            Ok(Self {
                config: config.clone(),
                provider,
                settings,
            })
        } else {
            Err(std::io::Error::new(
                ErrorKind::NotFound,
                "Could not find .devcontainer/devcontainer.json or .devcontainer.json",
            ))
        }
    }

    /// Build, start, and attach to the dev container, running lifecycle hooks.
    ///
    /// If `use_cache` is `false`, the image is built with `--no-cache`.
    ///
    /// # Errors
    /// Returns an error if any provider operation (build, start, attach, etc.) fails.
    pub fn run(&self, use_cache: bool) -> std::io::Result<()> {
        let provider = &self.provider;

        if let Some(cmd) = self.config.initialize_command.clone() {
            std::process::Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .status()?;
        }

        self.create(use_cache)?;
        if !provider.running()? {
            provider.start()?;
        }

        if let Some(cmd) = self.config.post_start_command.clone() {
            provider.exec(cmd)?;
        }

        self.post_create()?;
        provider.restart()?;
        provider.attach()?;

        if let Some(cmd) = self.config.post_attach_command.clone() {
            provider.exec(cmd)?;
        }

        if self.config.should_shutdown() {
            provider.stop()?;
        }

        Ok(())
    }

    /// Stop and remove the existing container, then run it fresh.
    ///
    /// # Errors
    /// Returns an error if stopping, removing, or restarting the container fails.
    pub fn rebuild(&self, use_cache: bool) -> std::io::Result<()> {
        let provider = &self.provider;
        if provider.exists()? {
            provider.stop()?;
            provider.rm()?;
        }

        self.run(use_cache)
    }

    fn create(&self, use_cache: bool) -> std::io::Result<()> {
        let provider = &self.provider;

        if !provider.exists()? {
            provider.build(use_cache)?;
            provider.create(self.create_args())?;
        }

        Ok(())
    }

    fn post_create(&self) -> std::io::Result<()> {
        let provider = &self.provider;

        if let Some(command) = self.config.on_create_command.clone() {
            provider.exec(command)?;
        }

        if let Some(command) = self.config.update_content_command.clone() {
            provider.exec(command)?;
        }

        if let Some(command) = self.config.post_create_command.clone() {
            provider.exec(command)?;
        }

        self.copy_gitconfig()?;
        self.copy_dotfiles()?;

        Ok(())
    }

    fn copy(&self, source: &Path, dest: &str) -> std::io::Result<bool> {
        if source.exists() {
            let provider = &self.provider;
            let destpath = PathBuf::from(dest);
            let basedir = destpath
                .parent()
                .and_then(|p| p.to_str())
                .unwrap_or("<non-utf8>");
            let destination = if source.is_dir() { basedir } else { dest };

            // Shell-quote the path to handle spaces and special characters safely.
            let basedir_quoted = format!("'{}'", basedir.replace('\'', r"'\''"));
            provider.exec(format!("mkdir -p -- {basedir_quoted}"))?;
            provider.cp(
                source.to_string_lossy().to_string(),
                destination.to_string(),
            )
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found {source:?}"),
            ))
        }
    }

    fn copy_dotfiles(&self) -> std::io::Result<()> {
        let homedir = if self.config.remote_user == "root" {
            PathBuf::from("/root")
        } else {
            PathBuf::from("/home").join(&self.config.remote_user)
        };

        for file in &self.settings.dotfiles {
            let tilded = format!("~/{file}");
            let expanded = shellexpand::tilde(&tilded).to_string();
            let source = PathBuf::from(expanded);
            let dest = homedir.join(file.clone());
            let dest_str = dest.to_string_lossy();

            self.copy(&source, &dest_str)?;
        }

        Ok(())
    }

    fn copy_gitconfig(&self) -> std::io::Result<bool> {
        let path = shellexpand::tilde("~/.gitconfig").to_string();
        let file = PathBuf::from(path);
        if !file.exists() {
            return Ok(false);
        }
        let homedir = if self.config.remote_user == "root" {
            PathBuf::from("/root")
        } else {
            PathBuf::from("/home").join(&self.config.remote_user)
        };
        let dest = homedir.join(".gitconfig");
        let dest_str = dest.to_string_lossy();
        self.copy(&file, &dest_str)
    }

    /// Build the list of extra arguments passed to the container create command
    /// (environment variables, working directory, and `runArgs` from config).
    #[must_use]
    pub fn create_args(&self) -> Vec<String> {
        let mut args = vec![];

        for (key, value) in &self.config.remote_env {
            args.push("-e".to_string());
            args.push(format!("{key}={value}"));
        }

        let workspace_folder = self.config.workspace_folder.clone();
        args.push("-w".to_string());
        args.push(workspace_folder);

        for arg in self.config.run_args.clone() {
            args.push(arg);
        }

        args
    }
}

fn missing_field(field: &str) -> std::io::Error {
    std::io::Error::new(
        ErrorKind::InvalidData,
        format!("devcontainer.json is missing required field: {field}"),
    )
}

fn build_provider(
    directory: &Path,
    settings: &Settings,
    config: &Config,
) -> std::io::Result<Box<dyn Provider>> {
    match settings.provider {
        crate::settings::Provider::Docker => {
            if config.is_compose() {
                let compose_file = config
                    .docker_compose_file
                    .as_deref()
                    .ok_or_else(|| missing_field("dockerComposeFile"))?;
                let compose_path = directory.join(".devcontainer").join(compose_file);
                let service = config
                    .service
                    .as_deref()
                    .ok_or_else(|| missing_field("service"))?;

                let mut env_vars: Vec<(String, String)> = config
                    .remote_env
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                env_vars.sort_unstable_by(|a, b| a.0.cmp(&b.0));

                Ok(Box::new(DockerCompose {
                    build_args: config.build_args(),
                    command: "docker".to_string(),
                    env_vars,
                    file: compose_path.to_string_lossy().to_string(),
                    name: config.safe_name(),
                    service: service.to_string(),
                    shell: "sh".to_string(),
                    user: config.remote_user.clone(),
                    workspace_folder: config.workspace_folder.clone(),
                }))
            } else {
                let build_source = if let Some(dockerfile) = config.dockerfile() {
                    BuildSource::Dockerfile(
                        directory.join(dockerfile).to_string_lossy().to_string(),
                    )
                } else if let Some(image) = config.image.clone() {
                    BuildSource::Image(image)
                } else {
                    return Err(missing_field("build.dockerfile or image"));
                };

                Ok(Box::new(Docker {
                    build_args: config.build_args(),
                    build_source,
                    command: "docker".to_string(),
                    directory: directory.to_string_lossy().to_string(),
                    forward_ports: config.forward_ports.clone(),
                    name: config.safe_name(),
                    override_command: config.override_command,
                    run_args: config.run_args.clone(),
                    mounts: config.mounts.clone(),
                    user: config.remote_user.clone(),
                    workspace_folder: config.workspace_folder.clone(),
                }))
            }
        }
        crate::settings::Provider::Podman => {
            if config.is_compose() {
                let compose_file = config
                    .docker_compose_file
                    .as_deref()
                    .ok_or_else(|| missing_field("dockerComposeFile"))?;
                let compose_path = directory.join(".devcontainer").join(compose_file);
                let service = config
                    .service
                    .as_deref()
                    .ok_or_else(|| missing_field("service"))?;

                let mut env_vars: Vec<(String, String)> = config
                    .remote_env
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                env_vars.sort_unstable_by(|a, b| a.0.cmp(&b.0));

                Ok(Box::new(PodmanCompose {
                    build_args: config.build_args(),
                    command: "podman-compose".to_string(),
                    env_vars,
                    file: compose_path.to_string_lossy().to_string(),
                    name: config.safe_name(),
                    podman_command: "podman".to_string(),
                    service: service.to_string(),
                    shell: "sh".to_string(),
                    user: config.remote_user.clone(),
                    workspace_folder: config.workspace_folder.clone(),
                }))
            } else {
                let build_source = if let Some(dockerfile) = config.dockerfile() {
                    let path = directory.join(".devcontainer").join(dockerfile);
                    BuildSource::Dockerfile(path.to_string_lossy().to_string())
                } else if let Some(image) = config.image.clone() {
                    BuildSource::Image(image)
                } else {
                    return Err(missing_field("build.dockerfile or image"));
                };

                Ok(Box::new(Podman {
                    build_args: config.build_args(),
                    build_source,
                    command: "podman".to_string(),
                    directory: directory.to_string_lossy().to_string(),
                    forward_ports: config.forward_ports.clone(),
                    name: config.safe_name(),
                    run_args: config.run_args.clone(),
                    override_command: config.override_command,
                    user: config.remote_user.clone(),
                    workspace_folder: config.workspace_folder.clone(),
                }))
            }
        }
    }
}
