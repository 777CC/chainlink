//! `Node3D` — a node with a 3-D transform; base for all 3-D objects.

use std::any::Any;
use cl_core::math::{Mat4, Quat, Vec3};
use cl_core::{Transform2D, Vec2};
use cl_input::InputEvent;
use crate::node::{Node, NodeContext};

/// A node that lives in 3-D space.  Equivalent to Godot's `Node3D`.
///
/// Provides `position`, `rotation` (quaternion), and `scale` in world space.
pub struct Node3D {
    pub name:     String,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale:    Vec3,
    pub visible:  bool,
}

impl Node3D {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name:     name.into(),
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale:    Vec3::ONE,
            visible:  true,
        }
    }

    /// Build the world-space model matrix from position + rotation + scale.
    pub fn global_transform(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

impl Node for Node3D {
    fn name(&self) -> &str { &self.name }

    /// Returns the 2-D projection of the 3-D position (XY plane).
    fn transform(&self) -> Transform2D {
        Transform2D::new(Vec2::new(self.position.x, self.position.y), 0.0, Vec2::ONE)
    }

    fn ready(&mut self, _ctx: &mut NodeContext) {}
    fn process(&mut self, _delta: f64, _ctx: &mut NodeContext) {}
    fn input(&mut self, _event: &InputEvent, _ctx: &mut NodeContext) {}
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
