//! **cl_engine** — the engine host: window, event loop, and game loop.
//!
//! # Usage
//!
//! ```rust,no_run
//! use cl_engine::Engine;
//! use cl_scene::{SceneTree, node::BaseNode};
//! use cl_input::InputServer;
//!
//! fn main() {
//!     Engine::builder()
//!         .title("My Game")
//!         .size(1280, 720)
//!         .build()
//!         .run(|tree, _input| {
//!             tree.set_root_scene(BaseNode::new("Root"));
//!         });
//! }
//! ```

pub mod engine;
pub mod input_bridge;
pub mod time;

pub use engine::{Engine, EngineBuilder};
pub use time::GameTime;
