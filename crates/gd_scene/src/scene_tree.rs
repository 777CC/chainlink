//! `SceneTree` — owns all nodes and their parent–child relationships.

use std::collections::HashMap;
use slotmap::SlotMap;
use log::debug;

use crate::node::{DrawRect, Node, NodeContext, NodeId, SceneCommand};
use gd_input::InputEvent;

/// The scene tree; the heart of the engine's object model.
///
/// # Ownership model
///
/// All nodes are stored in a `SlotMap<NodeId, Box<dyn Node>>`.  Parent–child
/// edges are stored in two separate maps (`children`, `parent`) so the borrow
/// checker never has to deal with aliased `Box<dyn Node>` references.
pub struct SceneTree {
    nodes:    SlotMap<NodeId, Box<dyn Node>>,
    children: HashMap<NodeId, Vec<NodeId>>,
    parent:   HashMap<NodeId, NodeId>,
    /// The root of the visible scene.
    pub root: Option<NodeId>,
}

impl SceneTree {
    pub fn new() -> Self {
        Self {
            nodes:    SlotMap::with_key(),
            children: HashMap::new(),
            parent:   HashMap::new(),
            root:     None,
        }
    }

    // ── Tree manipulation ─────────────────────────────────────────────────────

    /// Insert a node as a child of `parent_id` (or as the root when `None`).
    pub fn add_child(&mut self, parent_id: Option<NodeId>, node: Box<dyn Node>) -> NodeId {
        let id = self.nodes.insert(node);
        self.children.entry(id).or_default(); // ensure entry exists

        if let Some(pid) = parent_id {
            self.children.entry(pid).or_default().push(id);
            self.parent.insert(id, pid);
        } else {
            self.root = Some(id);
        }

        debug!("SceneTree: added node {:?}", id);
        id
    }

    /// Set the root scene, replacing the previous one entirely.
    pub fn set_root_scene(&mut self, node: Box<dyn Node>) -> NodeId {
        // Remove existing tree if any
        if let Some(old_root) = self.root.take() {
            self.remove_subtree(old_root);
        }
        self.add_child(None, node)
    }

    /// Remove a node and the entire subtree beneath it.
    pub fn remove_subtree(&mut self, id: NodeId) {
        let children: Vec<NodeId> = self.children
            .get(&id)
            .cloned()
            .unwrap_or_default();

        for child in children {
            self.remove_subtree(child);
        }

        // Detach from parent
        if let Some(pid) = self.parent.remove(&id) {
            if let Some(siblings) = self.children.get_mut(&pid) {
                siblings.retain(|&c| c != id);
            }
        }

        self.children.remove(&id);
        self.nodes.remove(id);
        debug!("SceneTree: removed node {:?}", id);
    }

    /// Get a reference to a node.
    pub fn get(&self, id: NodeId) -> Option<&dyn Node> {
        self.nodes.get(id).map(|b| b.as_ref())
    }

    /// Get a mutable reference to a node.
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut dyn Node> {
        self.nodes.get_mut(id).map(|b| b.as_mut())
    }

    /// Children of a node.
    pub fn children_of(&self, id: NodeId) -> &[NodeId] {
        self.children.get(&id).map_or(&[], |v| v.as_slice())
    }

    /// Parent of a node, if any.
    pub fn parent_of(&self, id: NodeId) -> Option<NodeId> {
        self.parent.get(&id).copied()
    }

    /// Collect all node IDs in depth-first pre-order.
    pub fn all_node_ids(&self) -> Vec<NodeId> {
        let mut out = Vec::with_capacity(self.nodes.len());
        if let Some(root) = self.root {
            self.collect_ids(root, &mut out);
        }
        out
    }

    fn collect_ids(&self, id: NodeId, out: &mut Vec<NodeId>) {
        out.push(id);
        for &child in self.children_of(id) {
            self.collect_ids(child, out);
        }
    }

    // ── Frame update ──────────────────────────────────────────────────────────

    /// Call `ready` on every node that hasn't had it called yet.
    /// (In practice you'd track a "is_ready" flag; here we call it on all nodes
    ///  the first time `update` is invoked — simplified for clarity.)
    pub fn ready_all(&mut self) {
        let ids = self.all_node_ids();
        let mut draw_queue = Vec::new();
        let mut commands   = Vec::new();

        for id in ids {
            if let Some(node) = self.nodes.get_mut(id) {
                let mut ctx = NodeContext {
                    this_id: id,
                    draw_queue: &mut draw_queue,
                    commands: &mut commands,
                };
                node.ready(&mut ctx);
            }
        }

        self.apply_commands(commands);
    }

    /// Process all nodes for one frame.  Returns the draw queue.
    pub fn process(
        &mut self,
        delta: f64,
        events: &[InputEvent],
    ) -> Vec<DrawRect> {
        let ids = self.all_node_ids();
        let mut draw_queue = Vec::new();
        let mut commands   = Vec::new();

        for id in &ids {
            let id = *id;
            if let Some(node) = self.nodes.get_mut(id) {
                let mut ctx = NodeContext {
                    this_id: id,
                    draw_queue: &mut draw_queue,
                    commands: &mut commands,
                };
                node.process(delta, &mut ctx);
                for ev in events {
                    node.input(ev, &mut ctx);
                }
            }
        }

        self.apply_commands(commands);
        draw_queue
    }

    // ── Command application ───────────────────────────────────────────────────

    fn apply_commands(&mut self, commands: Vec<SceneCommand>) {
        for cmd in commands {
            match cmd {
                SceneCommand::AddChild { parent, node } => {
                    self.add_child(Some(parent), node);
                }
                SceneCommand::QueueFree(id) => {
                    self.remove_subtree(id);
                }
                SceneCommand::ChangeScene(root_node) => {
                    self.set_root_scene(root_node);
                }
            }
        }
    }
}

impl Default for SceneTree {
    fn default() -> Self { Self::new() }
}
