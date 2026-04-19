//! `InputServer` — the engine's input singleton.
//!
//! The platform layer feeds raw `winit` events → `InputEvent`s into this
//! server each frame.  Game code queries it synchronously (like Chainlink's
//! `Input` singleton).

use std::collections::HashSet;
use cl_core::Vec2;
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

    // ── Query API (mirrors Chainlink's `Input` singleton) ─────────────────────────

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{InputEvent, Key, MouseButton};
    use cl_core::Vec2;

    fn server_with_actions() -> InputServer {
        let mut s = InputServer::new();
        s.map.add_action("left",  Key::A);
        s.map.add_action("right", Key::D);
        s.map.add_action("jump",  Key::Space);
        s
    }

    // ── Key state ─────────────────────────────────────────────────────────────

    #[test]
    fn key_pressed_and_released() {
        let mut s = InputServer::new();
        s.handle_event(InputEvent::KeyPressed { key: Key::W, repeat: false });
        assert!(s.is_key_pressed(Key::W));
        s.handle_event(InputEvent::KeyReleased { key: Key::W });
        assert!(!s.is_key_pressed(Key::W));
    }

    #[test]
    fn mouse_button_pressed_and_released() {
        let mut s = InputServer::new();
        s.handle_event(InputEvent::MousePressed {
            button: MouseButton::Left,
            position: Vec2::ZERO,
        });
        assert!(s.is_mouse_button_pressed(MouseButton::Left));
        s.handle_event(InputEvent::MouseReleased {
            button: MouseButton::Left,
            position: Vec2::ZERO,
        });
        assert!(!s.is_mouse_button_pressed(MouseButton::Left));
    }

    #[test]
    fn mouse_moved_updates_position_and_delta() {
        let mut s = InputServer::new();
        s.handle_event(InputEvent::MouseMoved {
            position: Vec2::new(100.0, 200.0),
            delta:    Vec2::new(5.0, 10.0),
        });
        assert_eq!(s.mouse_position(), Vec2::new(100.0, 200.0));
        assert_eq!(s.mouse_delta(),    Vec2::new(5.0, 10.0));
    }

    // ── Action API ────────────────────────────────────────────────────────────

    #[test]
    fn is_action_pressed_true_when_key_held() {
        let mut s = server_with_actions();
        s.handle_event(InputEvent::KeyPressed { key: Key::Space, repeat: false });
        assert!(s.is_action_pressed("jump"));
        assert!(!s.is_action_pressed("left"));
    }

    #[test]
    fn is_action_just_pressed_this_frame_only() {
        let mut s = server_with_actions();
        s.handle_event(InputEvent::KeyPressed { key: Key::A, repeat: false });
        assert!(s.is_action_just_pressed("left"));
        // after flush the event is gone
        s.flush();
        assert!(!s.is_action_just_pressed("left"));
    }

    #[test]
    fn is_action_just_released_this_frame_only() {
        let mut s = server_with_actions();
        s.handle_event(InputEvent::KeyPressed  { key: Key::A, repeat: false });
        s.handle_event(InputEvent::KeyReleased { key: Key::A });
        assert!(s.is_action_just_released("left"));
        s.flush();
        assert!(!s.is_action_just_released("left"));
    }

    // ── get_axis ──────────────────────────────────────────────────────────────

    #[test]
    fn get_axis_neither_pressed_is_zero() {
        let s = server_with_actions();
        assert_eq!(s.get_axis("left", "right"), 0.0);
    }

    #[test]
    fn get_axis_positive_action_pressed() {
        let mut s = server_with_actions();
        s.handle_event(InputEvent::KeyPressed { key: Key::D, repeat: false });
        assert_eq!(s.get_axis("left", "right"), 1.0);
    }

    #[test]
    fn get_axis_negative_action_pressed() {
        let mut s = server_with_actions();
        s.handle_event(InputEvent::KeyPressed { key: Key::A, repeat: false });
        assert_eq!(s.get_axis("left", "right"), -1.0);
    }

    #[test]
    fn get_axis_both_pressed_cancels_out() {
        let mut s = server_with_actions();
        s.handle_event(InputEvent::KeyPressed { key: Key::A, repeat: false });
        s.handle_event(InputEvent::KeyPressed { key: Key::D, repeat: false });
        assert_eq!(s.get_axis("left", "right"), 0.0);
    }

    // ── flush ─────────────────────────────────────────────────────────────────

    #[test]
    fn flush_clears_events_and_delta() {
        let mut s = InputServer::new();
        s.handle_event(InputEvent::MouseMoved {
            position: Vec2::new(10.0, 10.0),
            delta:    Vec2::new(5.0, 5.0),
        });
        s.flush();
        assert!(s.events.is_empty());
        assert_eq!(s.mouse_delta(), Vec2::ZERO);
    }
}
