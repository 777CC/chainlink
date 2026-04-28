//! Catalogue of node types the editor knows how to create.
//!
//! Each entry pairs a kind name with a list of (property_name, default).
//! `make_node` looks up the kind and builds a new [`EditorNode`].

use std::collections::HashMap;

use crate::scene_doc::{EditorNode, EditorValue};

/// Default value used to seed a property when a node is created.
pub enum EditorValueDefault {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(&'static str),
    Vec2(f32, f32),
    Color(f32, f32, f32, f32),
    AssetRef,
}

/// Static description of every node type available in the "Add Node" menu.
pub const NODE_KINDS: &[(&str, &[(&str, EditorValueDefault)])] = &[
    ("Node", &[]),
    (
        "Node2D",
        &[
            ("position", EditorValueDefault::Vec2(0.0, 0.0)),
            ("rotation", EditorValueDefault::Float(0.0)),
        ],
    ),
    (
        "Sprite2D",
        &[
            ("position", EditorValueDefault::Vec2(0.0, 0.0)),
            ("size", EditorValueDefault::Vec2(64.0, 64.0)),
            ("color", EditorValueDefault::Color(1.0, 1.0, 1.0, 1.0)),
            ("texture", EditorValueDefault::AssetRef),
        ],
    ),
    (
        "Camera2D",
        &[
            ("position", EditorValueDefault::Vec2(0.0, 0.0)),
            ("zoom", EditorValueDefault::Vec2(1.0, 1.0)),
        ],
    ),
    (
        "Label",
        &[
            ("position", EditorValueDefault::Vec2(0.0, 0.0)),
            ("text", EditorValueDefault::String("Label")),
            ("color", EditorValueDefault::Color(1.0, 1.0, 1.0, 1.0)),
        ],
    ),
    (
        "RigidBody2D",
        &[
            ("position", EditorValueDefault::Vec2(0.0, 0.0)),
            ("size", EditorValueDefault::Vec2(32.0, 32.0)),
            ("gravity_scale", EditorValueDefault::Float(1.0)),
        ],
    ),
    (
        "StaticBody2D",
        &[
            ("position", EditorValueDefault::Vec2(0.0, 0.0)),
            ("size", EditorValueDefault::Vec2(64.0, 16.0)),
            ("color", EditorValueDefault::Color(0.5, 0.5, 0.5, 1.0)),
        ],
    ),
    (
        "Area2D",
        &[
            ("position", EditorValueDefault::Vec2(0.0, 0.0)),
            ("size", EditorValueDefault::Vec2(64.0, 64.0)),
        ],
    ),
    (
        "ColorRect",
        &[
            ("position", EditorValueDefault::Vec2(0.0, 0.0)),
            ("size", EditorValueDefault::Vec2(100.0, 100.0)),
            ("color", EditorValueDefault::Color(1.0, 1.0, 1.0, 1.0)),
        ],
    ),
    (
        "ProgressBar",
        &[
            ("position", EditorValueDefault::Vec2(0.0, 0.0)),
            ("size", EditorValueDefault::Vec2(200.0, 20.0)),
            ("value", EditorValueDefault::Float(0.5)),
        ],
    ),
    (
        "AudioStreamPlayer",
        &[
            ("autoplay", EditorValueDefault::Bool(false)),
            ("volume_db", EditorValueDefault::Float(0.0)),
            ("stream", EditorValueDefault::AssetRef),
        ],
    ),
    ("Node3D", &[]),
    (
        "MeshInstance3D",
        &[("mesh", EditorValueDefault::AssetRef)],
    ),
];

/// Convert a [`EditorValueDefault`] into a real [`EditorValue`].
fn default_value(d: &EditorValueDefault) -> EditorValue {
    match d {
        EditorValueDefault::Bool(v) => EditorValue::Bool(*v),
        EditorValueDefault::Int(v) => EditorValue::Int(*v),
        EditorValueDefault::Float(v) => EditorValue::Float(*v),
        EditorValueDefault::String(s) => EditorValue::String((*s).to_string()),
        EditorValueDefault::Vec2(x, y) => EditorValue::Vec2 { x: *x, y: *y },
        EditorValueDefault::Color(r, g, b, a) => EditorValue::Color {
            r: *r,
            g: *g,
            b: *b,
            a: *a,
        },
        EditorValueDefault::AssetRef => EditorValue::AssetRef(String::new()),
    }
}

/// Build a new [`EditorNode`] of the requested kind, populating its
/// default property set. If `kind` is unknown, returns a bare `Node`.
pub fn make_node(kind: &str, id: u64) -> EditorNode {
    let entry = NODE_KINDS.iter().find(|(k, _)| *k == kind);
    let mut properties = HashMap::new();
    let kind_str = if let Some((k, defaults)) = entry {
        for (name, def) in *defaults {
            properties.insert((*name).to_string(), default_value(def));
        }
        (*k).to_string()
    } else {
        "Node".to_string()
    };

    EditorNode {
        id,
        name: format!("{}{}", kind_str, id),
        kind: kind_str,
        properties,
        children: Vec::new(),
    }
}
