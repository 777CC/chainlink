//! Standalone engine runner — launches a blank window with a dark background.
//!
//! Run with:
//!   cargo run -p chainlink_engine

use chainlink_engine::Engine;
use chainlink_scene::node::BaseNode;

fn main() {
    let engine = Engine::builder()
        .title("Chainlink Game Engine")
        .size(1280, 720)
        .build();

    engine.run(|tree, _input| {
        tree.set_root_scene(BaseNode::new("Root"));
    });
}
