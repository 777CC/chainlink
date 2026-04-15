//! `Node2D` — a node with a 2-D transform; base for all 2-D objects.

use std::any::Any;
use gd_core::{Transform2D, Variant, Vec2};
use gd_input::InputEvent;
use crate::node::{Node, NodeContext};

/// A node that lives in 2-D space.  Equivalent to Godot's `Node2D`.
///
/// Provides `position`, `rotation`, and `scale` properties.  Subclasses
/// (like `Sprite2D`) embed this to inherit the spatial API.
#[derive(Debug, Clone)]
pub struct Node2D {
    pub name: String,
    pub transform: Transform2D,
    pub visible: bool,
}

impl Node2D {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            transform: Transform2D::IDENTITY,
            visible: true,
        }
    }

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.transform.position = pos;
        self
    }

    // ── Convenience accessors ─────────────────────────────────────────────────

    pub fn position(&self) -> Vec2 { self.transform.position }
    pub fn set_position(&mut self, p: Vec2) { self.transform.position = p; }

    pub fn rotation(&self) -> f32 { self.transform.rotation }
    pub fn set_rotation(&mut self, r: f32) { self.transform.rotation = r; }

    pub fn scale(&self) -> Vec2 { self.transform.scale }
    pub fn set_scale(&mut self, s: Vec2) { self.transform.scale = s; }
}

impl Node for Node2D {
    fn name(&self) -> &str { &self.name }
    fn transform(&self) -> Transform2D { self.transform }

    fn get(&self, property: &str) -> Option<Variant> {
        match property {
            "position" => Some(self.transform.position.into()),
            "rotation" => Some(self.transform.rotation.into()),
            "visible"  => Some(self.visible.into()),
            _ => None,
        }
    }

    fn set(&mut self, property: &str, value: Variant) -> bool {
        match property {
            "position" => {
                if let Variant::Vec2(v) = value { self.transform.position = v; true } else { false }
            }
            "rotation" => {
                if let Some(f) = value.as_float() { self.transform.rotation = f as f32; true } else { false }
            }
            "visible" => {
                if let Some(b) = value.as_bool() { self.visible = b; true } else { false }
            }
            _ => false,
        }
    }

    fn ready(&mut self, _ctx: &mut NodeContext) {}
    fn process(&mut self, _delta: f64, _ctx: &mut NodeContext) {}
    fn input(&mut self, _event: &InputEvent, _ctx: &mut NodeContext) {}

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
