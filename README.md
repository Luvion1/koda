# Koda 🚀

A modern, high-performance log tailer with a beautiful Terminal User Interface (TUI), built with Rust and Ratatui.

![Koda Screenshot Placeholder](https://via.placeholder.com/800x400?text=Koda+TUI+Screenshot)

## Features

- **Real-time Tailing**: Monitor multiple files simultaneously using `linemux`.
- **Smart Parsing**: Automatically detects and parses JSON and common plain-text log formats.
- **Interactive Dashboard**:
    - **Filter & Search**: Instant filtering with search term highlighting.
    - **Log Details**: Select a log and press `Enter` to see full metadata and multi-line messages in a popup.
    - **Statistics**: Real-time distribution of log levels (Info, Warn, Error, Debug).
- **Vim Navigation**: Support for `j/k`, `gg`, and `G` for fast log traversal.
- **Smooth UI**: Animated transitions and spinners for a polished experience.

## Installation

### From Crates.io
```bash
cargo install koda
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
koda /var/log/syslog ./app.log
```

### Keybindings

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `Tab` / `→` | Next Tab |
| `←` | Previous Tab |
| `f` | Enter Filter Mode |
| `c` | Clear Filter |
| `Enter` | View Log Detail (Dashboard) / Confirm Filter |
| `j` / `↓` | Scroll Down |
| `k` / `↑` | Scroll Up |
| `gg` | Jump to Top |
| `G` | Jump to Bottom |
| `End` | Return to Auto-scroll Mode |

## License

MIT License. See [LICENSE](LICENSE) for details.

---
Built with ❤️ by Luvion using Rust.
