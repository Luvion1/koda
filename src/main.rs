use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::env;
use std::io;
use tokio::sync::mpsc;

use koda::app::events::run_event_loop;
use koda::app::state::AppState;
use koda::core::tailer::TailManager;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Setup CLI args (files to tail)
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Koda-Tail - A modern log tailer");
        println!("Usage: koda-tail <file1> <file2> ...");
        return Ok(());
    }

    // 2. Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 3. Setup communication channels
    let (tx, rx) = mpsc::unbounded_channel();

    // 4. Start Tailing Engine in a background task
    let files_to_tail = args.clone();
    tokio::spawn(async move {
        let tailer = TailManager::new();
        if let Err(e) = tailer.tail_files(files_to_tail, tx).await {
            eprintln!("Tailing error: {}", e);
        }
    });

    // 5. Initialize App State & Event Loop (TUI)
    let app = AppState::new(args);
    let res = run_event_loop(app, rx, &mut terminal).await;

    // 6. Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
