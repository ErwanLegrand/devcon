use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    name: String,
    // Retained for devcontainer spec completeness; may be used in future features.
    #[allow(dead_code)]
    pub file: Option<String>,
    pub build: Option<Build>,
    #[serde(default)]
    pub forward_ports: Vec<u16>,
    pub on_create_command: Option<String>,
    pub update_content_command: Option<String>,
    pub post_create_command: Option<String>,
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub dockerfile: Option<String>,
    // Retained for devcontainer spec completeness; may be used in future features.
    #[allow(dead_code)]
    pub context: Option<String>,

    #[serde(default)]
    pub args: HashMap<String, String>,
}

impl Config {
    pub fn parse(file: &Path) -> Result<Config> {
        let contents = std::fs::read_to_string(file).map_err(Error::Io)?;
        let config: Config =
            json5::from_str(&contents).map_err(|e| Error::ConfigParse(e.to_string()))?;
        Ok(config)
    }

    pub fn dockerfile(&self) -> Option<String> {
        self.build.clone().and_then(|b| b.dockerfile)
    }

    pub fn build_args(&self) -> HashMap<String, String> {
        self.build.clone().map(|b| b.args).unwrap_or_default()
    }

    pub fn safe_name(&self) -> String {
        let name = self
            .name
            .to_lowercase()
            .replace(' ', "-")
            .trim()
            .to_string();

        format!("devcont-{}", name)
    }

    pub fn should_shutdown(&self) -> bool {
        !matches!(self.shutdown_action, ShutdownAction::None)
    }

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
            "safe_name() should start with 'devcont-', got '{}'",
            name
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
        matches!(result.unwrap_err(), Error::ConfigParse(_));
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
