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
        let mut valid_file_count = 0;
        
        for file in &files {
            let path = Path::new(file);
            if !path.exists() {
                eprintln!("Warning: File not found: {}", file);
                let entry = LogEntry::new(file.to_string(), format!("[ERROR] File not found: {}", file));
                let _ = tx.send(entry);
                continue;
            }
            match lines.add_file(file).await {
                Ok(_) => valid_file_count += 1,
                Err(e) => {
                    eprintln!("Warning: Failed to tail file '{}': {}", file, e);
                    let entry = LogEntry::new(file.to_string(), format!("[ERROR] Failed to tail: {}", e));
                    let _ = tx.send(entry);
                }
            }
        }

        if valid_file_count == 0 {
            eprintln!("Error: No valid files to tail.");
            let entry = LogEntry::new(String::new(), "[ERROR] No valid files to tail. Exiting.".to_string());
            let _ = tx.send(entry);
            return Ok(());
        }

        loop {
            match lines.next_line().await {
                Ok(Some(line)) => {
                    let source = line.source().display().to_string();
                    let entry = self.parser.parse(&source, line.line());
                    if tx.send(entry).is_err() {
                        break;
                    }
                }
                Ok(None) => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
                Err(e) => {
                    eprintln!("Error reading from file: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            }
        }

        Ok(())
    }
}
