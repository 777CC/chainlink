//! **cl_input** — input abstraction layer for the Chainlink Game Engine.
//!
//! Provides:
//! - [`Key`] — keyboard key enum (mirrors Chainlink's `Key`)
//! - [`MouseButton`] — mouse button enum
//! - [`InputEvent`] — a single raw input event
//! - [`InputMap`] — action → key bindings
//! - [`InputServer`] — runtime state (pressed keys, mouse position, actions)

pub mod action;
pub mod event;
pub mod key;
pub mod mouse;
pub mod server;

pub use action::{InputAction, InputMap};
pub use event::InputEvent;
pub use key::Key;
pub use mouse::MouseButton;
pub use server::InputServer;
