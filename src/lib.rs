//! Koda is a modern log tailer with a beautiful TUI.
//!
//! This library provides the core logic for the Koda application,
//! including log parsing, tailing management, and the TUI state machine.
//!
//! ## Modules
//!
//! - `app` - Application state, events, and configuration
//! - `core` - Core business logic (tailer, parser, models)
//! - `ui` - TUI components, layout, and theming
//! - `utils` - Utility functions (animations)

pub mod app;
pub mod core;
pub mod ui;
pub mod utils;
