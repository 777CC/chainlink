//! `RigidBody2D` — a dynamic physics body driven by the PhysicsServer2D.

use std::any::Any;
use cl_core::{Color, Rid, Transform2D, Vec2};
use cl_input::InputEvent;
use crate::node::{Node, NodeContext};

/// A dynamic rigid body.  Its position is read back from the physics server
/// every frame and used to emit a draw call.
pub struct RigidBody2D {
    pub name:          String,
    /// Half-extents of the box collider (full size is 2× this in each axis).
    pub size:          Vec2,
    pub color:         Color,
    pub gravity_scale: f32,
    /// RID assigned by the physics server in `ready()`.
    pub body_rid:      Option<Rid>,
}

impl RigidBody2D {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name:          name.into(),
            size:          Vec2::splat(32.0),
            color:         Color::WHITE,
            gravity_scale: 1.0,
            body_rid:      None,
        }
    }

    pub fn with_size(mut self, size: Vec2) -> Self { self.size = size; self }
    pub fn with_color(mut self, color: Color) -> Self { self.color = color; self }

    /// Current world-space position (read from physics server).
    pub fn position(&self, ctx: &NodeContext) -> Vec2 {
        self.body_rid.map(|r| ctx.physics_2d.body_get_position(r)).unwrap_or(Vec2::ZERO)
    }
}

impl Node for RigidBody2D {
    fn name(&self) -> &str { &self.name }

    fn ready(&mut self, ctx: &mut NodeContext) {
        // Create the body at origin with no initial velocity
        let rid = ctx.physics_2d.body_create_dynamic(Vec2::ZERO, Vec2::ZERO);
        // Add a box collider using half-extents (size/2)
        ctx.physics_2d.body_add_collider_rect(rid, self.size * 0.5, 0.3, 0.5);
        self.body_rid = Some(rid);
    }

    fn process(&mut self, _delta: f64, ctx: &mut NodeContext) {
        if let Some(rid) = self.body_rid {
            let pos = ctx.physics_2d.body_get_position(rid);
            ctx.draw_rect(pos - self.size * 0.5, self.size, self.color);
        }
    }

    fn input(&mut self, _event: &InputEvent, _ctx: &mut NodeContext) {}

    fn transform(&self) -> Transform2D { Transform2D::IDENTITY }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
