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
    /// Lowercases the project name and replaces spaces with dashes.
    #[must_use]
    pub fn safe_name(&self) -> String {
        let name = self
            .name
            .to_lowercase()
            .replace(' ', "-")
            .trim()
            .to_string();

        format!("devcont-{name}")
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
        let name = config.safe_name();
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
        let name = config.safe_name();
        assert_eq!(name, "devcont-minimal");
        assert!(!name.contains(' '), "safe_name must not contain spaces");
    }
}
