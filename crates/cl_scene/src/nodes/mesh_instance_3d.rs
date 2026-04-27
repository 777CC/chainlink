//! `MeshInstance3D` — a 3-D node that references a mesh resource by RID.

use std::any::Any;
use cl_core::{Rid, Transform2D, Vec2};
use cl_input::InputEvent;
use crate::node::{Node, NodeContext};
use super::node3d::Node3D;

/// Holds a reference to a mesh RID (issued by the RenderingServer) and a
/// [`Node3D`] for spatial placement.  Currently a stub — rendering 3-D meshes
/// is left to future wgpu backend work.
pub struct MeshInstance3D {
    pub name:    String,
    pub node3d:  Node3D,
    /// RID of the mesh resource, or [`Rid::INVALID`] if none assigned.
    pub mesh_rid: Rid,
}

impl MeshInstance3D {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name:     name.into(),
            node3d:   Node3D::new(""),
            mesh_rid: Rid::INVALID,
        }
    }
}

impl Node for MeshInstance3D {
    fn name(&self) -> &str { &self.name }
    fn transform(&self) -> Transform2D {
        let p = self.node3d.position;
        Transform2D::new(Vec2::new(p.x, p.y), 0.0, Vec2::ONE)
    }
    fn ready(&mut self, _ctx: &mut NodeContext) {}
    fn process(&mut self, _delta: f64, _ctx: &mut NodeContext) {}
    fn input(&mut self, _event: &InputEvent, _ctx: &mut NodeContext) {}
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
