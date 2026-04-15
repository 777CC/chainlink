//! **gd_render** — GPU rendering back-end for the Godot-Rust engine.
//!
//! Architecture
//! ────────────
//! The renderer is intentionally kept thin:
//!
//! 1. The scene tree produces a `Vec<DrawRect>` each frame (draw calls).
//! 2. `Renderer::submit` sorts them by z-index and uploads them to the GPU
//!    as instanced quads drawn with a single wgpu render pass.
//!
//! The geometry is a unit quad (`[0,0]–[1,1]`).  Each instance carries a
//! model matrix (built from position + size) and a color.  A camera uniform
//! provides the orthographic view-projection matrix.

pub mod pipeline;
pub mod renderer;
pub mod vertex;

pub use renderer::Renderer;
