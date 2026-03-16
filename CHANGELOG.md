# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2026-03-16

### Added
- **Timestamp Parsing**: Fixed timestamp parsing to correctly extract timestamps from log entries (both plain text and JSON formats)
- **Pause/Resume**: Press `p` to pause and resume log streaming
- **Log Export**: Press `x` to export filtered logs to a file (default: koda-export-TIMESTAMP.log)
- **Source Filter**: Press `s` to cycle through source file filters
- **Enhanced Statistics**: Added session uptime, logs/minute rate, percentage breakdown by level, and top 5 sources
- **Status Bar Improvements**: Shows real-time log rate (logs/second), file count, and total logs
- **Copy Log Message**: Press `y` to copy selected log message to clipboard file (/tmp/koda-clipboard.txt)

### Improved
- **Stats Tracking**: Real-time calculation of logs per second with smooth averaging
- **Filter System**: Added source file filtering alongside existing text/regex and level filters

### Testing
- Added 2 new tests for timestamp parsing (total: 9 tests)
- All tests pass

## [0.2.0] - 2026-03-16

### Added
- **Professional CLI**: Added clap for robust command-line argument parsing
- **Log Level Filtering**: Press `l` to cycle through log level filters (Error -> Warn -> Info -> Debug -> All)
- **Application Logging**: Added tracing for debugging and monitoring
- **Configuration File Support**: TOML-based configuration with `--init-config` option
- **Command-line Options**:
  - `--max-logs <N>`: Maximum logs to keep in memory (default: 1000)
  - `--auto-scroll <bool>`: Enable/disable auto-scroll (default: true)
  - `--timestamps <bool>`: Show/hide timestamps (default: true)
  - `--tick-rate <ms>`: Refresh rate in milliseconds (default: 16)
  - `--verbose`: Enable verbose logging
  - `--config <path>`: Config file path
  - `--init-config`: Generate a default config file
  - `--version`: Show version information

### Improved
- **Error Handling**: Better error messages and graceful shutdown
- **Status Bar**: Shows active level filter alongside text/regex filter
- **Help Documentation**: Updated to include level filter keybinding

### Testing
- Added unit tests for Config module (4 new tests)
- All tests pass: 7 total

## [0.1.2] - 2026-03-16

### Added
- **Regex Filter Support**: Toggle between text and regex filtering with `r` key
- **Configuration System**: Configurable max logs, tick rate, auto-scroll, and timestamps
- **Efficient Filtering**: Filter only recalculates when new logs arrive
- **Settings Tab**: View and monitor configuration settings
- **File Rotation Handling**: Better error handling for file rotation scenarios

### Fixed
- **Bug Fix**: Corrected empty files check logic in tailer
- **Bug Fix**: Improved error handling for file read operations
- **Bug Fix**: Fixed off-by-one error in scroll calculation

### Improved
- **Performance**: Reduced CPU usage with optimized filter updates
- **UX**: Status bar shows current filter mode (TEXT/REGEX)
- **Help Documentation**: Added regex filter documentation in Help tab

## [0.1.0] - Initial Release

### Added
- Real-time multi-file tailing
- JSON and plain-text log parsing
- Interactive dashboard with filtering
- Vim-style navigation (j/k/gg/G)
- Log detail popup
- Statistics tab
- Smooth animated UI
