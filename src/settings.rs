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
    pub dotfiles: Vec<String>,
    #[serde(default)]
    /// Container engine to use.
    pub provider: Provider,
}

impl Settings {
    /// Load settings from the user config file, or return defaults on any error.
    ///
    /// Errors are logged to stderr rather than propagated so that a missing or
    /// malformed config file never prevents the tool from running.
    pub fn load() -> Self {
        match Self::try_load() {
            Ok(settings) => settings,
            Err(e) => {
                eprintln!("Warning: could not load settings, using defaults: {e}");
                Self::default()
            }
        }
    }

    fn try_load() -> Result<Self> {
        let Some(dirs) = ProjectDirs::from("com", "Big Refactor", "devcont") else {
            return Ok(Self::default());
        };

        let file = dirs.config_dir().join("config.toml");

        if !file.is_file() {
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(&file).map_err(Error::Io)?;
        let settings: Self =
            toml::from_str(&contents).map_err(|e| Error::SettingsLoad(e.to_string()))?;
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
        let settings = Settings::load();
        // Default provider is Docker
        assert!(matches!(settings.provider, Provider::Docker));
        assert!(settings.dotfiles.is_empty());
    }
}
