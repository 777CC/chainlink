//! `Area2D` — a trigger zone that detects overlaps (no physics response).

use std::any::Any;
use cl_core::{Color, Rect2, Transform2D, Vec2};
use cl_input::InputEvent;
use crate::node::{Node, NodeContext};

/// An axis-aligned trigger area.
///
/// Uses simple AABB overlap checks via [`Rect2::intersects`].  Does **not**
/// use the physics server — Area2D intentionally produces no physical forces.
pub struct Area2D {
    pub name:    String,
    pub position: Vec2,
    pub size:    Vec2,
    pub color:   Color,
    pub visible: bool,
}

impl Area2D {
    pub fn new(name: impl Into<String>, position: Vec2, size: Vec2) -> Self {
        Self {
            name:    name.into(),
            position,
            size,
            color:   Color::rgba(0.2, 0.8, 0.2, 0.4),
            visible: false,  // invisible by default (trigger only)
        }
    }

    /// Return `true` if this area overlaps the given rectangle.
    pub fn overlaps_rect(&self, other_pos: Vec2, other_size: Vec2) -> bool {
        let a = Rect2 { position: self.position - self.size * 0.5, size: self.size };
        let b = Rect2 { position: other_pos - other_size * 0.5,    size: other_size };
        a.intersects(b)
    }
}

impl Node for Area2D {
    fn name(&self) -> &str { &self.name }

    fn process(&mut self, _delta: f64, ctx: &mut NodeContext) {
        if self.visible {
            ctx.draw_rect(self.position - self.size * 0.5, self.size, self.color);
        }
    }

    fn input(&mut self, _event: &InputEvent, _ctx: &mut NodeContext) {}

    fn transform(&self) -> Transform2D {
        Transform2D::new(self.position, 0.0, Vec2::ONE)
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
