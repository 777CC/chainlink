//! `Sprite2D` — a colored/textured rectangle in 2-D space.
//!
//! In a full engine this would hold a `Texture2D` handle.  For now it
//! renders a solid-color rectangle so the renderer stays dependency-free.

use std::any::Any;
use cl_core::{Color, Transform2D, Variant, Vec2};
use cl_input::InputEvent;
use crate::node::{DrawRect, Node, NodeContext};

/// A 2-D sprite; the most common visible node type.
pub struct Sprite2D {
    pub name:     String,
    pub position: Vec2,
    pub size:     Vec2,
    pub color:    Color,
    pub visible:  bool,
    pub z_index:  i32,
}

impl Sprite2D {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name:    name.into(),
            position: Vec2::ZERO,
            size:     Vec2::new(64.0, 64.0),
            color:    Color::WHITE,
            visible:  true,
            z_index:  0,
        }
    }

    pub fn with_position(mut self, p: Vec2) -> Self { self.position = p; self }
    pub fn with_size(mut self, s: Vec2)     -> Self { self.size = s; self }
    pub fn with_color(mut self, c: Color)   -> Self { self.color = c; self }
}

impl Node for Sprite2D {
    fn name(&self) -> &str { &self.name }

    fn transform(&self) -> Transform2D {
        Transform2D::new(self.position, 0.0, Vec2::ONE)
    }

    fn process(&mut self, _delta: f64, ctx: &mut NodeContext) {
        if self.visible {
            ctx.draw_queue.push(DrawRect {
                position: self.position,
                size:     self.size,
                color:    self.color,
                z_index:  self.z_index,
            });
        }
    }

    fn input(&mut self, _event: &InputEvent, _ctx: &mut NodeContext) {}

    fn get(&self, property: &str) -> Option<Variant> {
        match property {
            "position" => Some(self.position.into()),
            "size"     => Some(self.size.into()),
            "color"    => Some(self.color.into()),
            "visible"  => Some(self.visible.into()),
            _ => None,
        }
    }

    fn set(&mut self, property: &str, value: Variant) -> bool {
        match property {
            "position" => { if let Variant::Vec2(v)   = value { self.position = v; true } else { false } }
            "size"     => { if let Variant::Vec2(v)   = value { self.size     = v; true } else { false } }
            "color"    => { if let Variant::Color(c)  = value { self.color    = c; true } else { false } }
            "visible"  => { if let Some(b) = value.as_bool()  { self.visible  = b; true } else { false } }
            _ => false,
        }
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
