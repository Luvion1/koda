# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2] - 2024

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
