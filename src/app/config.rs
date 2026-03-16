//! Application configuration module

use serde::{Deserialize, Serialize};

/// Application configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
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
