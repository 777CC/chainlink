//! Chainlink Editor library — IDE for the Chainlink Game Engine.
//!
//! This crate exposes the editor's modules so that the binary in `main.rs`
//! and any UI sub-modules can refer to them via the `cl_editor::` path.

pub mod app;
pub mod importer;
pub mod node_kinds;
pub mod project;
pub mod scene_doc;
pub mod ui;
