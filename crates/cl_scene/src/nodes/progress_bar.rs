//! `ProgressBar` — a horizontal progress bar UI element.

use std::any::Any;
use cl_core::{Color, Transform2D, Vec2};
use cl_input::InputEvent;
use crate::node::{Node, NodeContext};

/// A horizontal bar that fills from left to right based on `value` (0.0–1.0).
pub struct ProgressBar {
    pub name:       String,
    pub position:   Vec2,
    pub size:       Vec2,
    /// Normalised fill amount in `[0.0, 1.0]`.
    pub value:      f32,
    pub bg_color:   Color,
    pub fill_color: Color,
    pub visible:    bool,
}

impl ProgressBar {
    pub fn new(name: impl Into<String>, position: Vec2, size: Vec2) -> Self {
        Self {
            name:       name.into(),
            position,
            size,
            value:      1.0,
            bg_color:   Color::rgba(0.2, 0.2, 0.2, 1.0),
            fill_color: Color::rgba(0.2, 0.8, 0.2, 1.0),
            visible:    true,
        }
    }
}

impl Node for ProgressBar {
    fn name(&self) -> &str { &self.name }

    fn process(&mut self, _delta: f64, ctx: &mut NodeContext) {
        if !self.visible { return; }

        // Background
        ctx.draw_rect(self.position, self.size, self.bg_color);

        // Fill (clamped to [0,1])
        let fill = self.value.clamp(0.0, 1.0);
        if fill > 0.0 {
            ctx.draw_rect(
                self.position,
                Vec2::new(self.size.x * fill, self.size.y),
                self.fill_color,
            );
        }
    }

    fn input(&mut self, _event: &InputEvent, _ctx: &mut NodeContext) {}

    fn transform(&self) -> Transform2D {
        Transform2D::new(self.position, 0.0, Vec2::ONE)
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
