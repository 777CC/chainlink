//! Raw input event types — engine-agnostic, produced by the platform layer.

use crate::{Key, MouseButton};
use gd_core::Vec2;

/// A single discrete input event.
#[derive(Debug, Clone)]
pub enum InputEvent {
    /// A keyboard key was pressed.
    KeyPressed { key: Key, repeat: bool },
    /// A keyboard key was released.
    KeyReleased { key: Key },
    /// A mouse button was pressed.
    MousePressed { button: MouseButton, position: Vec2 },
    /// A mouse button was released.
    MouseReleased { button: MouseButton, position: Vec2 },
    /// The mouse cursor moved.
    MouseMoved {
        position: Vec2,
        /// Relative motion since last frame.
        delta: Vec2,
    },
    /// Mouse wheel scrolled.
    MouseScrolled { delta: Vec2 },
    /// An OS-level action string (e.g. from `InputMap`).
    Action { name: String, pressed: bool },
}

impl InputEvent {
    /// Return `true` if this event is a press of the given key.
    pub fn is_key_pressed(&self, k: Key) -> bool {
        matches!(self, InputEvent::KeyPressed { key, .. } if *key == k)
    }

    /// Return `true` if this event is a release of the given key.
    pub fn is_key_released(&self, k: Key) -> bool {
        matches!(self, InputEvent::KeyReleased { key } if *key == k)
    }
}
