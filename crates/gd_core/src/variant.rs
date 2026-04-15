//! `Variant` — a dynamically-typed value, similar to Godot's `Variant`.
//!
//! Used for property bags, inter-node messaging, and scripting bridges.

use crate::{Color, Vec2, Vec3};

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Vec2(Vec2),
    Vec3(Vec3),
    Color(Color),
    Array(Vec<Variant>),
}

impl Variant {
    pub fn is_nil(&self) -> bool { matches!(self, Self::Nil) }

    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(v) = self { Some(*v) } else { None }
    }
    pub fn as_int(&self) -> Option<i64> {
        if let Self::Int(v) = self { Some(*v) } else { None }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(v) => Some(*v),
            Self::Int(v)   => Some(*v as f64),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        if let Self::String(s) = self { Some(s.as_str()) } else { None }
    }
}

impl Default for Variant {
    fn default() -> Self { Self::Nil }
}

// ── From impls ───────────────────────────────────────────────────────────────

impl From<bool>   for Variant { fn from(v: bool)   -> Self { Self::Bool(v) } }
impl From<i32>    for Variant { fn from(v: i32)    -> Self { Self::Int(v as i64) } }
impl From<i64>    for Variant { fn from(v: i64)    -> Self { Self::Int(v) } }
impl From<f32>    for Variant { fn from(v: f32)    -> Self { Self::Float(v as f64) } }
impl From<f64>    for Variant { fn from(v: f64)    -> Self { Self::Float(v) } }
impl From<String> for Variant { fn from(v: String) -> Self { Self::String(v) } }
impl From<&str>   for Variant { fn from(v: &str)   -> Self { Self::String(v.to_owned()) } }
impl From<Vec2>   for Variant { fn from(v: Vec2)   -> Self { Self::Vec2(v) } }
impl From<Vec3>   for Variant { fn from(v: Vec3)   -> Self { Self::Vec3(v) } }
impl From<Color>  for Variant { fn from(v: Color)  -> Self { Self::Color(v) } }
