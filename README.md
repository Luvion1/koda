# Koda-Tail

A modern, high-performance log tailer with a beautiful Terminal User Interface (TUI), built with Rust and Ratatui.

## Features

- **Real-time Tailing**: Monitor multiple files simultaneously using `linemux`
- **Smart Parsing**: Automatically detects and parses JSON and common plain-text log formats with proper timestamp extraction
- **Interactive Dashboard**:
  - **Filter & Search**: Instant filtering with search term highlighting
  - **Regex Support**: Toggle between text and regex filtering
  - **Log Details**: Select a log and press `Enter` to see full metadata
  - **Statistics**: Real-time distribution of log levels with percentage breakdown and logs/minute rate
- **Vim Navigation**: Support for `j/k`, `gg`, and `G` for fast log traversal
- **Pause/Resume**: Press `p` to pause and resume log streaming
- **Log Export**: Press `x` to export filtered logs to a file
- **Source Filtering**: Press `s` to cycle through source file filters
- **Smooth UI**: Animated transitions and spinners for a polished experience
- **Real-time Stats**: Live log rate display in status bar

## Installation

### From Crates.io

```bash
cargo install koda-tail
```

### From Source

```bash
git clone https://github.com/Luvion1/koda
cd koda
cargo build --release
```

## Usage

Run Koda by passing the files you want to tail:

```bash
koda-tail /var/log/syslog ./app.log
```

### Multiple Files

```bash
koda-tail /var/log/*.log
```

### Keybindings

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit Application |
| `Tab` / `→` | Next Tab |
| `←` | Previous Tab |
| `f` | Enter Filter Mode |
| `p` | Pause/Resume Log Streaming |
| `x` | Export Filtered Logs |
| `r` | Toggle Regex/Text Filter |
| `l` | Cycle Level Filter (Error→Warn→Info→Debug→All) |
| `s` | Cycle Source Filter |
| `c` | Clear All Filters |
| `Enter` | View Log Detail (Dashboard) / Confirm Filter |
| `y` | Copy Log Message |
| `j` / `↓` | Scroll Down |
| `k` / `↑` | Scroll Up |
| `gg` | Jump to Top |
| `G` | Jump to Bottom |
| `End` | Return to Auto-scroll Mode |

### Filter Mode

When in filter mode:
- Type your search query
- Press `Enter` or `Esc` to confirm
- Press `r` to toggle between text and regex filtering

Regex example: `error|warn.*connection`

### Export

Press `x` to enter export mode:
- Default filename: `koda-export-TIMESTAMP.log`
- Custom filename: Type your desired filename and press Enter

## Configuration

Access the Settings tab to view:
- Max Log Lines (default: 1000)
- Refresh Rate (default: 60 FPS)
- Auto-scroll status

### CLI Options

```bash
koda-tail --help
```

Options:
- `--max-logs <N>`: Maximum logs to keep in memory (default: 1000)
- `--auto-scroll <bool>`: Enable/disable auto-scroll (default: true)
- `--timestamps <bool>`: Show/hide timestamps (default: true)
- `--tick-rate <ms>`: Refresh rate in milliseconds (default: 16)
- `--verbose`: Enable verbose logging
- `--config <path>`: Config file path
- `--init-config`: Generate a default config file

## Statistics Tab

The Stats tab provides:
- Session uptime
- Total logs received
- Logs per minute rate
- Percentage breakdown by level (ERROR, WARN, INFO, DEBUG, TRACE, UNK)
- Top 5 sources by log count

## Architecture

```
koda/
├── src/
│   ├── app/           # Application logic
│   │   ├── config.rs  # Configuration management
│   │   ├── events.rs  # Event loop handling
│   │   └── state.rs   # Application state
│   ├── core/          # Core business logic
│   │   ├── tailer.rs  # File tailing engine
│   │   ├── parser.rs  # Log parsing
│   │   └── models.rs  # Data models
│   ├── ui/            # TUI components
│   │   ├── components/  # UI widgets
│   │   ├── layout.rs    # Layout utilities
│   │   └── theme.rs     # Theme styling
│   └── utils/         # Utilities
│       └── anim.rs     # Animation helpers
├── .github/           # GitHub workflows
└── Cargo.toml
```

## Performance

- Efficient filtering: Only re-calculates when new logs arrive
- Async I/O: Uses tokio for non-blocking file operations
- Smooth animations: 60 FPS rendering with easing
- Smart memory management: Configurable max log buffer size

## License

MIT License. See [LICENSE](LICENSE) for details.

---

Built with Rust
