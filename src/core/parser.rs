use super::models::{LogEntry, LogLevel};
use chrono::{DateTime, Local};
use regex::Regex;
use serde_json::Value;

pub struct LogParser {
    regex: Regex,
    time_regex: Regex,
}

impl Default for LogParser {
    fn default() -> Self {
        let regex = Regex::new(
            r"(?i)^(?:\[?(?P<time>\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})?)\]?\s+)?(?:\[?(?P<level>ERROR|WARN|INFO|DEBUG|TRACE)\]?\s+)?(?P<msg>.*)",
        ).unwrap();
        let time_regex =
            Regex::new(r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})?")
                .unwrap();
        Self { regex, time_regex }
    }
}

impl LogParser {
    pub fn new() -> Self {
        Self::default()
    }

    fn parse_timestamp(&self, line: &str) -> Option<DateTime<Local>> {
        if let Some(cap) = self.time_regex.find(line) {
            let time_str = cap.as_str();
            let time_str = time_str.replace('T', " ");

            if let Ok(dt) = time_str.parse::<DateTime<Local>>() {
                return Some(dt);
            }

            let formats = [
                "%Y-%m-%d %H:%M:%S%.f%z",
                "%Y-%m-%d %H:%M:%S%.f",
                "%Y-%m-%d %H:%M:%S%z",
                "%Y-%m-%d %H:%M:%S",
            ];

            for fmt in &formats {
                if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&time_str, fmt) {
                    return Some(dt.and_local_timezone(Local).unwrap());
                }
            }
        }
        None
    }

    pub fn parse(&self, source_file: &str, line: &str) -> LogEntry {
        let trimmed = line.trim();
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            if let Ok(json_val) = serde_json::from_str::<Value>(trimmed) {
                return self.parse_json(source_file, line, json_val);
            }
        }

        if let Some(caps) = self.regex.captures(line) {
            let mut entry = LogEntry::new(source_file.to_string(), line.to_string());

            if let Some(time_match) = caps.name("time") {
                if let Some(dt) = self.parse_timestamp(time_match.as_str()) {
                    entry.timestamp = dt;
                }
            }

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

        if let Some(ts) = json
            .get("timestamp")
            .or(json.get("time"))
            .and_then(|v| v.as_str())
        {
            if let Some(dt) = self.parse_timestamp(ts) {
                entry.timestamp = dt;
            }
        }

        entry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::LogLevel;
    use chrono::Timelike;

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

    #[test]
    fn test_parse_timestamp_plain() {
        let parser = LogParser::new();
        let line = "[2023-10-27 10:00:00] [INFO] Test message";
        let entry = parser.parse("test.log", line);

        assert_eq!(entry.timestamp.hour(), 10);
        assert_eq!(entry.timestamp.minute(), 0);
        assert_eq!(entry.timestamp.second(), 0);
    }

    #[test]
    fn test_parse_timestamp_iso() {
        let parser = LogParser::new();
        let line =
            r#"{"timestamp": "2023-10-27T15:30:45Z", "level": "DEBUG", "message": "ISO format"}"#;
        let entry = parser.parse("test.log", line);

        assert_eq!(entry.timestamp.hour(), 15);
        assert_eq!(entry.timestamp.minute(), 30);
    }
}
