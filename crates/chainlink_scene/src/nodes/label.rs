//! `Label` — a UI text label (placeholder; real text rendering requires a
//! font atlas, which would be a crate of its own).

use std::any::Any;
use chainlink_core::{Color, Transform2D, Vec2};
use chainlink_input::InputEvent;
use crate::node::{Node, NodeContext};

/// A text label node.  Currently emits no draw calls (font rendering is out of
/// scope for the minimal renderer), but is retained in the scene tree and
/// participates in all lifecycle callbacks.
pub struct Label {
    pub name:     String,
    pub text:     String,
    pub position: Vec2,
    pub color:    Color,
    pub visible:  bool,
}

impl Label {
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            name:    name.into(),
            text:    text.into(),
            position: Vec2::ZERO,
            color:    Color::WHITE,
            visible:  true,
        }
    }
}

impl Node for Label {
    fn name(&self) -> &str { &self.name }
    fn transform(&self) -> Transform2D {
        Transform2D::new(self.position, 0.0, Vec2::ONE)
    }
    fn ready(&mut self, _ctx: &mut NodeContext) {}
    fn process(&mut self, _delta: f64, _ctx: &mut NodeContext) {}
    fn input(&mut self, _event: &InputEvent, _ctx: &mut NodeContext) {}
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
