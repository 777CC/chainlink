//! Editor UI panels.
//!
//! Each sub-module renders one egui panel of the editor. The
//! [`crate::app::EditorApp`] composes them in its `update` method.

pub mod asset_browser;
pub mod file_panel;
pub mod inspector_panel;
pub mod menu_bar;
pub mod scene_tree_panel;
pub mod viewport_panel;
