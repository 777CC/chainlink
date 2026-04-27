//! **cl_servers** — server singletons that mirror Godot's server architecture.
//!
//! Servers own all "heavy" resources (GPU objects, physics bodies, audio sinks)
//! and expose a RID-based API that nodes use at arm's length — the same pattern
//! Godot uses to decouple scene logic from backend implementations.

pub mod audio_server;
pub mod physics_server_2d;
pub mod rendering_server;

pub use audio_server::AudioServer;
pub use physics_server_2d::PhysicsServer2D;
pub use rendering_server::RenderingServer;
