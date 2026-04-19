//! `Variant` — a dynamically-typed value, similar to Chainlink's `Variant`.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_nil_is_nil() {
        assert!(Variant::Nil.is_nil());
        assert!(!Variant::Bool(true).is_nil());
    }

    #[test]
    fn variant_as_bool() {
        assert_eq!(Variant::Bool(true).as_bool(), Some(true));
        assert_eq!(Variant::Bool(false).as_bool(), Some(false));
        assert_eq!(Variant::Int(1).as_bool(), None);
    }

    #[test]
    fn variant_as_int() {
        assert_eq!(Variant::Int(7).as_int(), Some(7));
        assert_eq!(Variant::Float(1.0).as_int(), None);
    }

    #[test]
    fn variant_as_float_coerces_int() {
        assert_eq!(Variant::Float(3.14).as_float(), Some(3.14));
        assert_eq!(Variant::Int(5).as_float(), Some(5.0));
        assert_eq!(Variant::Bool(true).as_float(), None);
    }

    #[test]
    fn variant_as_str() {
        assert_eq!(Variant::String("hi".into()).as_str(), Some("hi"));
        assert_eq!(Variant::Int(1).as_str(), None);
    }

    #[test]
    fn variant_from_primitives() {
        assert!(matches!(Variant::from(true),         Variant::Bool(true)));
        assert!(matches!(Variant::from(42_i32),       Variant::Int(42)));
        assert!(matches!(Variant::from(42_i64),       Variant::Int(42)));
        assert!(matches!(Variant::from(1.5_f32),      Variant::Float(_)));
        assert!(matches!(Variant::from(1.5_f64),      Variant::Float(_)));
        assert!(matches!(Variant::from("hello"),      Variant::String(_)));
        assert!(matches!(Variant::from("x".to_string()), Variant::String(_)));
    }

    #[test]
    fn variant_default_is_nil() {
        assert!(Variant::default().is_nil());
    }
}
