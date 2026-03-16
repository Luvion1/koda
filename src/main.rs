use anyhow::Result;
use clap::{Parser, ValueHint};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

use koda::app::config::Config;
use koda::app::events::run_event_loop;
use koda::app::state::AppState;
use koda::core::tailer::TailManager;

#[derive(Parser, Debug)]
#[command(name = "koda-tail")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A modern, high-performance log tailer with TUI", long_about = None)]
struct Args {
    /// Files to tail (supports glob patterns)
    #[arg(value_hint = ValueHint::FilePath)]
    files: Vec<PathBuf>,

    /// Maximum number of logs to keep in memory
    #[arg(short, long)]
    max_logs: Option<usize>,

    /// Enable auto-scroll to bottom
    #[arg(long)]
    auto_scroll: Option<bool>,

    /// Show timestamps in log entries
    #[arg(long)]
    timestamps: Option<bool>,

    /// Refresh rate in milliseconds (lower = faster)
    #[arg(short, long)]
    tick_rate: Option<u64>,

    /// Enable verbose logging
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Config file path
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    config: Option<PathBuf>,

    /// Generate a sample config file
    #[arg(long)]
    init_config: bool,
}

fn setup_logging(verbose: bool) -> Result<()> {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    let filter = if verbose {
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("debug,koda=debug"))
    } else {
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info,koda=debug"))
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(true).with_thread_ids(true))
        .init();

    tracing::info!("Koda-Tail starting up...");
    Ok(())
}

fn load_config(args: &Args) -> Result<Config> {
    let mut config = if let Some(ref config_path) = args.config {
        Config::from_file(config_path)?
    } else if let Some(default_path) = Config::default_config_path() {
        Config::from_file(&default_path).unwrap_or_default()
    } else {
        Config::default()
    };

    if let Some(max_logs) = args.max_logs {
        config.max_logs = max_logs;
    }
    if let Some(tick_rate) = args.tick_rate {
        config.tick_rate_ms = tick_rate;
    }
    if let Some(auto_scroll) = args.auto_scroll {
        config.auto_scroll = auto_scroll;
    }
    if let Some(timestamps) = args.timestamps {
        config.show_timestamps = timestamps;
    }

    Ok(config)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Err(e) = setup_logging(args.verbose) {
        eprintln!("Warning: Failed to setup logging: {}", e);
    }

    if args.init_config {
        let config = Config::default();
        if let Some(path) = Config::default_config_path() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            config.save(&path)?;
            println!("Created config file at: {}", path.display());
        } else {
            println!("Could not determine default config path");
        }
        return Ok(());
    }

    if args.files.is_empty() {
        println!("Koda-Tail v{}", env!("CARGO_PKG_VERSION"));
        println!("A modern log tailer with TUI");
        println!();
        println!("Usage: koda-tail <file1> <file2> ...");
        println!("       koda-tail /var/log/syslog");
        println!("       koda-tail --help for more options");
        println!();
        println!("Use --init-config to create a default config file");
        return Ok(());
    }

    let config = load_config(&args)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::unbounded_channel();

    let files_to_tail: Vec<String> = args.files.iter().map(|p| p.display().to_string()).collect();

    let tailer = TailManager::new();
    let tailer = Arc::new(Mutex::new(tailer));

    let tailer_clone = Arc::clone(&tailer);
    let files_clone = files_to_tail.clone();
    let tx_clone = tx.clone();

    tokio::spawn(async move {
        let tailer = tailer_clone.lock().await;
        if let Err(e) = tailer.tail_files(files_clone, tx_clone).await {
            tracing::error!("Tailing error: {}", e);
        }
    });

    let app = AppState::new(files_to_tail, config);
    let res = run_event_loop(app, rx, &mut terminal).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        tracing::error!("Application error: {}", err);
    }

    tracing::info!("Koda-Tail shutting down gracefully");
    Ok(())
}
