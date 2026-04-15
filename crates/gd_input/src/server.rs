//! `InputServer` — the engine's input singleton.
//!
//! The platform layer feeds raw `winit` events → `InputEvent`s into this
//! server each frame.  Game code queries it synchronously (like Godot's
//! `Input` singleton).

use std::collections::HashSet;
use gd_core::Vec2;
use crate::{InputEvent, InputMap, Key, MouseButton};

/// Runtime input state.  One instance lives for the lifetime of the engine.
#[derive(Debug, Default)]
pub struct InputServer {
    pressed_keys:    HashSet<Key>,
    pressed_buttons: HashSet<MouseButton>,
    mouse_position:  Vec2,
    mouse_delta:     Vec2,

    /// Events accumulated this frame; drained at the start of the next.
    pub events: Vec<InputEvent>,

    pub map: InputMap,
}

impl InputServer {
    pub fn new() -> Self { Self::default() }

    // ── Feed from platform ────────────────────────────────────────────────────

    /// Process a raw input event, updating internal state and pushing to `events`.
    pub fn handle_event(&mut self, ev: InputEvent) {
        match &ev {
            InputEvent::KeyPressed  { key, .. } => { self.pressed_keys.insert(*key); }
            InputEvent::KeyReleased { key }     => { self.pressed_keys.remove(key); }
            InputEvent::MousePressed { button, .. } => { self.pressed_buttons.insert(*button); }
            InputEvent::MouseReleased { button, .. } => { self.pressed_buttons.remove(button); }
            InputEvent::MouseMoved { position, delta } => {
                self.mouse_position = *position;
                self.mouse_delta    = *delta;
            }
            _ => {}
        }
        self.events.push(ev);
    }

    /// Call at the end of each frame to reset per-frame state.
    pub fn flush(&mut self) {
        self.events.clear();
        self.mouse_delta = Vec2::ZERO;
    }

    // ── Query API (mirrors Godot's `Input` singleton) ─────────────────────────

    /// Is `key` currently held down?
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    /// Is `button` currently held down?
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_buttons.contains(&button)
    }

    /// Current mouse cursor position (in window pixels).
    pub fn mouse_position(&self) -> Vec2 { self.mouse_position }

    /// Mouse movement since last frame.
    pub fn mouse_delta(&self) -> Vec2 { self.mouse_delta }

    /// Is the named action currently active (any bound key held)?
    pub fn is_action_pressed(&self, action: &str) -> bool {
        self.map
            .keys_for(action)
            .map_or(false, |keys| keys.iter().any(|k| self.pressed_keys.contains(k)))
    }

    /// Was the named action pressed THIS frame?
    pub fn is_action_just_pressed(&self, action: &str) -> bool {
        let Some(keys) = self.map.keys_for(action) else { return false };
        self.events.iter().any(|ev| {
            if let InputEvent::KeyPressed { key, .. } = ev {
                keys.contains(key)
            } else {
                false
            }
        })
    }

    /// Was the named action released THIS frame?
    pub fn is_action_just_released(&self, action: &str) -> bool {
        let Some(keys) = self.map.keys_for(action) else { return false };
        self.events.iter().any(|ev| {
            if let InputEvent::KeyReleased { key } = ev {
                keys.contains(key)
            } else {
                false
            }
        })
    }

    /// Returns a value in `[-1, 1]` based on two opposing actions.
    /// Positive when `positive_action` is pressed, negative when `negative_action` is pressed.
    pub fn get_axis(&self, negative_action: &str, positive_action: &str) -> f32 {
        let neg: f32 = if self.is_action_pressed(negative_action) { -1.0 } else { 0.0 };
        let pos: f32 = if self.is_action_pressed(positive_action) {  1.0 } else { 0.0 };
        (neg + pos).clamp(-1.0, 1.0)
    }
}
