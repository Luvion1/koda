# Contributing to Koda-Tail

Thank you for your interest in contributing!

## Development Setup

```bash
# Clone the repository
git clone https://github.com/Luvion1/koda
cd koda

# Build the project
cargo build

# Run tests
cargo test

# Run with debug output
cargo run -- /var/log/syslog
```

## Code Style

- Follow standard Rust conventions
- Use `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes

## Project Structure

- `src/app/` - Application state and event loop
- `src/core/` - Core business logic (tailing, parsing)
- `src/ui/` - TUI components and theming
- `src/utils/` - Utility functions

## Adding New Features

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Reporting Issues

Please include:
- Rust version (`rustc --version`)
- OS and terminal details
- Steps to reproduce
- Expected vs actual behavior
