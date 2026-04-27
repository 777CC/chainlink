//! **cl_core** — fundamental types for the Chainlink Game Engine.
//!
//! Re-exports everything a user needs: math primitives, colors,
//! a dynamic `Variant` type, and the signal/slot system.

pub mod color;
pub mod math;
pub mod node_path;
pub mod resource;
pub mod rid;
pub mod signal;
pub mod string_name;
pub mod variant;

pub use color::Color;
pub use math::{Rect2, Transform2D, Vec2, Vec3};
pub use node_path::NodePath;
pub use resource::Resource;
pub use rid::Rid;
pub use signal::Signal;
pub use string_name::StringName;
pub use variant::Variant;
