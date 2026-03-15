// src/core/parser.rs
use super::models::{LogEntry, LogLevel};
use regex::Regex;
use serde_json::Value;

pub struct LogParser {
    regex: Regex,
}

impl Default for LogParser {
    fn default() -> Self {
        // A generic regex that tries to catch common log formats:
        // [YYYY-MM-DD HH:MM:SS] [LEVEL] Message
        let regex = Regex::new(
            r"(?i)^(?:\[?(?P<time>\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})?)\]?\s+)?(?:\[?(?P<level>ERROR|WARN|INFO|DEBUG|TRACE)\]?\s+)?(?P<msg>.*)",
        ).unwrap();
        Self { regex }
    }
}

impl LogParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(&self, source_file: &str, line: &str) -> LogEntry {
        // Try JSON first if it looks like JSON
        let trimmed = line.trim();
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            if let Ok(json_val) = serde_json::from_str::<Value>(trimmed) {
                return self.parse_json(source_file, line, json_val);
            }
        }

        // Fallback to Regex
        if let Some(caps) = self.regex.captures(line) {
            let mut entry = LogEntry::new(source_file.to_string(), line.to_string());

            if let Some(level_match) = caps.name("level") {
                entry.level = match level_match.as_str().to_uppercase().as_str() {
                    "ERROR" => LogLevel::Error,
                    "WARN" | "WARNING" => LogLevel::Warn,
                    "INFO" => LogLevel::Info,
                    "DEBUG" => LogLevel::Debug,
                    "TRACE" => LogLevel::Trace,
                    _ => LogLevel::Unknown,
                };
            }

            if let Some(msg_match) = caps.name("msg") {
                entry.message = msg_match.as_str().to_string();
            }

            // We ignore time parsing complexity here for brevity,
            // relying on the Local::now() from LogEntry::new as fallback if not precisely parsed.

            return entry;
        }

        LogEntry::new(source_file.to_string(), line.to_string())
    }

    fn parse_json(&self, source_file: &str, raw: &str, json: Value) -> LogEntry {
        let mut entry = LogEntry::new(source_file.to_string(), raw.to_string());

        if let Some(level) = json.get("level").and_then(|v| v.as_str()) {
            entry.level = match level.to_uppercase().as_str() {
                "ERROR" => LogLevel::Error,
                "WARN" | "WARNING" => LogLevel::Warn,
                "INFO" => LogLevel::Info,
                "DEBUG" => LogLevel::Debug,
                "TRACE" => LogLevel::Trace,
                _ => LogLevel::Unknown,
            };
        }

        if let Some(msg) = json
            .get("message")
            .or(json.get("msg"))
            .and_then(|v| v.as_str())
        {
            entry.message = msg.to_string();
        }

        entry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::LogLevel;

    #[test]
    fn test_parse_plain_text() {
        let parser = LogParser::new();
        let line = "[2023-10-27 10:00:00] [ERROR] Something went wrong";
        let entry = parser.parse("test.log", line);

        assert_eq!(entry.level, LogLevel::Error);
        assert_eq!(entry.message, "Something went wrong");
        assert_eq!(entry.source_file, "test.log");
    }

    #[test]
    fn test_parse_json() {
        let parser = LogParser::new();
        let line = r#"{"timestamp": "2023-10-27T10:00:00Z", "level": "INFO", "message": "User login success"}"#;
        let entry = parser.parse("test.log", line);

        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.message, "User login success");
    }

    #[test]
    fn test_parse_unknown_format() {
        let parser = LogParser::new();
        let line = "Just a random line of text";
        let entry = parser.parse("test.log", line);

        assert_eq!(entry.level, LogLevel::Unknown);
        assert_eq!(entry.message, "Just a random line of text");
    }
}
