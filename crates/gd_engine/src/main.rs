//! Standalone engine runner — launches a blank window with a dark background.
//!
//! Run with:
//!   cargo run -p gd_engine

use gd_engine::Engine;
use gd_scene::node::BaseNode;

fn main() {
    let engine = Engine::builder()
        .title("Godot-Rust Engine")
        .size(1280, 720)
        .build();

    engine.run(|tree, _input| {
        tree.set_root_scene(BaseNode::new("Root"));
    });
}
