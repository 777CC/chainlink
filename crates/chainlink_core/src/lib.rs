//! **chainlink_core** — fundamental types for the Chainlink Game Engine.
//!
//! Re-exports everything a user needs: math primitives, colors,
//! a dynamic `Variant` type, and the signal/slot system.

pub mod color;
pub mod math;
pub mod signal;
pub mod variant;

pub use color::Color;
pub use math::{Rect2, Transform2D, Vec2, Vec3};
pub use signal::Signal;
pub use variant::Variant;
