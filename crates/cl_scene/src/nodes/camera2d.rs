//! `Camera2D` — 2-D orthographic camera.

use std::any::Any;
use cl_core::{Transform2D, Vec2};
use cl_core::math::Mat4;
use cl_input::InputEvent;
use crate::node::{Node, NodeContext};

/// An orthographic 2-D camera.
///
/// The renderer queries the active camera each frame to build the
/// view-projection matrix.  The viewport size is read from the
/// [`DisplayServer`] if available.
pub struct Camera2D {
    pub name:     String,
    pub position: Vec2,
    pub zoom:     Vec2,
    pub enabled:  bool,
}

impl Camera2D {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name:    name.into(),
            position: Vec2::ZERO,
            zoom:     Vec2::ONE,
            enabled:  true,
        }
    }

    /// Orthographic view-projection matrix for a given viewport size (pixels).
    pub fn view_proj(&self, viewport_size: Vec2) -> Mat4 {
        let w = viewport_size.x / self.zoom.x;
        let h = viewport_size.y / self.zoom.y;
        let hw = w * 0.5;
        let hh = h * 0.5;

        // Translate so `position` is at the center.
        let cx = self.position.x;
        let cy = self.position.y;

        Mat4::orthographic_rh(cx - hw, cx + hw, cy + hh, cy - hh, -1000.0, 1000.0)
    }

    /// Orthographic matrix using the current display server viewport size.
    pub fn view_proj_display(&self, ctx: &NodeContext) -> Mat4 {
        self.view_proj(ctx.display.size_vec2())
    }
}

impl Node for Camera2D {
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
