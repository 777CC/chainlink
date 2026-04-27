//! `SceneTree` — owns all nodes and their parent–child relationships.

use std::collections::HashMap;
use slotmap::SlotMap;
use log::debug;

use cl_input::{InputEvent, InputServer};
use cl_servers::{AudioServer, PhysicsServer2D, RenderingServer};
use cl_platform::DisplayServer;

use crate::node::{DrawRect, Node, NodeContext, NodeId, SceneCommand};

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

    /// Call `ready` on every node in the tree.
    pub fn ready_all(
        &mut self,
        rendering:  &mut RenderingServer,
        physics_2d: &mut PhysicsServer2D,
        audio:      &mut AudioServer,
        display:    &DisplayServer,
        input:      &InputServer,
    ) {
        let ids = self.all_node_ids();
        let mut draw_queue = Vec::new();
        let mut commands   = Vec::new();

        for id in ids {
            if let Some(node) = self.nodes.get_mut(id) {
                let mut ctx = NodeContext {
                    this_id:    id,
                    draw_queue: &mut draw_queue,
                    commands:   &mut commands,
                    rendering,
                    physics_2d,
                    audio,
                    display,
                    input,
                    delta: 0.0,
                };
                node.ready(&mut ctx);
            }
        }

        self.apply_commands(commands);
    }

    /// Process all nodes for one frame.  Returns the draw queue.
    pub fn process(
        &mut self,
        delta:      f64,
        events:     &[InputEvent],
        rendering:  &mut RenderingServer,
        physics_2d: &mut PhysicsServer2D,
        audio:      &mut AudioServer,
        display:    &DisplayServer,
        input:      &InputServer,
    ) -> Vec<DrawRect> {
        let ids = self.all_node_ids();
        let mut draw_queue = Vec::new();
        let mut commands   = Vec::new();

        for id in &ids {
            let id = *id;
            if let Some(node) = self.nodes.get_mut(id) {
                let mut ctx = NodeContext {
                    this_id:    id,
                    draw_queue: &mut draw_queue,
                    commands:   &mut commands,
                    rendering,
                    physics_2d,
                    audio,
                    display,
                    input,
                    delta,
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

    pub(crate) fn apply_commands(&mut self, commands: Vec<SceneCommand>) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::{BaseNode, Node, NodeContext, SceneCommand};
    use cl_core::{Color, Vec2};
    use std::any::Any;

    // ── Helpers ──────────────────────────────────────────────────────────────

    fn make_servers() -> (RenderingServer, PhysicsServer2D, AudioServer, DisplayServer, InputServer) {
        (
            RenderingServer::new(),
            PhysicsServer2D::new(),
            AudioServer::new(),
            DisplayServer::new(1280, 720, "Test"),
            InputServer::new(),
        )
    }

    fn tree_with_root(name: &str) -> (SceneTree, NodeId) {
        let mut tree = SceneTree::new();
        let id = tree.set_root_scene(BaseNode::new(name));
        (tree, id)
    }

    // ── add_child / structure ─────────────────────────────────────────────────

    #[test]
    fn add_root_sets_root_id() {
        let (tree, root) = tree_with_root("Root");
        assert_eq!(tree.root, Some(root));
    }

    #[test]
    fn add_child_records_parent_child_edges() {
        let (mut tree, root) = tree_with_root("Root");
        let child = tree.add_child(Some(root), BaseNode::new("Child"));
        assert!(tree.children_of(root).contains(&child));
        assert_eq!(tree.parent_of(child), Some(root));
    }

    #[test]
    fn get_returns_node_by_id() {
        let (tree, root) = tree_with_root("Root");
        assert_eq!(tree.get(root).unwrap().name(), "Root");
    }

    #[test]
    fn all_node_ids_depth_first() {
        let (mut tree, root) = tree_with_root("Root");
        let a = tree.add_child(Some(root), BaseNode::new("A"));
        let b = tree.add_child(Some(root), BaseNode::new("B"));
        let a1 = tree.add_child(Some(a), BaseNode::new("A1"));

        let ids = tree.all_node_ids();
        assert_eq!(ids[0], root);
        // A must come before A1
        let pos_a  = ids.iter().position(|&x| x == a).unwrap();
        let pos_a1 = ids.iter().position(|&x| x == a1).unwrap();
        assert!(pos_a < pos_a1);
        // B must be present
        assert!(ids.contains(&b));
    }

    // ── remove_subtree ────────────────────────────────────────────────────────

    #[test]
    fn remove_subtree_drops_node_and_children() {
        let (mut tree, root) = tree_with_root("Root");
        let child = tree.add_child(Some(root), BaseNode::new("Child"));
        let grand = tree.add_child(Some(child), BaseNode::new("Grand"));

        tree.remove_subtree(child);

        assert!(tree.get(child).is_none());
        assert!(tree.get(grand).is_none());
        assert!(tree.children_of(root).is_empty());
    }

    // ── set_root_scene ────────────────────────────────────────────────────────

    #[test]
    fn set_root_scene_replaces_previous_tree() {
        let (mut tree, old_root) = tree_with_root("Old");
        let new_root = tree.set_root_scene(BaseNode::new("New"));
        assert!(tree.get(old_root).is_none());
        assert_eq!(tree.root, Some(new_root));
    }

    // ── process / draw queue ──────────────────────────────────────────────────

    struct DrawingNode { name: String }

    impl Node for DrawingNode {
        fn name(&self) -> &str { &self.name }
        fn process(&mut self, _delta: f64, ctx: &mut NodeContext) {
            ctx.draw_rect(Vec2::new(1.0, 2.0), Vec2::new(10.0, 5.0), Color::RED);
        }
        fn as_any(&self) -> &dyn Any { self }
        fn as_any_mut(&mut self) -> &mut dyn Any { self }
    }

    #[test]
    fn process_returns_draw_calls_from_nodes() {
        let mut tree = SceneTree::new();
        tree.set_root_scene(Box::new(DrawingNode { name: "D".into() }));

        let (mut r, mut p, mut a, d, i) = make_servers();
        let draws = tree.process(0.016, &[], &mut r, &mut p, &mut a, &d, &i);
        assert_eq!(draws.len(), 1);
        assert_eq!(draws[0].position, Vec2::new(1.0, 2.0));
    }

    // ── deferred commands ─────────────────────────────────────────────────────

    struct SpawnerNode { spawned: bool }

    impl Node for SpawnerNode {
        fn name(&self) -> &str { "Spawner" }
        fn process(&mut self, _delta: f64, ctx: &mut NodeContext) {
            if !self.spawned {
                self.spawned = true;
                ctx.commands.push(SceneCommand::AddChild {
                    parent: ctx.this_id,
                    node:   BaseNode::new("Spawned"),
                });
            }
        }
        fn as_any(&self) -> &dyn Any { self }
        fn as_any_mut(&mut self) -> &mut dyn Any { self }
    }

    #[test]
    fn deferred_add_child_applied_after_process() {
        let mut tree = SceneTree::new();
        let root = tree.set_root_scene(Box::new(SpawnerNode { spawned: false }));

        assert_eq!(tree.children_of(root).len(), 0);
        let (mut r, mut p, mut a, d, i) = make_servers();
        tree.process(0.016, &[], &mut r, &mut p, &mut a, &d, &i);
        assert_eq!(tree.children_of(root).len(), 1);
    }

    #[test]
    fn deferred_queue_free_removes_node() {
        let (mut tree, root) = tree_with_root("Root");
        let child = tree.add_child(Some(root), BaseNode::new("Doomed"));

        // Manually apply a QueueFree command
        tree.apply_commands(vec![SceneCommand::QueueFree(child)]);
        assert!(tree.get(child).is_none());
    }
}
