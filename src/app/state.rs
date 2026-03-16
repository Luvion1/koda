use crate::app::config::Config;
use crate::core::models::{LogEntry, LogLevel};
use crate::ui::components::{
    log_view::LogViewComponent, spinner::SpinnerComponent, tabs::TabsComponent,
};
use crate::ui::layout::{app_layout, centered_rect};
use crossterm::event::KeyCode;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;
use regex::Regex;
use std::collections::VecDeque;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Filtering,
    Detail,
    Export,
}

pub struct AppState {
    pub is_running: bool,
    pub input_mode: InputMode,
    pub filter_query: String,
    pub filter_regex: Option<Regex>,
    pub use_regex_filter: bool,
    pub level_filter: Option<LogLevel>,
    pub source_filter: Option<String>,
    pub tabs: TabsComponent,
    pub logs: VecDeque<LogEntry>,
    pub filtered_logs: Vec<LogEntry>,
    pub selected_log: Option<LogEntry>,
    pub dirty_filter: bool,
    pub new_logs_arrived: bool,
    pub config: Config,
    pub log_view: LogViewComponent,
    pub spinner: SpinnerComponent,
    pub is_tailing: bool,
    pub paused: bool,
    pub last_key: Option<KeyCode>,
    pub files: Vec<String>,
    pub export_path: String,
    pub export_message: Option<String>,
    pub stats_start_time: Option<chrono::DateTime<chrono::Local>>,
    pub stats_log_count: usize,
    pub logs_per_sec: f64,
    pub last_log_count: usize,
}

impl AppState {
    pub fn new(files: Vec<String>, config: Config) -> Self {
        Self {
            is_running: true,
            input_mode: InputMode::Normal,
            filter_query: String::new(),
            filter_regex: None,
            use_regex_filter: false,
            level_filter: None,
            tabs: TabsComponent::new(vec![
                "Dashboard".to_string(),
                "Stats".to_string(),
                "Settings".to_string(),
                "Help".to_string(),
            ]),
            logs: VecDeque::with_capacity(config.max_logs),
            filtered_logs: Vec::new(),
            selected_log: None,
            dirty_filter: false,
            new_logs_arrived: false,
            config,
            log_view: LogViewComponent::new(),
            spinner: SpinnerComponent::new(),
            is_tailing: !files.is_empty(),
            paused: false,
            last_key: None,
            files,
            export_path: String::new(),
            export_message: None,
            source_filter: None,
            stats_start_time: Some(chrono::Local::now()),
            stats_log_count: 0,
            logs_per_sec: 0.0,
            last_log_count: 0,
        }
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn export_logs(&mut self) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        if self.export_path.is_empty() {
            self.export_path = format!(
                "koda-export-{}.log",
                chrono::Local::now().format("%Y%m%d-%H%M%S")
            );
        }

        let mut file = File::create(&self.export_path)?;
        for log in &self.filtered_logs {
            writeln!(file, "{}", log.raw)?;
        }

        self.export_message = Some(format!(
            "Exported {} logs to {}",
            self.filtered_logs.len(),
            self.export_path
        ));
        self.input_mode = InputMode::Normal;
        Ok(())
    }

    pub fn copy_log_to_clipboard(&mut self) {
        if let Some(ref entry) = self.selected_log {
            let clipboard_path = "/tmp/koda-clipboard.txt";
            if std::fs::write(clipboard_path, &entry.message).is_ok() {
                self.export_message =
                    Some(format!("Copied to {} (paste manually)", clipboard_path));
            }
        }
    }

    pub fn push_log(&mut self, entry: LogEntry) {
        if self.paused {
            return;
        }
        if self.logs.len() >= self.config.max_logs {
            self.logs.pop_front();
        }
        self.logs.push_back(entry);
        self.stats_log_count += 1;
        self.dirty_filter = true;
        self.new_logs_arrived = true;
    }

    pub fn update_filter(&mut self) {
        if !self.dirty_filter && !self.new_logs_arrived {
            return;
        }

        let query = self.filter_query.to_lowercase();

        if self.use_regex_filter {
            if self.filter_query.is_empty() {
                self.filter_regex = None;
            } else if self.filter_regex.is_none() {
                if let Ok(re) = Regex::new(&self.filter_query) {
                    self.filter_regex = Some(re);
                }
            }
        } else {
            self.filter_regex = None;
        }

        let level_filter = self.level_filter;
        let source_filter = self.source_filter.clone();

        self.filtered_logs = self
            .logs
            .iter()
            .filter(|entry| {
                if let Some(ref level) = level_filter {
                    if entry.level != *level {
                        return false;
                    }
                }

                if let Some(ref source) = source_filter {
                    if !entry.source_file.contains(source) {
                        return false;
                    }
                }

                if let Some(ref re) = self.filter_regex {
                    if !re.is_match(&entry.raw) {
                        return false;
                    }
                } else if !query.is_empty() && !entry.raw.to_lowercase().contains(&query) {
                    return false;
                }

                true
            })
            .cloned()
            .collect();

        self.dirty_filter = false;
    }

    pub fn cycle_source_filter(&mut self) {
        let mut sources: Vec<String> = self.logs.iter().map(|e| e.source_file.clone()).collect();
        sources.sort();
        sources.dedup();

        if sources.is_empty() {
            return;
        }

        if let Some(ref current) = self.source_filter {
            if let Some(idx) = sources.iter().position(|s| s == current) {
                let next_idx = (idx + 1) % sources.len();
                self.source_filter = Some(sources[next_idx].clone());
            } else {
                self.source_filter = Some(sources[0].clone());
            }
        } else {
            self.source_filter = Some(sources[0].clone());
        }
        self.dirty_filter = true;
    }

    pub fn on_tick(&mut self) {
        self.spinner.tick();
        self.tabs.update_animation();

        let log_diff = self.logs.len().saturating_sub(self.last_log_count);
        self.logs_per_sec = self.logs_per_sec * 0.7 + (log_diff as f64 * 60.0) * 0.3;
        self.last_log_count = self.logs.len();

        if self.new_logs_arrived || self.dirty_filter {
            self.update_filter();
            self.new_logs_arrived = false;
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let size = frame.size();
        let [tabs_area, main_area, status_area] = app_layout(size);

        self.tabs.render(frame, tabs_area);

        match self.tabs.index {
            0 => {
                // Dashboard: Logs
                let filter_active = !self.filter_query.is_empty() || self.level_filter.is_some();
                self.log_view.render(
                    frame,
                    main_area,
                    &self.filtered_logs,
                    &self.filter_query,
                    filter_active,
                );
            }
            1 => {
                let mut info_count = 0;
                let mut warn_count = 0;
                let mut error_count = 0;
                let mut debug_count = 0;
                let mut trace_count = 0;
                let mut unknown_count = 0;
                let mut source_counts: std::collections::HashMap<String, usize> =
                    std::collections::HashMap::new();

                for log in &self.logs {
                    *source_counts.entry(log.source_file.clone()).or_insert(0) += 1;
                    match log.level {
                        crate::core::models::LogLevel::Info => info_count += 1,
                        crate::core::models::LogLevel::Warn => warn_count += 1,
                        crate::core::models::LogLevel::Error => error_count += 1,
                        crate::core::models::LogLevel::Debug => debug_count += 1,
                        crate::core::models::LogLevel::Trace => trace_count += 1,
                        _ => unknown_count += 1,
                    }
                }

                let total = self.logs.len();
                let log_rate = if let Some(start) = self.stats_start_time {
                    let duration = chrono::Local::now().signed_duration_since(start);
                    let secs = duration.num_seconds().max(1) as f64;
                    (total as f64 / secs * 60.0) as usize
                } else {
                    0
                };

                let uptime = if let Some(start) = self.stats_start_time {
                    let duration = chrono::Local::now().signed_duration_since(start);
                    format!(
                        "{}m {}s",
                        duration.num_minutes(),
                        duration.num_seconds() % 60
                    )
                } else {
                    "N/A".to_string()
                };

                let mut stats_text = format!(
                    "=== Session Statistics ===\n\nUptime: {}\nTotal Logs: {}\nLogs/Min: {}\n\n=== By Level ===\n- ERROR : {:>5} ({:>5.1}%)\n- WARN  : {:>5} ({:>5.1}%)\n- INFO  : {:>5} ({:>5.1}%)\n- DEBUG : {:>5} ({:>5.1}%)\n- TRACE : {:>5} ({:>5.1}%)\n- UNK   : {:>5} ({:>5.1}%)\n",
                    uptime, total, log_rate,
                    error_count, if total > 0 { error_count as f64 / total as f64 * 100.0 } else { 0.0 },
                    warn_count, if total > 0 { warn_count as f64 / total as f64 * 100.0 } else { 0.0 },
                    info_count, if total > 0 { info_count as f64 / total as f64 * 100.0 } else { 0.0 },
                    debug_count, if total > 0 { debug_count as f64 / total as f64 * 100.0 } else { 0.0 },
                    trace_count, if total > 0 { trace_count as f64 / total as f64 * 100.0 } else { 0.0 },
                    unknown_count, if total > 0 { unknown_count as f64 / total as f64 * 100.0 } else { 0.0 }
                );

                if !source_counts.is_empty() {
                    stats_text.push_str("\n=== By Source ===\n");
                    let mut sources: Vec<_> = source_counts.iter().collect();
                    sources.sort_by(|a, b| b.1.cmp(a.1));
                    for (source, count) in sources.iter().take(5) {
                        let short_source: Vec<&str> = source.split('/').collect();
                        let short = short_source.last().copied().unwrap_or(source);
                        stats_text.push_str(&format!("- {}: {}\n", short, count));
                    }
                }

                let block = ratatui::widgets::Block::default()
                    .title(" Log Statistics ")
                    .borders(ratatui::widgets::Borders::ALL);
                let text = ratatui::widgets::Paragraph::new(stats_text).block(block);
                frame.render_widget(text, main_area);
            }
            2 => {
                let block = ratatui::widgets::Block::default()
                    .title(" Settings ")
                    .borders(ratatui::widgets::Borders::ALL);
                let settings_text = format!(
                    "Configuration\n\n- Max Log Lines: {}\n- Refresh Rate: {} FPS\n- Auto-scroll: {}\n- Show Timestamps: {}",
                    self.config.max_logs,
                    1000 / self.config.tick_rate_ms as usize,
                    if self.config.auto_scroll { "Yes" } else { "No" },
                    if self.config.show_timestamps { "Yes" } else { "No" }
                );
                let text = ratatui::widgets::Paragraph::new(settings_text).block(block);
                frame.render_widget(text, main_area);
            }
            3 => {
                // Help
                let block = ratatui::widgets::Block::default()
                    .title(" Keybindings Help ")
                    .borders(ratatui::widgets::Borders::ALL);
                let help_text = vec![
                    " [q / Esc]   : Quit Application",
                    " [Tab / →]   : Next Tab",
                    " [←]         : Previous Tab",
                    " [f]         : Enter Filter Mode",
                    " [p]         : Pause/Resume Log Streaming",
                    " [x]         : Export Filtered Logs",
                    " [r]         : Toggle Regex/Text Filter",
                    " [l]         : Cycle Level Filter (Error->Warn->Info->Debug->All)",
                    " [s]         : Cycle Source Filter",
                    " [c]         : Clear All Filters",
                    " [↑/↓ / j/k] : Scroll Logs (Manual Mode)",
                    " [gg / G]    : Top / Bottom",
                    " [End]       : Return to Auto-scroll Mode",
                    " [Enter]     : View Log Detail",
                    " [y]         : Copy Log Message",
                    "",
                    " While in Filter Mode:",
                    " [Enter]     : Confirm and return to Normal Mode",
                    " [Backspace] : Delete characters",
                    " [r]         : Toggle Regex/Text Filter",
                    " [l]         : Cycle Level Filter",
                    "",
                    " Export Mode:",
                    " [x]         : Enter Export Mode",
                    " [Enter]     : Save to file (default: koda-export-TIMESTAMP.log)",
                ]
                .join("\n");
                let text = ratatui::widgets::Paragraph::new(help_text).block(block);
                frame.render_widget(text, main_area);
            }
            _ => {}
        }

        // Display filter status if in Filtering mode
        let filter_mode = if self.use_regex_filter {
            "REGEX"
        } else {
            "TEXT"
        };

        let level_indicator = if let Some(ref level) = self.level_filter {
            format!("[LEVEL:{}] ", level.as_str())
        } else {
            String::new()
        };

        let source_indicator = if let Some(ref source) = self.source_filter {
            format!("[SOURCE:{}] ", source)
        } else {
            String::new()
        };

        let status_text = if self.input_mode == InputMode::Filtering {
            format!(
                " [FILTERING ({})] > {}█ (Esc/Enter to finish)",
                filter_mode, self.filter_query
            )
        } else if self.input_mode == InputMode::Export {
            format!(
                " [EXPORT] > {}█ (Enter to save, Esc to cancel)",
                self.export_path
            )
        } else if self.paused {
            format!(
                " {}{} [PAUSED] {}Files: {} | Logs: {}",
                level_indicator,
                source_indicator,
                if !self.filter_query.is_empty()
                    || self.level_filter.is_some()
                    || self.source_filter.is_some()
                {
                    format!("[FILTERED:{}] ", filter_mode)
                } else {
                    String::new()
                },
                self.files.len(),
                self.logs.len()
            )
        } else if !self.filter_query.is_empty()
            || self.level_filter.is_some()
            || self.source_filter.is_some()
        {
            format!(
                " {}{} [FILTERED:{}] : \"{}\" (f: Edit, r: Regex, l: Level, s: Source, c: Clear)",
                level_indicator, source_indicator, filter_mode, self.filter_query
            )
        } else {
            format!(
                " Files: {} | Logs: {} | Rate: {:.1}/s | [q:Quit p:Pause f:Filter x:Export]",
                self.files.len(),
                self.logs.len(),
                self.logs_per_sec
            )
        };

        self.spinner
            .render(frame, status_area, self.is_tailing, &status_text);

        if self.input_mode == InputMode::Detail {
            if let Some(entry) = &self.selected_log {
                let area = centered_rect(80, 80, size);
                frame.render_widget(Clear, area);

                let content = format!(
                    "Timestamp: {}\nSource: {}\nLevel: {:?}\n\nMessage:\n{}\n\nRaw:\n{}",
                    entry.timestamp, entry.source_file, entry.level, entry.message, entry.raw
                );

                let block = Block::default()
                    .title(" Log Detail ")
                    .borders(Borders::ALL)
                    .border_style(ratatui::style::Style::default().fg(entry.level.color()));

                let paragraph = Paragraph::new(content)
                    .block(block)
                    .wrap(Wrap { trim: true });

                frame.render_widget(paragraph, area);
            }
        }

        if let Some(ref msg) = self.export_message {
            let area = centered_rect(60, 10, size);
            frame.render_widget(Clear, area);

            let block = Block::default().title(" Export ").borders(Borders::ALL);

            let paragraph = Paragraph::new(msg.clone()).block(block);
            frame.render_widget(paragraph, area);
            self.export_message = None;
        }
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }
}
