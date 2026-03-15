use super::models::LogEntry;
use super::parser::LogParser;
use linemux::MuxedLines;
use tokio::sync::mpsc;
use std::path::Path;

pub struct TailManager {
    parser: LogParser,
}

impl Default for TailManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TailManager {
    pub fn new() -> Self {
        Self {
            parser: LogParser::new(),
        }
    }

    pub async fn tail_files(
        &self,
        files: Vec<String>,
        tx: mpsc::UnboundedSender<LogEntry>,
    ) -> anyhow::Result<()> {
        let mut lines = MuxedLines::new()?;
        
        for file in &files {
            let path = Path::new(file);
            if !path.exists() {
                eprintln!("Warning: File not found: {}", file);
                // Try to create an empty entry to notify user
                let entry = LogEntry::new(file.to_string(), format!("[ERROR] File not found: {}", file));
                let _ = tx.send(entry);
                continue;
            }
            if let Err(e) = lines.add_file(file).await {
                eprintln!("Warning: Failed to tail file '{}': {}", file, e);
                let entry = LogEntry::new(file.to_string(), format!("[ERROR] Failed to tail: {}", e));
                let _ = tx.send(entry);
                continue;
            }
        }

        // If no files were successfully added, warn the user
        if files.is_empty() {
            eprintln!("Error: No valid files to tail.");
            return Ok(());
        }

        while let Ok(Some(line)) = lines.next_line().await {
            let source = line.source().display().to_string();
            let entry = self.parser.parse(&source, line.line());
            if tx.send(entry).is_err() {
                break; // Receiver dropped
            }
        }

        Ok(())
    }
}
