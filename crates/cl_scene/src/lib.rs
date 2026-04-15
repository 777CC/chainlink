//! **cl_scene** — scene graph and node system for the Chainlink Game Engine.
//!
//! Architecture overview
//! ─────────────────────
//! * Every object in the scene implements the [`Node`] trait.
//! * Nodes are owned by the [`SceneTree`] and addressed by stable [`NodeId`]s
//!   (backed by `slotmap`).
//! * Parent–child relationships are stored separately in [`SceneTree`], not
//!   inside the node, to avoid borrow-checker complications.
//! * During `process`, nodes enqueue [`SceneCommand`]s that are applied
//!   atomically after the full process pass — the same pattern Chainlink uses
//!   for `queue_free` / `add_child` deferred calls.
//! * Built-in node types (`Node2D`, `Sprite2D`, `Camera2D`, `Label`) live in
//!   the [`nodes`] module and serve as concrete starting points.

pub mod node;
pub mod nodes;
pub mod scene_tree;

pub use node::{Node, NodeContext, NodeId, SceneCommand};
pub use scene_tree::SceneTree;
