//! Application configuration module

use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Application configuration settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// Maximum number of logs to keep in memory
    pub max_logs: usize,
    /// Tick rate in milliseconds for the event loop
    pub tick_rate_ms: u64,
    /// Enable auto-scroll to bottom when new logs arrive
    pub auto_scroll: bool,
    /// Show timestamps in log entries
    pub show_timestamps: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_logs: 1000,
            tick_rate_ms: 16,
            auto_scroll: true,
            show_timestamps: true,
        }
    }
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            tracing::debug!("Config file not found at {:?}, using defaults", path);
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        tracing::info!("Loaded configuration from {}", path.display());
        Ok(config)
    }

    /// Save configuration to a TOML file
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self).with_context(|| "Failed to serialize config")?;

        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        tracing::info!("Saved configuration to {}", path.display());
        Ok(())
    }

    /// Get the default config file path
    pub fn default_config_path() -> Option<std::path::PathBuf> {
        directories::ProjectDirs::from("com", "koda", "koda-tail")
            .map(|dirs| dirs.config_dir().join("config.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.max_logs, 1000);
        assert_eq!(config.tick_rate_ms, 16);
        assert!(config.auto_scroll);
        assert!(config.show_timestamps);
    }

    #[test]
    fn test_config_from_file_not_exists() {
        let temp_path = std::path::PathBuf::from("/nonexistent/path/config.toml");
        let config = Config::from_file(&temp_path).unwrap();
        assert_eq!(config, Config::default());
    }

    #[test]
    fn test_config_save_and_load() {
        let temp_file = NamedTempFile::new().unwrap();
        let config = Config {
            max_logs: 500,
            tick_rate_ms: 32,
            auto_scroll: false,
            show_timestamps: false,
        };

        config.save(temp_file.path()).unwrap();
        let loaded = Config::from_file(temp_file.path()).unwrap();

        assert_eq!(loaded.max_logs, 500);
        assert_eq!(loaded.tick_rate_ms, 32);
        assert!(!loaded.auto_scroll);
        assert!(!loaded.show_timestamps);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("max_logs = 1000"));
        assert!(toml_str.contains("tick_rate_ms = 16"));
    }
}
