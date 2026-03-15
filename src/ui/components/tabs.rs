use crate::ui::theme::Theme;
use crate::utils::anim::AnimatedValue;
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Tabs as RatatuiTabs},
    Frame,
};

pub struct TabsComponent {
    pub titles: Vec<String>,
    pub index: usize,
    pub anim_index: AnimatedValue,
}

impl TabsComponent {
    pub fn new(titles: Vec<String>) -> Self {
        Self {
            titles,
            index: 0,
            anim_index: AnimatedValue::new(0.0),
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
        self.anim_index.set_target(self.index as f64);
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
        self.anim_index.set_target(self.index as f64);
    }

    pub fn update_animation(&mut self) {
        self.anim_index.update(0.2);
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let titles: Vec<Line> = self
            .titles
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let dist = (i as f64 - self.anim_index.current).abs();
                let label = if dist < 0.1 {
                    format!("● {}", t)
                } else if dist < 0.5 {
                    format!("○ {}", t)
                } else {
                    format!("  {}", t)
                };
                Line::from(label)
            })
            .collect();

        let tabs = RatatuiTabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Log-Tailer")
                    .title_style(Theme::title_style()),
            )
            .select(self.index)
            .highlight_style(Theme::tab_active_style())
            .divider(" | ");

        frame.render_widget(tabs, area);
    }
}
