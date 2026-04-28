//! Editor-side editable scene representation.
//!
//! Mirrors the role of Godot's `.tscn` document: it is the in-memory model
//! the inspector and scene-tree panels mutate. Saving serialises this struct
//! to JSON.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A whole scene as edited in the editor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorScene {
    pub name: String,
    pub root: EditorNode,
    #[serde(skip)]
    pub dirty: bool,
    #[serde(skip)]
    pub selected_id: Option<u64>,
}

/// A node in the editable scene tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorNode {
    pub id: u64,
    pub name: String,
    pub kind: String,
    pub properties: HashMap<String, EditorValue>,
    pub children: Vec<EditorNode>,
}

/// Property value variants understood by the inspector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditorValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Vec2 { x: f32, y: f32 },
    Color { r: f32, g: f32, b: f32, a: f32 },
    AssetRef(String),
}

impl EditorScene {
    /// Construct a new scene with a single `Node` root named "Root".
    pub fn new(name: impl Into<String>) -> Self {
        let root = EditorNode {
            id: 1,
            name: "Root".to_string(),
            kind: "Node".to_string(),
            properties: HashMap::new(),
            children: Vec::new(),
        };
        Self {
            name: name.into(),
            root,
            dirty: false,
            selected_id: Some(1),
        }
    }

    /// Find the smallest unused id (= max id in the tree + 1).
    pub fn next_id(&self) -> u64 {
        fn walk(node: &EditorNode, max: &mut u64) {
            if node.id > *max {
                *max = node.id;
            }
            for c in &node.children {
                walk(c, max);
            }
        }
        let mut max = 0;
        walk(&self.root, &mut max);
        max + 1
    }

    /// Recursively look up a node by id (immutable).
    pub fn find(&self, id: u64) -> Option<&EditorNode> {
        fn walk<'a>(node: &'a EditorNode, id: u64) -> Option<&'a EditorNode> {
            if node.id == id {
                return Some(node);
            }
            for c in &node.children {
                if let Some(found) = walk(c, id) {
                    return Some(found);
                }
            }
            None
        }
        walk(&self.root, id)
    }

    /// Recursively look up a node by id (mutable).
    pub fn find_mut(&mut self, id: u64) -> Option<&mut EditorNode> {
        fn walk<'a>(node: &'a mut EditorNode, id: u64) -> Option<&'a mut EditorNode> {
            if node.id == id {
                return Some(node);
            }
            for c in &mut node.children {
                if let Some(found) = walk(c, id) {
                    return Some(found);
                }
            }
            None
        }
        walk(&mut self.root, id)
    }

    /// Remove a node by id. The root cannot be removed.
    pub fn remove(&mut self, id: u64) -> bool {
        if id == self.root.id {
            return false;
        }
        fn walk(node: &mut EditorNode, id: u64) -> bool {
            if let Some(idx) = node.children.iter().position(|c| c.id == id) {
                node.children.remove(idx);
                return true;
            }
            for c in &mut node.children {
                if walk(c, id) {
                    return true;
                }
            }
            false
        }
        let removed = walk(&mut self.root, id);
        if removed {
            self.dirty = true;
            if self.selected_id == Some(id) {
                self.selected_id = Some(self.root.id);
            }
        }
        removed
    }

    /// Append `child` underneath the node with `parent_id`.
    pub fn add_child(&mut self, parent_id: u64, child: EditorNode) -> bool {
        if let Some(parent) = self.find_mut(parent_id) {
            parent.children.push(child);
            self.dirty = true;
            true
        } else {
            false
        }
    }
}
