//! Action map — binds named actions to one or more physical keys.
//!
//! This mirrors Chainlink's `InputMap` singleton.

use std::collections::{HashMap, HashSet};
use crate::Key;

/// A named gameplay action (e.g. `"jump"`, `"move_left"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputAction(pub String);

impl InputAction {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
    pub fn name(&self) -> &str { &self.0 }
}

/// Maps action names to sets of triggering keys.
#[derive(Debug, Default)]
pub struct InputMap {
    bindings: HashMap<String, HashSet<Key>>,
}

impl InputMap {
    pub fn new() -> Self { Self::default() }

    /// Bind a key to an action.  Creates the action if it doesn't exist.
    pub fn add_action(&mut self, action: impl Into<String>, key: Key) {
        self.bindings
            .entry(action.into())
            .or_default()
            .insert(key);
    }

    /// Remove all bindings for an action.
    pub fn remove_action(&mut self, action: &str) {
        self.bindings.remove(action);
    }

    /// Return the set of keys bound to `action`, if any.
    pub fn keys_for(&self, action: &str) -> Option<&HashSet<Key>> {
        self.bindings.get(action)
    }

    /// True if `key` is bound to `action`.
    pub fn action_has_key(&self, action: &str, key: Key) -> bool {
        self.bindings
            .get(action)
            .map_or(false, |keys| keys.contains(&key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_action_and_query() {
        let mut map = InputMap::new();
        map.add_action("jump", Key::Space);
        assert!(map.action_has_key("jump", Key::Space));
        assert!(!map.action_has_key("jump", Key::Enter));
    }

    #[test]
    fn add_action_multiple_keys() {
        let mut map = InputMap::new();
        map.add_action("confirm", Key::Enter);
        map.add_action("confirm", Key::Space);
        let keys = map.keys_for("confirm").unwrap();
        assert!(keys.contains(&Key::Enter));
        assert!(keys.contains(&Key::Space));
    }

    #[test]
    fn remove_action_clears_bindings() {
        let mut map = InputMap::new();
        map.add_action("fire", Key::Space);
        map.remove_action("fire");
        assert!(map.keys_for("fire").is_none());
    }

    #[test]
    fn unknown_action_returns_none() {
        let map = InputMap::new();
        assert!(map.keys_for("nonexistent").is_none());
        assert!(!map.action_has_key("nonexistent", Key::Space));
    }

    #[test]
    fn input_action_name() {
        let a = InputAction::new("move_left");
        assert_eq!(a.name(), "move_left");
    }
}
