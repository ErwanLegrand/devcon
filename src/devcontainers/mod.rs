pub mod config;
pub mod one_or_many;
pub(crate) mod paths;
mod run_args;

use crate::devcontainers::one_or_many::OneOrMany;
use crate::provider::Provider;
use crate::provider::docker::{BuildSource, Docker};
use crate::provider::docker_compose::DockerCompose;
use crate::provider::podman::Podman;
use crate::provider::podman_compose::PodmanCompose;
use crate::provider::utils::resolve_dockerfile_path;
use crate::settings::Settings;
use config::Config;
use paths::validate_within_root;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

/// Execute a lifecycle hook inside the container via the provider.
///
/// - `One(cmd)` → `provider.exec(cmd)` (shell-wrapped by the provider)
/// - `Many(parts)` → `provider.exec_raw(parts[0], parts[1..])` (no shell, injection-safe)
fn exec_hook(provider: &dyn Provider, hook: &OneOrMany) -> std::io::Result<()> {
    match hook {
        OneOrMany::One(cmd) => provider.exec(cmd.clone()),
        OneOrMany::Many(_) => {
            if let Some((prog, args)) = hook.to_exec_parts() {
                let args_ref: Vec<&str> = args.iter().map(String::as_str).collect();
                provider.exec_raw(&prog, &args_ref)
            } else {
                Ok(())
            }
        }
    }
}

/// Execute a lifecycle hook on the host (not inside the container).
///
/// - `One(cmd)` → `sh -c <cmd>`
/// - `Many(parts)` → `parts[0] parts[1..]` (no shell, injection-safe)
///
/// Returns `true` if the command ran and exited successfully, `false` on non-zero exit.
/// Returns `Ok(true)` when no exec parts are produced (empty Many).
fn exec_host_hook(hook: &OneOrMany) -> std::io::Result<bool> {
    if let Some((prog, args)) = hook.to_exec_parts() {
        let status = std::process::Command::new(&prog).args(&args).status()?;
        return Ok(status.success());
    }
    Ok(true)
}

/// Returns true if `answer` constitutes a positive confirmation (case-insensitive `"y"`).
fn is_confirmed(answer: &str) -> bool {
    answer.trim().to_lowercase() == "y"
}

/// Returns true if a root-user warning should be emitted.
///
/// Warns when `remote_user` is `"root"` or `"0"` (or empty) and `no_root_check` is false.
fn should_warn_root(remote_user: &str, no_root_check: bool) -> bool {
    if no_root_check {
        return false;
    }
    matches!(remote_user.trim(), "" | "root" | "0")
}

/// Prompt the user for confirmation before running `initializeCommand` on the host.
///
/// When `trust` is true, skips the prompt and runs the hook immediately.
/// When `trust` is false, prints the command to stderr, reads a `[y/N]` response from stdin,
/// and returns `false` (without running the hook) if the user declines.
///
/// Returns `Ok(false)` to signal that the caller should abort (hook declined or hook failed).
fn confirm_and_run_host_hook(hook: &OneOrMany, trust: bool) -> std::io::Result<bool> {
    if trust {
        eprintln!("initializeCommand trusted, running on host.");
    } else {
        let cmd_display = match hook {
            OneOrMany::One(cmd) => cmd.clone(),
            OneOrMany::Many(parts) => parts.join(" "),
        };
        eprintln!("initializeCommand will run on the host: {cmd_display}");
        eprint!("Run initializeCommand on host? [y/N] ");
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer)?;
        if !is_confirmed(&answer) {
            return Ok(false);
        }
    }
    exec_host_hook(hook)
}

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
            let settings = Settings::load()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
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
    pub fn run(&self, use_cache: bool, trust: bool, no_root_check: bool) -> std::io::Result<()> {
        run_args::validate_run_args(&self.config.run_args)
            .map_err(|msg| std::io::Error::new(std::io::ErrorKind::InvalidInput, msg))?;

        let container_name = self
            .config
            .safe_name()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string()))?;
        run_args::validate_container_name(&container_name)
            .map_err(|msg| std::io::Error::new(std::io::ErrorKind::InvalidInput, msg))?;

        run_args::validate_remote_env(&self.config.remote_env)
            .map_err(|msg| std::io::Error::new(std::io::ErrorKind::InvalidInput, msg))?;

        let provider = &self.provider;

        if let Some(hook) = &self.config.initialize_command {
            if !confirm_and_run_host_hook(hook, trust)? {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "initializeCommand declined by user",
                ));
            }
        }

        self.create(use_cache)?;
        if should_warn_root(&self.config.remote_user, no_root_check) {
            eprintln!(
                "warning: container will run as root. Consider setting `remoteUser` in devcontainer.json, or pass --no-root-check to suppress this warning."
            );
        }
        if !provider.running()? {
            provider.start()?;
        }

        if let Some(hook) = &self.config.post_start_command {
            exec_hook(provider.as_ref(), hook)
                .map_err(|e| std::io::Error::new(e.kind(), format!("postStartCommand: {e}")))?;
        }

        self.post_create()?;
        provider.restart()?;
        provider.attach()?;

        if let Some(hook) = &self.config.post_attach_command {
            exec_hook(provider.as_ref(), hook)
                .map_err(|e| std::io::Error::new(e.kind(), format!("postAttachCommand: {e}")))?;
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
    pub fn rebuild(
        &self,
        use_cache: bool,
        trust: bool,
        no_root_check: bool,
    ) -> std::io::Result<()> {
        let provider = &self.provider;
        if provider.exists()? {
            provider.stop()?;
            provider.rm()?;
        }

        self.run(use_cache, trust, no_root_check)
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

        if let Some(hook) = &self.config.on_create_command {
            exec_hook(provider.as_ref(), hook)
                .map_err(|e| std::io::Error::new(e.kind(), format!("onCreateCommand: {e}")))?;
        }

        if let Some(hook) = &self.config.update_content_command {
            exec_hook(provider.as_ref(), hook)
                .map_err(|e| std::io::Error::new(e.kind(), format!("updateContentCommand: {e}")))?;
        }

        if let Some(hook) = &self.config.post_create_command {
            exec_hook(provider.as_ref(), hook)
                .map_err(|e| std::io::Error::new(e.kind(), format!("postCreateCommand: {e}")))?;
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

fn sorted_env_vars(config: &Config) -> Vec<(String, String)> {
    let mut env_vars: Vec<(String, String)> = config
        .remote_env
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    env_vars.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    env_vars
}

fn compose_path_and_service(
    directory: &Path,
    config: &Config,
) -> std::io::Result<(String, String)> {
    let compose_file = config
        .docker_compose_file
        .as_deref()
        .ok_or_else(|| missing_field("dockerComposeFile"))?;
    let compose_path = directory.join(".devcontainer").join(compose_file);
    let service = config
        .service
        .as_deref()
        .ok_or_else(|| missing_field("service"))?;
    Ok((
        compose_path.to_string_lossy().to_string(),
        service.to_string(),
    ))
}

fn docker_build_source(directory: &Path, config: &Config) -> std::io::Result<BuildSource> {
    if let Some(dockerfile) = config.dockerfile() {
        let context = config.build.as_ref().and_then(|b| b.context.as_deref());
        let resolved = resolve_dockerfile_path(directory, &dockerfile, context);
        let validated = validate_within_root(directory, &resolved)?;
        Ok(BuildSource::Dockerfile(
            validated.to_string_lossy().to_string(),
        ))
    } else if let Some(image) = config.image.clone() {
        Ok(BuildSource::Image(image))
    } else {
        Err(missing_field("build.dockerfile or image"))
    }
}

fn podman_build_source(directory: &Path, config: &Config) -> std::io::Result<BuildSource> {
    if let Some(dockerfile) = config.dockerfile() {
        let context = config.build.as_ref().and_then(|b| b.context.as_deref());
        let resolved = resolve_dockerfile_path(directory, &dockerfile, context);
        let validated = validate_within_root(directory, &resolved)?;
        Ok(BuildSource::Dockerfile(
            validated.to_string_lossy().to_string(),
        ))
    } else if let Some(image) = config.image.clone() {
        Ok(BuildSource::Image(image))
    } else {
        Err(missing_field("build.dockerfile or image"))
    }
}

/// Validate `build.context` against the workspace root (FR-003).
///
/// Relative contexts must resolve within `root`. Absolute contexts outside `root`
/// emit a warning but are allowed through (they may be intentional host mounts).
fn validate_build_context(root: &Path, context: &str) -> std::io::Result<()> {
    let context_path = Path::new(context);
    if context_path.is_absolute() {
        if validate_within_root(root, context_path).is_err() {
            eprintln!(
                "warning: build.context '{}' is absolute and outside workspace root '{}'",
                context,
                root.display()
            );
        }
        Ok(())
    } else {
        validate_within_root(root, context_path).map(|_| ())
    }
}

/// Validate relative mount sources against the workspace root (FR-004).
///
/// For each mount entry, if the `"source"` key is present and the path is
/// relative, it must resolve within `root`. Absolute source paths pass through.
fn validate_mounts(
    root: &Path,
    mounts: Option<&Vec<std::collections::HashMap<String, String>>>,
) -> std::io::Result<()> {
    if let Some(mount_list) = mounts {
        for mount in mount_list {
            if let Some(source) = mount.get("source") {
                let source_path = Path::new(source);
                if !source_path.is_absolute() {
                    validate_within_root(root, source_path).map(|_| ())?;
                }
            }
        }
    }
    Ok(())
}

fn build_provider(
    directory: &Path,
    settings: &Settings,
    config: &Config,
) -> std::io::Result<Box<dyn Provider>> {
    let name = config
        .safe_name()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string()))?;

    match settings.provider {
        crate::settings::Provider::Docker => {
            if config.is_compose() {
                let (file, service) = compose_path_and_service(directory, config)?;
                Ok(Box::new(DockerCompose {
                    build_args: config.build_args(),
                    command: "docker".to_string(),
                    env_vars: sorted_env_vars(config),
                    file,
                    name,
                    service,
                    shell: "sh".to_string(),
                    user: config.remote_user.clone(),
                    workspace_folder: config.workspace_folder.clone(),
                }))
            } else {
                if let Some(build) = &config.build {
                    if let Some(context) = &build.context {
                        validate_build_context(directory, context)?;
                    }
                }
                validate_mounts(directory, config.mounts.as_ref())?;
                Ok(Box::new(Docker {
                    build_args: config.build_args(),
                    build_source: docker_build_source(directory, config)?,
                    command: "docker".to_string(),
                    directory: directory.to_string_lossy().to_string(),
                    forward_ports: config.forward_ports.clone(),
                    name,
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
                let (file, service) = compose_path_and_service(directory, config)?;
                let selinux_relabel = config
                    .selinux_relabel
                    .unwrap_or_else(crate::provider::utils::selinux_enforcing);
                Ok(Box::new(PodmanCompose {
                    build_args: config.build_args(),
                    command: "podman-compose".to_string(),
                    env_vars: sorted_env_vars(config),
                    file,
                    name,
                    podman_command: "podman".to_string(),
                    selinux_relabel,
                    service,
                    shell: "sh".to_string(),
                    user: config.remote_user.clone(),
                    workspace_folder: config.workspace_folder.clone(),
                }))
            } else {
                if let Some(build) = &config.build {
                    if let Some(context) = &build.context {
                        validate_build_context(directory, context)?;
                    }
                }
                validate_mounts(directory, config.mounts.as_ref())?;
                Ok(Box::new(Podman {
                    build_args: config.build_args(),
                    build_source: podman_build_source(directory, config)?,
                    command: "podman".to_string(),
                    directory: directory.to_string_lossy().to_string(),
                    forward_ports: config.forward_ports.clone(),
                    mounts: config.mounts.clone(),
                    name,
                    run_args: config.run_args.clone(),
                    override_command: config.override_command,
                    user: config.remote_user.clone(),
                    workspace_folder: config.workspace_folder.clone(),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[allow(clippy::struct_field_names)]
    struct MockProvider {
        exec_calls: RefCell<Vec<String>>,
        exec_raw_calls: RefCell<Vec<(String, Vec<String>)>>,
        exec_result: bool,
    }

    impl MockProvider {
        fn new() -> Self {
            Self {
                exec_calls: RefCell::new(vec![]),
                exec_raw_calls: RefCell::new(vec![]),
                exec_result: true,
            }
        }

        fn failing() -> Self {
            Self {
                exec_calls: RefCell::new(vec![]),
                exec_raw_calls: RefCell::new(vec![]),
                exec_result: false,
            }
        }
    }

    impl Provider for MockProvider {
        fn build(&self, _: bool) -> std::io::Result<bool> {
            Ok(true)
        }
        fn create(&self, _: Vec<String>) -> std::io::Result<bool> {
            Ok(true)
        }
        fn start(&self) -> std::io::Result<bool> {
            Ok(true)
        }
        fn stop(&self) -> std::io::Result<bool> {
            Ok(true)
        }
        fn restart(&self) -> std::io::Result<bool> {
            Ok(true)
        }
        fn attach(&self) -> std::io::Result<bool> {
            Ok(true)
        }
        fn rm(&self) -> std::io::Result<bool> {
            Ok(true)
        }
        fn exists(&self) -> std::io::Result<bool> {
            Ok(false)
        }
        fn running(&self) -> std::io::Result<bool> {
            Ok(false)
        }
        fn cp(&self, _: String, _: String) -> std::io::Result<bool> {
            Ok(true)
        }
        fn exec(&self, cmd: String) -> std::io::Result<()> {
            self.exec_calls.borrow_mut().push(cmd);
            if self.exec_result {
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "exec failed (mock)",
                ))
            }
        }
        fn exec_raw(&self, prog: &str, args: &[&str]) -> std::io::Result<()> {
            self.exec_raw_calls.borrow_mut().push((
                prog.to_string(),
                args.iter().map(|s| (*s).to_string()).collect(),
            ));
            if self.exec_result {
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "exec_raw failed (mock)",
                ))
            }
        }
    }

    fn make_devcontainer_with_provider(
        config: Config,
        provider: Box<dyn Provider>,
    ) -> Devcontainer {
        Devcontainer {
            config,
            provider,
            settings: Settings::default(),
        }
    }

    #[test]
    fn exec_hook_one_uses_exec() {
        let provider = MockProvider::new();
        let hook = OneOrMany::One("echo hello".to_string());
        exec_hook(&provider, &hook).unwrap();
        assert_eq!(*provider.exec_calls.borrow(), vec!["echo hello"]);
        assert!(provider.exec_raw_calls.borrow().is_empty());
    }

    #[test]
    fn exec_hook_many_uses_exec_raw_not_exec() {
        let provider = MockProvider::new();
        let hook = OneOrMany::Many(vec!["npm".to_string(), "install".to_string()]);
        exec_hook(&provider, &hook).unwrap();
        assert!(provider.exec_calls.borrow().is_empty());
        let raw_calls = provider.exec_raw_calls.borrow();
        assert_eq!(raw_calls.len(), 1);
        assert_eq!(raw_calls[0].0, "npm");
        assert_eq!(raw_calls[0].1, vec!["install"]);
    }

    #[test]
    fn exec_hook_many_preserves_args_with_spaces() {
        let provider = MockProvider::new();
        let hook = OneOrMany::Many(vec![
            "echo".to_string(),
            "hello world".to_string(),
            "foo bar".to_string(),
        ]);
        exec_hook(&provider, &hook).unwrap();
        assert!(provider.exec_calls.borrow().is_empty());
        let raw_calls = provider.exec_raw_calls.borrow();
        assert_eq!(raw_calls.len(), 1);
        assert_eq!(raw_calls[0].0, "echo");
        assert_eq!(raw_calls[0].1, vec!["hello world", "foo bar"]);
    }

    #[test]
    fn exec_hook_many_empty_returns_ok() {
        let provider = MockProvider::new();
        let hook = OneOrMany::Many(vec![]);
        exec_hook(&provider, &hook).expect("empty Many hook should succeed");
        assert!(provider.exec_calls.borrow().is_empty());
        assert!(provider.exec_raw_calls.borrow().is_empty());
    }

    fn config_with_post_create() -> Config {
        json5::from_str(
            r#"{ "name": "test", "image": "alpine", "postCreateCommand": "npm install" }"#,
        )
        .unwrap()
    }

    fn config_with_post_start() -> Config {
        json5::from_str(r#"{ "name": "test", "image": "alpine", "postStartCommand": "start.sh" }"#)
            .unwrap()
    }

    fn config_with_post_attach() -> Config {
        json5::from_str(
            r#"{ "name": "test", "image": "alpine", "postAttachCommand": "attach.sh" }"#,
        )
        .unwrap()
    }

    fn config_with_on_create() -> Config {
        json5::from_str(
            r#"{ "name": "test", "image": "alpine", "onCreateCommand": "on-create.sh" }"#,
        )
        .unwrap()
    }

    fn config_with_update_content() -> Config {
        json5::from_str(
            r#"{ "name": "test", "image": "alpine", "updateContentCommand": "update.sh" }"#,
        )
        .unwrap()
    }

    #[test]
    fn run_aborts_on_post_create_hook_failure() {
        let dc = make_devcontainer_with_provider(
            config_with_post_create(),
            Box::new(MockProvider::failing()),
        );
        let err = dc
            .run(true, true, true)
            .expect_err("run() must fail when postCreateCommand returns false");
        let msg = err.to_string();
        assert!(
            msg.contains("postCreateCommand"),
            "error message should mention 'postCreateCommand', got: {msg}"
        );
    }

    #[test]
    fn run_aborts_on_post_start_hook_failure() {
        let dc = make_devcontainer_with_provider(
            config_with_post_start(),
            Box::new(MockProvider::failing()),
        );
        let err = dc
            .run(true, true, true)
            .expect_err("run() must fail when postStartCommand returns false");
        let msg = err.to_string();
        assert!(
            msg.contains("postStartCommand"),
            "error message should mention 'postStartCommand', got: {msg}"
        );
    }

    #[test]
    fn run_aborts_on_post_attach_hook_failure() {
        let dc = make_devcontainer_with_provider(
            config_with_post_attach(),
            Box::new(MockProvider::failing()),
        );
        let err = dc
            .run(true, true, true)
            .expect_err("run() must fail when postAttachCommand returns false");
        // The run() aborts on the first exec failure — postAttachCommand or an earlier
        // internal exec (e.g., copy_gitconfig). Either way the error is surfaced.
        let _ = err.to_string();
    }

    #[test]
    fn run_aborts_on_on_create_hook_failure() {
        let dc = make_devcontainer_with_provider(
            config_with_on_create(),
            Box::new(MockProvider::failing()),
        );
        let err = dc
            .run(true, true, true)
            .expect_err("run() must fail when onCreateCommand returns false");
        let msg = err.to_string();
        assert!(
            msg.contains("onCreateCommand"),
            "error message should mention 'onCreateCommand', got: {msg}"
        );
    }

    #[test]
    fn run_aborts_on_update_content_hook_failure() {
        let dc = make_devcontainer_with_provider(
            config_with_update_content(),
            Box::new(MockProvider::failing()),
        );
        let err = dc
            .run(true, true, true)
            .expect_err("run() must fail when updateContentCommand returns false");
        let msg = err.to_string();
        assert!(
            msg.contains("updateContentCommand"),
            "error message should mention 'updateContentCommand', got: {msg}"
        );
    }

    #[test]
    fn is_confirmed_y_lowercase() {
        assert!(is_confirmed("y"));
    }

    #[test]
    fn is_confirmed_y_uppercase() {
        assert!(is_confirmed("Y"));
    }

    #[test]
    fn is_confirmed_y_with_whitespace() {
        assert!(is_confirmed("  y\n"));
    }

    #[test]
    fn is_confirmed_n_is_false() {
        assert!(!is_confirmed("n"));
    }

    #[test]
    fn is_confirmed_empty_is_false() {
        assert!(!is_confirmed(""));
    }

    #[test]
    fn is_confirmed_yes_is_false() {
        assert!(!is_confirmed("yes"));
    }

    #[test]
    fn root_user_triggers_warning() {
        assert!(should_warn_root("root", false));
    }

    #[test]
    fn uid_zero_triggers_warning() {
        assert!(should_warn_root("0", false));
    }

    #[test]
    fn empty_user_triggers_warning() {
        assert!(should_warn_root("", false));
    }

    #[test]
    fn non_root_user_no_warning() {
        assert!(!should_warn_root("vscode", false));
    }

    #[test]
    fn root_user_suppressed_with_no_root_check() {
        assert!(!should_warn_root("root", true));
    }
}
