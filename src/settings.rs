use directories::ProjectDirs;
use serde::Deserialize;

use crate::error::{Error, Result};

/// Container engine provider selection.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    /// Use Docker (default).
    Docker,
    /// Use Podman.
    Podman,
}

impl Default for Provider {
    fn default() -> Self {
        Self::Docker
    }
}

/// User settings loaded from `~/.config/devcont/config.toml`.
#[derive(Debug, Default, Deserialize)]
pub struct Settings {
    /// Dotfiles to copy into the container (relative paths from `~`).
    #[serde(default)]
    pub dotfiles: Vec<String>,
    #[serde(default)]
    /// Container engine to use.
    pub provider: Provider,
}

impl Settings {
    /// Load settings from the user config file.
    ///
    /// Returns `Ok(default)` when the settings file does not exist.
    /// Returns `Err` when the file exists but cannot be read or parsed.
    ///
    /// # Errors
    /// Returns [`Error::Io`] if the settings file exists but cannot be read.
    /// Returns [`Error::SettingsLoad`] if the settings file exists but cannot be parsed.
    pub fn load() -> Result<Self> {
        let Some(dirs) = ProjectDirs::from("com", "Big Refactor", "devcont") else {
            return Ok(Self::default());
        };

        let file = dirs.config_dir().join("config.toml");

        if !file.is_file() {
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(&file).map_err(|e| {
            Error::Io(std::io::Error::new(
                e.kind(),
                format!("could not read settings file {}: {e}", file.display()),
            ))
        })?;
        let settings: Self = toml::from_str(&contents).map_err(|e| {
            Error::SettingsLoad(format!(
                "could not parse settings file {}: {e}",
                file.display()
            ))
        })?;
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_returns_default_when_no_config_file() {
        // This test relies on the fact that in CI/test environments there is
        // no devcont config file present.  Settings::load() must not panic.
        let settings = Settings::load().expect("load should succeed with no config file");
        // Default provider is Docker
        assert!(matches!(settings.provider, Provider::Docker));
        assert!(settings.dotfiles.is_empty());
    }

    #[test]
    fn invalid_toml_fails_to_parse() {
        // Verify that malformed TOML yields a parse error (unit-tests the parsing layer
        // without needing a real settings file on disk).
        let contents = "not valid toml {{{";
        let result: std::result::Result<Settings, _> = toml::from_str(contents);
        assert!(result.is_err(), "invalid TOML should fail to parse");
    }

    #[test]
    fn provider_docker_is_default() {
        let s: Settings = toml::from_str("").expect("empty TOML should parse");
        assert!(matches!(s.provider, Provider::Docker));
    }

    #[test]
    fn provider_podman_parses() {
        let s: Settings =
            toml::from_str("provider = \"podman\"").expect("podman provider should parse");
        assert!(matches!(s.provider, Provider::Podman));
    }

    #[test]
    fn unknown_provider_value_fails_to_parse() {
        let result: std::result::Result<Settings, _> = toml::from_str("provider = \"nspawn\"");
        assert!(
            result.is_err(),
            "unknown provider value should fail to parse"
        );
    }

    #[test]
    fn dotfiles_parsed_as_vec() {
        let s: Settings =
            toml::from_str("dotfiles = [\".bashrc\", \".vimrc\"]").expect("dotfiles should parse");
        assert_eq!(s.dotfiles, vec![".bashrc", ".vimrc"]);
    }

    #[test]
    fn dotfiles_absent_defaults_to_empty() {
        let s: Settings = toml::from_str("").expect("empty TOML should parse");
        assert!(s.dotfiles.is_empty());
    }
}
