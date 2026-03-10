use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

use crate::devcontainers::one_or_many::OneOrMany;
use crate::error::{Error, Result};

#[derive(Debug, Deserialize, Clone)]
enum ShutdownAction {
    None,
    StopContainer,
    StopCompose,
}

impl Default for ShutdownAction {
    fn default() -> Self {
        Self::StopContainer
    }
}

/// Parsed representation of a `devcontainer.json` configuration file.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    name: String,
    pub image: Option<String>,
    pub build: Option<Build>,
    #[serde(default)]
    pub forward_ports: Vec<u16>,
    pub initialize_command: Option<OneOrMany>,
    pub on_create_command: Option<OneOrMany>,
    pub update_content_command: Option<OneOrMany>,
    pub post_create_command: Option<OneOrMany>,
    pub post_start_command: Option<OneOrMany>,
    pub post_attach_command: Option<OneOrMany>,
    #[serde(default = "default_remote_user")]
    pub remote_user: String,
    #[serde(default)]
    pub run_args: Vec<String>,
    #[serde(default)]
    pub override_command: bool,
    pub mounts: Option<Vec<HashMap<String, String>>>,
    #[serde(default)]
    pub remote_env: HashMap<String, String>,
    pub docker_compose_file: Option<String>,
    pub service: Option<String>,
    /// Override `SELinux` auto-detection for SSH socket relabelling (`:z`).
    /// When absent, `SELinux` enforcing mode is detected at runtime.
    pub selinux_relabel: Option<bool>,
    #[serde(default = "default_workspace_folder")]
    pub workspace_folder: String,
    #[serde(default)]
    shutdown_action: ShutdownAction,
}

/// Build configuration block within `devcontainer.json`.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub dockerfile: Option<String>,
    // Dev Containers spec §build.context — path from which docker build is run.
    // Parsed from devcontainer.json but not yet wired into providers; tracked in
    // conductor/tracks/code_inventory_20260228/findings.md (Item 3, Retain).
    pub context: Option<String>,

    #[serde(default)]
    pub args: HashMap<String, String>,
}

impl Config {
    /// Parse a `devcontainer.json` (or JSON5) file from `file`.
    ///
    /// # Errors
    /// Returns [`Error::Io`] if the file cannot be read, or [`Error::ConfigParse`]
    /// if the content is not valid JSON5 or does not match the expected schema.
    pub fn parse(file: &Path) -> Result<Config> {
        let contents = std::fs::read_to_string(file).map_err(Error::Io)?;
        let config: Config =
            json5::from_str(&contents).map_err(|e| Error::ConfigParse(e.to_string()))?;
        Ok(config)
    }

    /// Return the Dockerfile path from the `build` block, if present.
    #[must_use]
    pub fn dockerfile(&self) -> Option<String> {
        self.build.clone().and_then(|b| b.dockerfile)
    }

    /// Return the build arguments from the `build` block, or an empty map.
    #[must_use]
    pub fn build_args(&self) -> HashMap<String, String> {
        self.build.clone().map(|b| b.args).unwrap_or_default()
    }

    /// Return a container-safe name with the `devcont-` prefix.
    ///
    /// Lowercases the project name, replaces spaces with dashes, and strips
    /// characters that are not alphanumeric, `-`, `_`, or `.`.
    ///
    /// # Errors
    /// Returns [`Error::InvalidConfig`] when the project name produces an empty
    /// string after stripping — this happens for all-Unicode names that have no
    /// ASCII representation.
    pub fn safe_name(&self) -> Result<String> {
        let stripped: String = self
            .name
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| matches!(c, 'a'..='z' | '0'..='9' | '-' | '_' | '.'))
            .collect();

        if stripped.is_empty() {
            return Err(Error::InvalidConfig(format!(
                "Cannot derive a container name from project name '{}'. \
                 Rename the project to use ASCII characters.",
                self.name
            )));
        }

        // Docker requires names to start with a letter or digit.
        let name = if stripped.starts_with(|c: char| c.is_ascii_alphanumeric()) {
            stripped
        } else {
            eprintln!(
                "notice: container name derived from '{}' starts with a non-alphanumeric \
                 character; prepending 'dev-'",
                self.name
            );
            format!("dev-{stripped}")
        };

        Ok(format!("devcont-{name}"))
    }

    /// Return `true` if the container should be stopped after the session ends.
    ///
    /// Note: `StopCompose` and `StopContainer` are currently treated identically —
    /// both stop the container/project on exit. The distinction (stop only the
    /// service vs. full `compose down`) is not yet implemented; tracked in
    /// `conductor/tracks/code_inventory_20260228/findings.md` (Item 4, Retain).
    #[must_use]
    pub fn should_shutdown(&self) -> bool {
        !matches!(self.shutdown_action, ShutdownAction::None)
    }

    /// Return `true` if this config uses Docker Compose (i.e., `dockerComposeFile` is set).
    #[must_use]
    pub fn is_compose(&self) -> bool {
        self.docker_compose_file.is_some()
    }
}

fn default_remote_user() -> String {
    "root".to_string()
}

fn default_workspace_folder() -> String {
    "/workspace".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn safe_name_uses_devcont_prefix() {
        let fixture = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/devcontainer.json"
        ));
        let config = Config::parse(fixture).expect("fixture should parse");
        let name = config.safe_name().expect("safe_name should succeed");
        assert!(
            name.starts_with("devcont-"),
            "safe_name() should start with 'devcont-', got '{name}'"
        );
        assert_eq!(name, "devcont-test-project");
    }

    #[test]
    fn parse_missing_file_returns_err_not_panic() {
        let missing = Path::new("/tmp/nonexistent_devcontainer_fixture.json");
        let result = Config::parse(missing);
        assert!(
            result.is_err(),
            "Config::parse() on a missing file must return Err"
        );
    }

    #[test]
    fn parse_invalid_json5_returns_err_not_panic() {
        let invalid = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/devcontainer_invalid.json"
        ));
        let result = Config::parse(invalid);
        assert!(
            result.is_err(),
            "Config::parse() on invalid JSON5 must return Err"
        );
        // Ensure it's a ConfigParse variant, not an Io error
        assert!(matches!(result.unwrap_err(), Error::ConfigParse(_)));
    }

    #[test]
    fn parse_image_field() {
        let fixture = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/devcontainer_image.json"
        ));
        let config = Config::parse(fixture).expect("image fixture should parse");
        assert_eq!(
            config.image,
            Some("alpine".to_string()),
            "image field should be 'alpine'"
        );
        assert!(config.build.is_none(), "build should be absent");
    }

    #[test]
    fn safe_name_replaces_spaces_with_dashes() {
        let minimal = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/devcontainer_minimal.json"
        ));
        let config = Config::parse(minimal).expect("minimal fixture should parse");
        let name = config.safe_name().expect("safe_name should succeed");
        assert_eq!(name, "devcont-minimal");
        assert!(!name.contains(' '), "safe_name must not contain spaces");
    }

    #[test]
    fn safe_name_strips_unicode_and_returns_error_when_empty() {
        let config = Config {
            name: "\u{4e2d}\u{6587}".to_string(), // "中文" (Chinese)
            image: None,
            build: None,
            forward_ports: vec![],
            initialize_command: None,
            on_create_command: None,
            update_content_command: None,
            post_create_command: None,
            post_start_command: None,
            post_attach_command: None,
            remote_user: "root".to_string(),
            run_args: vec![],
            override_command: false,
            mounts: None,
            remote_env: std::collections::HashMap::new(),
            docker_compose_file: None,
            service: None,
            selinux_relabel: None,
            workspace_folder: "/workspace".to_string(),
            shutdown_action: ShutdownAction::default(),
        };
        let result = config.safe_name();
        assert!(result.is_err(), "all-unicode name should produce an error");
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("ASCII"), "error message should mention ASCII");
    }

    #[test]
    fn safe_name_prepends_dev_when_starts_with_dash() {
        let config = Config {
            name: "-myproject".to_string(),
            image: None,
            build: None,
            forward_ports: vec![],
            initialize_command: None,
            on_create_command: None,
            update_content_command: None,
            post_create_command: None,
            post_start_command: None,
            post_attach_command: None,
            remote_user: "root".to_string(),
            run_args: vec![],
            override_command: false,
            mounts: None,
            remote_env: std::collections::HashMap::new(),
            docker_compose_file: None,
            service: None,
            selinux_relabel: None,
            workspace_folder: "/workspace".to_string(),
            shutdown_action: ShutdownAction::default(),
        };
        let name = config.safe_name().expect("should succeed");
        assert!(
            name.starts_with("devcont-dev-"),
            "name starting with '-' should get 'dev-' prefix, got: {name}"
        );
    }
}
