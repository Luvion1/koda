use super::models::LogEntry;
use super::parser::LogParser;
use linemux::MuxedLines;
// Removed unused Path
use tokio::sync::mpsc;

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
        
        for file in files {
            lines.add_file(file).await?;
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
