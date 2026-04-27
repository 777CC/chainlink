//! `DisplayServer` — window and display management queries.
//!
//! Mirrors Godot's `DisplayServer` singleton.  The actual window is owned by
//! `cl_engine` (via winit); this struct is a lightweight view that nodes can
//! read without touching winit directly.

use cl_core::Vec2;

/// Queryable state of the application window.
#[derive(Debug, Default)]
pub struct DisplayServer {
    /// Current window size in pixels.
    pub window_size: (u32, u32),
    /// Window title string.
    pub window_title: String,
    /// Whether the window is in fullscreen mode.
    pub fullscreen: bool,
}

impl DisplayServer {
    /// Create a new `DisplayServer` with the given initial dimensions and title.
    pub fn new(width: u32, height: u32, title: impl Into<String>) -> Self {
        Self {
            window_size: (width, height),
            window_title: title.into(),
            fullscreen: false,
        }
    }

    /// Current window width in pixels.
    pub fn width(&self) -> u32 { self.window_size.0 }

    /// Current window height in pixels.
    pub fn height(&self) -> u32 { self.window_size.1 }

    /// Window size as a [`Vec2`] (width, height).
    pub fn size_vec2(&self) -> Vec2 {
        Vec2::new(self.window_size.0 as f32, self.window_size.1 as f32)
    }
}
