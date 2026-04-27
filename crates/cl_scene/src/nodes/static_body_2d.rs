//! `StaticBody2D` — an immovable physics body (ground, platforms, walls).

use std::any::Any;
use cl_core::{Color, Rid, Transform2D, Vec2};
use cl_input::InputEvent;
use crate::node::{Node, NodeContext};

/// A fixed (kinematic/static) rigid body.
///
/// Doesn't move under physics forces; other bodies collide against it.
pub struct StaticBody2D {
    pub name:     String,
    pub position: Vec2,
    pub size:     Vec2,
    pub color:    Color,
    pub body_rid: Option<Rid>,
}

impl StaticBody2D {
    pub fn new(name: impl Into<String>, position: Vec2, size: Vec2, color: Color) -> Self {
        Self { name: name.into(), position, size, color, body_rid: None }
    }
}

impl Node for StaticBody2D {
    fn name(&self) -> &str { &self.name }

    fn ready(&mut self, ctx: &mut NodeContext) {
        let rid = ctx.physics_2d.body_create_static(self.position);
        ctx.physics_2d.body_add_collider_rect(rid, self.size * 0.5, 0.3, 0.5);
        self.body_rid = Some(rid);
    }

    fn process(&mut self, _delta: f64, ctx: &mut NodeContext) {
        ctx.draw_rect(self.position - self.size * 0.5, self.size, self.color);
    }

    fn input(&mut self, _event: &InputEvent, _ctx: &mut NodeContext) {}

    fn transform(&self) -> Transform2D {
        Transform2D::new(self.position, 0.0, Vec2::ONE)
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
