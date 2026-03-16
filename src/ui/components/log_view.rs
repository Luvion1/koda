use crate::core::models::LogEntry;
use crate::ui::theme::Theme;
use crate::utils::anim::AnimatedValue;
use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

pub struct LogViewComponent {
    pub state: ListState,
    pub auto_scroll: bool,
    pub scroll_anim: AnimatedValue,
}

impl Default for LogViewComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl LogViewComponent {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            auto_scroll: true,
            scroll_anim: AnimatedValue::new(0.0),
        }
    }

    pub fn update_scroll(&mut self, item_count: usize, height: usize) {
        if item_count == 0 {
            self.state.select(None);
            return;
        }

        if height == 0 {
            return;
        }

        if self.auto_scroll {
            let max_scroll = item_count.saturating_sub(height);
            self.scroll_anim.set_target(max_scroll as f64);
        }

        self.scroll_anim.update(0.15);

        let scroll_pos = self.scroll_anim.current.round() as usize;
        let selected = scroll_pos.min(item_count.saturating_sub(1));
        self.state.select(Some(selected));
    }

    fn highlight_text<'a>(text: &'a str, query: &str) -> Vec<Span<'a>> {
        let mut spans = Vec::new();
        if query.is_empty() {
            spans.push(Span::raw(text));
            return spans;
        }

        let query_lower = query.to_lowercase();
        let text_lower = text.to_lowercase();

        let mut last_idx = 0;
        for (start, _) in text_lower.match_indices(&query_lower) {
            if start > last_idx {
                spans.push(Span::raw(&text[last_idx..start]));
            }
            let end = start + query.len();
            spans.push(Span::styled(
                &text[start..end],
                Style::default()
                    .bg(ratatui::style::Color::Yellow)
                    .fg(ratatui::style::Color::Black),
            ));
            last_idx = end;
        }
        if last_idx < text.len() {
            spans.push(Span::raw(&text[last_idx..]));
        }

        spans
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, logs: &[LogEntry], filter: &str) {
        let items: Vec<ListItem> = logs
            .iter()
            .map(|entry| {
                let time = Span::styled(
                    entry.timestamp.format("%H:%M:%S").to_string() + " ",
                    Theme::log_timestamp_style(),
                );
                let level = Span::styled(
                    format!("[{}] ", entry.level.as_str()),
                    Style::default().fg(entry.level.color()),
                );

                // Truncate source path for brevity in display
                let source_parts: Vec<&str> = entry.source_file.split('/').collect();
                let short_source = source_parts.last().unwrap_or(&"");
                let source = Span::styled(format!("{short_source}: "), Theme::log_source_style());

                let mut spans = vec![time, level, source];

                // Highlight search term in message
                let message_spans = Self::highlight_text(&entry.message, filter);
                spans.extend(message_spans);

                ListItem::new(Line::from(spans))
            })
            .collect();

        // Calculate available height for scrolling logic
        let list_height = area.height.saturating_sub(2) as usize; // account for borders
        self.update_scroll(items.len(), list_height);

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Logs"))
            .highlight_style(Style::default().bg(ratatui::style::Color::DarkGray));

        // We use state to handle the selection/scrolling visual
        frame.render_stateful_widget(list, area, &mut self.state);
    }
}
