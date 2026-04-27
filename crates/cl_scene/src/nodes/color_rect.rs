//! `ColorRect` — a filled-rectangle UI element.

use std::any::Any;
use cl_core::{Color, Transform2D, Vec2};
use cl_input::InputEvent;
use crate::node::{Node, NodeContext};

/// A simple filled rectangle, useful for UI panels, health bars, etc.
pub struct ColorRect {
    pub name:    String,
    pub position: Vec2,
    pub size:    Vec2,
    pub color:   Color,
    pub visible: bool,
}

impl ColorRect {
    pub fn new(name: impl Into<String>, position: Vec2, size: Vec2, color: Color) -> Self {
        Self { name: name.into(), position, size, color, visible: true }
    }
}

impl Node for ColorRect {
    fn name(&self) -> &str { &self.name }

    fn process(&mut self, _delta: f64, ctx: &mut NodeContext) {
        if self.visible {
            ctx.draw_rect(self.position, self.size, self.color);
        }
    }

    fn input(&mut self, _event: &InputEvent, _ctx: &mut NodeContext) {}

    fn transform(&self) -> Transform2D {
        Transform2D::new(self.position, 0.0, Vec2::ONE)
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
