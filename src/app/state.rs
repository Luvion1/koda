use crate::app::config::Config;
use crate::core::models::LogEntry;
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
}

pub struct AppState {
    pub is_running: bool,
    pub input_mode: InputMode,
    pub filter_query: String,
    pub filter_regex: Option<Regex>,
    pub use_regex_filter: bool,
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
    pub last_key: Option<KeyCode>,
}

impl AppState {
    pub fn new(files: Vec<String>) -> Self {
        let config = Config::default();
        Self {
            is_running: true,
            input_mode: InputMode::Normal,
            filter_query: String::new(),
            filter_regex: None,
            use_regex_filter: false,
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
            last_key: None,
        }
    }

    pub fn push_log(&mut self, entry: LogEntry) {
        if self.logs.len() >= self.config.max_logs {
            self.logs.pop_front();
        }
        self.logs.push_back(entry);
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

            self.filtered_logs = self
                .logs
                .iter()
                .filter(|entry| {
                    if let Some(ref re) = self.filter_regex {
                        re.is_match(&entry.raw)
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();
        } else {
            self.filter_regex = None;
            self.filtered_logs = self
                .logs
                .iter()
                .filter(|entry| query.is_empty() || entry.raw.to_lowercase().contains(&query))
                .cloned()
                .collect();
        }

        self.dirty_filter = false;
    }

    pub fn on_tick(&mut self) {
        self.spinner.tick();
        self.tabs.update_animation();
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
                self.log_view
                    .render(frame, main_area, &self.filtered_logs, &self.filter_query);
            }
            1 => {
                // Stats
                let mut info_count = 0;
                let mut warn_count = 0;
                let mut error_count = 0;
                let mut debug_count = 0;
                let mut other_count = 0;

                for log in &self.logs {
                    match log.level {
                        crate::core::models::LogLevel::Info => info_count += 1,
                        crate::core::models::LogLevel::Warn => warn_count += 1,
                        crate::core::models::LogLevel::Error => error_count += 1,
                        crate::core::models::LogLevel::Debug => debug_count += 1,
                        _ => other_count += 1,
                    }
                }

                let total = self.logs.len();
                let stats_text = format!(
                    "Total Logs: {}\n\n- INFO  : {}\n- WARN  : {}\n- ERROR : {}\n- DEBUG : {}\n- OTHER : {}",
                    total, info_count, warn_count, error_count, debug_count, other_count
                );

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
                    " [q / Esc] : Quit Application",
                    " [Tab / →] : Next Tab",
                    " [←]       : Previous Tab",
                    " [f]       : Enter Filter Mode",
                    " [r]       : Toggle Regex/Text Filter",
                    " [c]       : Clear Filter",
                    " [↑/↓ / j/k]: Scroll Logs (Manual Mode)",
                    " [gg / G]  : Top / Bottom",
                    " [End]     : Return to Auto-scroll Mode",
                    " [Enter]   : View Log Detail",
                    "",
                    " While in Filter Mode:",
                    " [Enter]   : Confirm and return to Normal Mode",
                    " [Backspace]: Delete characters",
                    " [r]       : Toggle Regex/Text Filter",
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
        let status_text = if self.input_mode == InputMode::Filtering {
            format!(
                " [FILTERING ({})] > {}█ (Esc/Enter to finish)",
                filter_mode, self.filter_query
            )
        } else if !self.filter_query.is_empty() {
            format!(
                " [FILTERED:{}] : \"{}\" (f: Edit, r: Regex, c: Clear)",
                filter_mode, self.filter_query
            )
        } else {
            String::new()
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
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }
}
