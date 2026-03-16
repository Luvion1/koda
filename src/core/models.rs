use chrono::{DateTime, Local};
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
    Unknown,
}

impl LogLevel {
    pub fn color(&self) -> Color {
        match self {
            LogLevel::Error => Color::Red,
            LogLevel::Warn => Color::Yellow,
            LogLevel::Info => Color::Green,
            LogLevel::Debug => Color::Blue,
            LogLevel::Trace => Color::Magenta,
            LogLevel::Unknown => Color::Gray,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
            LogLevel::Unknown => "UNK",
        }
    }
}

/// A single parsed log entry.
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// The timestamp when the log was recorded.
    pub timestamp: DateTime<Local>,
    /// The severity level.
    pub level: LogLevel,
    /// The source file this log came from.
    pub source_file: String,
    /// The actual log message.
    pub message: String,
    /// The original raw line.
    pub raw: String,
}

impl LogEntry {
    pub fn new(source_file: String, raw: String) -> Self {
        // Fallback or unparsed entry
        Self {
            timestamp: Local::now(),
            level: LogLevel::Unknown,
            source_file,
            message: raw.clone(),
            raw,
        }
    }
}
