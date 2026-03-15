use ratatui::style::{Color, Style};

pub struct Theme;

impl Theme {
    pub fn title_style() -> Style {
        Style::default().fg(Color::Cyan)
    }

    pub fn tab_active_style() -> Style {
        Style::default().fg(Color::Yellow).bg(Color::DarkGray)
    }

    pub fn tab_inactive_style() -> Style {
        Style::default().fg(Color::Gray)
    }

    pub fn log_timestamp_style() -> Style {
        Style::default().fg(Color::DarkGray)
    }

    pub fn log_source_style() -> Style {
        Style::default().fg(Color::Magenta)
    }
}
