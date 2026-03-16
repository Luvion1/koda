use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;
use tokio::sync::mpsc;
use crate::core::models::LogEntry;
use super::state::AppState;

pub enum AppEvent {
    Tick,
    Input(KeyCode),
    NewLog(LogEntry),
}

pub async fn run_event_loop(
    mut app: AppState,
    mut rx: mpsc::UnboundedReceiver<LogEntry>,
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
) -> anyhow::Result<()> {
    // 60 FPS tick rate for smooth animations
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = std::time::Instant::now();

    while app.is_running {
        terminal.draw(|f| app.draw(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        tokio::select! {
            // 1. Handle UI events (Keyboard)
            event_result = tokio::task::spawn_blocking(move || {
                if event::poll(timeout).unwrap_or(false) {
                    if let Event::Key(key) = event::read().unwrap() {
                        if key.kind == KeyEventKind::Press {
                            return Some(AppEvent::Input(key.code));
                        }
                    }
                }
                None
            }) => {
                if let Ok(Some(AppEvent::Input(keycode))) = event_result {
                    handle_input(&mut app, keycode);
                }
            }

            // 2. Handle incoming logs from tailer
            Some(entry) = rx.recv() => {
                app.push_log(entry);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = std::time::Instant::now();
        }
    }

    Ok(())
}

use crate::app::state::InputMode;

fn handle_input(app: &mut AppState, keycode: KeyCode) {
    let prev_key = app.last_key;
    
    // Reset last_key for most keys, keep it for sequential keys like 'g'
    app.last_key = Some(keycode);

    match app.input_mode {
        InputMode::Normal => match keycode {
            KeyCode::Char('q') | KeyCode::Esc => app.quit(),
            KeyCode::Char('f') => app.input_mode = InputMode::Filtering,
            KeyCode::Char('p') => {
                app.toggle_pause();
                app.last_key = None;
            }
            KeyCode::Char('x') => {
                app.input_mode = InputMode::Export;
                app.last_key = None;
            }
            KeyCode::Char('r') => {
                app.use_regex_filter = !app.use_regex_filter;
                app.dirty_filter = true;
                app.last_key = None;
            }
            KeyCode::Char('l') => {
                app.level_filter = match app.level_filter {
                    None => Some(crate::core::models::LogLevel::Error),
                    Some(crate::core::models::LogLevel::Error) => Some(crate::core::models::LogLevel::Warn),
                    Some(crate::core::models::LogLevel::Warn) => Some(crate::core::models::LogLevel::Info),
                    Some(crate::core::models::LogLevel::Info) => Some(crate::core::models::LogLevel::Debug),
                    Some(crate::core::models::LogLevel::Debug) => None,
                    _ => None,
                };
                app.dirty_filter = true;
                app.last_key = None;
            }
            KeyCode::Char('c') => {
                app.filter_query.clear();
                app.filter_regex = None;
                app.level_filter = None;
                app.source_filter = None;
                app.dirty_filter = true;
                app.last_key = None;
            }
            KeyCode::Char('s') => {
                app.cycle_source_filter();
                app.last_key = None;
            }
            KeyCode::Tab | KeyCode::Right => app.tabs.next(),
            KeyCode::BackTab | KeyCode::Left => app.tabs.previous(),
            KeyCode::Up | KeyCode::Char('k') => {
                app.log_view.auto_scroll = false;
                let current = app.log_view.scroll_anim.target;
                app.log_view.scroll_anim.set_target((current - 1.0).max(0.0));
                app.last_key = None;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.log_view.auto_scroll = false;
                let current = app.log_view.scroll_anim.target;
                let max = app.filtered_logs.len().saturating_sub(1) as f64;
                app.log_view.scroll_anim.set_target((current + 1.0).min(max));
                app.last_key = None;
            }
            KeyCode::Char('g') => {
                if let Some(KeyCode::Char('g')) = prev_key {
                    // gg pressed - jump to top
                    app.log_view.auto_scroll = false;
                    app.log_view.scroll_anim.set_target(0.0);
                }
                // Don't reset last_key here - we want to wait for the second 'g'
            }
            KeyCode::Char('G') => {
                app.log_view.auto_scroll = false;
                let max = app.filtered_logs.len().saturating_sub(1) as f64;
                app.log_view.scroll_anim.set_target(max.max(0.0));
                app.last_key = None;
            }
            KeyCode::End => {
                app.log_view.auto_scroll = true;
                app.last_key = None;
            }
            KeyCode::Enter => {
                if let Some(index) = app.log_view.state.selected() {
                    if index < app.filtered_logs.len() {
                        app.selected_log = Some(app.filtered_logs[index].clone());
                        app.input_mode = InputMode::Detail;
                    }
                }
                app.last_key = None;
            }
            KeyCode::Char('y') => {
                app.copy_log_to_clipboard();
                app.last_key = None;
            }
            _ => {
                app.last_key = None;
            }
        },
        InputMode::Filtering => match keycode {
            KeyCode::Enter | KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
                app.last_key = None;
            }
            KeyCode::Char(c) => {
                app.filter_query.push(c);
                app.dirty_filter = true;
            }
            KeyCode::Backspace => {
                app.filter_query.pop();
                app.dirty_filter = true;
            }
            _ => {
                app.last_key = None;
            }
        },
        InputMode::Detail => match keycode {
            KeyCode::Enter | KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
                app.last_key = None;
            }
            _ => {
                app.last_key = None;
            }
        },
        InputMode::Export => match keycode {
            KeyCode::Enter => {
                if let Err(e) = app.export_logs() {
                    app.export_message = Some(format!("Export failed: {}", e));
                }
                app.last_key = None;
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
                app.export_path.clear();
                app.last_key = None;
            }
            KeyCode::Char(c) => {
                app.export_path.push(c);
            }
            KeyCode::Backspace => {
                app.export_path.pop();
            }
            _ => {
                app.last_key = None;
            }
        },
    }
}
