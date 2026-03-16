# Koda-Tail

A modern, high-performance log tailer with a beautiful Terminal User Interface (TUI), built with Rust and Ratatui.

## Features

- **Real-time Tailing**: Monitor multiple files simultaneously using `linemux`
- **Smart Parsing**: Automatically detects and parses JSON and common plain-text log formats
- **Interactive Dashboard**:
  - **Filter & Search**: Instant filtering with search term highlighting
  - **Regex Support**: Toggle between text and regex filtering
  - **Log Details**: Select a log and press `Enter` to see full metadata
  - **Statistics**: Real-time distribution of log levels (Info, Warn, Error, Debug)
- **Vim Navigation**: Support for `j/k`, `gg`, and `G` for fast log traversal
- **Smooth UI**: Animated transitions and spinners for a polished experience
- **Configurable**: Adjust max logs, refresh rate via settings

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
| `r` | Toggle Regex/Text Filter |
| `c` | Clear Filter |
| `Enter` | View Log Detail (Dashboard) / Confirm Filter |
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

## Configuration

Access the Settings tab to view:
- Max Log Lines (default: 1000)
- Refresh Rate (default: 60 FPS)
- Auto-scroll status

## Architecture

```
koda/
├── src/
│   ├── app/           # Application logic
│   │   ├── config.rs  # Configuration management
│   │   ├── events.rs # Event loop handling
│   │   └── state.rs  # Application state
│   ├── core/          # Core business logic
│   │   ├── tailer.rs # File tailing engine
│   │   ├── parser.rs # Log parsing
│   │   └── models.rs # Data models
│   ├── ui/            # TUI components
│   │   ├── components/  # UI widgets
│   │   ├── layout.rs     # Layout utilities
│   │   └── theme.rs      # Theme styling
│   └── utils/         # Utilities
│       └── anim.rs    # Animation helpers
└── Cargo.toml
```

## Performance

- Efficient filtering: Only re-calculates when new logs arrive
- Async I/O: Uses tokio for non-blocking file operations
- Smooth animations: 60 FPS rendering with easing

## License

MIT License. See [LICENSE](LICENSE) for details.

---

Built with Rust
