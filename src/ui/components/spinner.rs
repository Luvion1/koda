use ratatui::{layout::Rect, Frame};
use throbber_widgets_tui::{Throbber, ThrobberState};

pub struct SpinnerComponent {
    pub state: ThrobberState,
}

impl SpinnerComponent {
    pub fn new() -> Self {
        Self {
            state: ThrobberState::default(),
        }
    }

    pub fn tick(&mut self) {
        self.state.calc_next();
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, active: bool, status_text: &str) {
        let label = if !status_text.is_empty() {
            status_text.to_string()
        } else if active {
            "Tailing logs... (Live)".to_string()
        } else {
            "Idle".to_string()
        };

        let throbber = Throbber::default().label(label);

        frame.render_stateful_widget(throbber, area, &mut self.state);
    }
}
