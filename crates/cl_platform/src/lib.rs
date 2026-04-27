//! **cl_platform** — OS and display abstraction layer.
//!
//! Mirrors Godot's `platform/` directory.  Provides OS queries and a display
//! server interface that nodes use without touching winit directly.

pub mod display_server;
pub mod os;

pub use display_server::DisplayServer;
pub use os::OS;
