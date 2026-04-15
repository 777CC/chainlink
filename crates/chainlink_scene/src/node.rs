//! Core node abstraction.

use std::any::Any;
use chainlink_core::{Transform2D, Variant, Vec2};
use chainlink_input::InputEvent;
use slotmap::new_key_type;

new_key_type! {
    /// Stable, opaque handle to a node in the [`SceneTree`].
    pub struct NodeId;
}

// ─── Draw command ─────────────────────────────────────────────────────────────

/// A draw call emitted by a node during `process` / `draw`.
/// The renderer consumes these each frame.
#[derive(Debug, Clone)]
pub struct DrawRect {
    pub position: Vec2,
    pub size:     Vec2,
    pub color:    chainlink_core::Color,
    pub z_index:  i32,
}

// ─── Scene commands ──────────────────────────────────────────────────────────

/// Deferred structural changes requested by nodes during `process`.
pub enum SceneCommand {
    /// Add a new node as a child of `parent`.
    AddChild { parent: NodeId, node: Box<dyn Node> },
    /// Remove (and drop) a node and all its children.
    QueueFree(NodeId),
    /// Replace the active scene.
    ChangeScene(Box<dyn Node>),
}

// ─── NodeContext ─────────────────────────────────────────────────────────────

/// Context passed to node lifecycle callbacks.
///
/// Contains read access to the tree and a mutable command queue so that
/// nodes can request structural changes without borrowing the tree mutably.
pub struct NodeContext<'a> {
    /// The id of the node currently being processed.
    pub this_id: NodeId,
    /// Pending draw calls (nodes push into this).
    pub draw_queue: &'a mut Vec<DrawRect>,
    /// Deferred scene commands.
    pub commands: &'a mut Vec<SceneCommand>,
}

impl<'a> NodeContext<'a> {
    /// Enqueue a colored rectangle draw call.
    pub fn draw_rect(&mut self, position: Vec2, size: Vec2, color: chainlink_core::Color) {
        self.draw_queue.push(DrawRect { position, size, color, z_index: 0 });
    }

    /// Schedule removal of a node (safe to call with `this_id`).
    pub fn queue_free(&mut self, id: NodeId) {
        self.commands.push(SceneCommand::QueueFree(id));
    }
}

// ─── Node trait ───────────────────────────────────────────────────────────────

/// The fundamental building block of the engine, mirroring Chainlink's `Node`.
///
/// Implement this trait for every game object.  The engine calls:
/// 1. [`Node::ready`]   — once, after the node enters the tree.
/// 2. [`Node::process`] — every frame with the elapsed `delta` (seconds).
/// 3. [`Node::input`]   — for each raw input event that frame.
pub trait Node: Any + Send {
    /// Human-readable name (e.g. `"Player"`, `"Ball"`).
    fn name(&self) -> &str;

    // ── Lifecycle ─────────────────────────────────────────────────────────────

    /// Called once after the node (and its children) have entered the tree.
    fn ready(&mut self, _ctx: &mut NodeContext) {}

    /// Called every frame.  `delta` is elapsed seconds since the last frame.
    fn process(&mut self, _delta: f64, _ctx: &mut NodeContext) {}

    /// Called for every input event that occurred this frame.
    fn input(&mut self, _event: &InputEvent, _ctx: &mut NodeContext) {}

    // ── Transform ─────────────────────────────────────────────────────────────

    /// World-space 2-D transform.  Override to expose a transform.
    fn transform(&self) -> Transform2D { Transform2D::IDENTITY }

    // ── Properties ────────────────────────────────────────────────────────────

    /// Get a named property value.
    fn get(&self, _property: &str) -> Option<Variant> { None }

    /// Set a named property value.  Returns `true` if the property was found.
    fn set(&mut self, _property: &str, _value: Variant) -> bool { false }

    // ── Downcast helpers ──────────────────────────────────────────────────────

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// ─── Blanket base node ────────────────────────────────────────────────────────

/// A plain node with no built-in behaviour — the equivalent of Chainlink's bare
/// `Node`.  Use it as a grouping container.
pub struct BaseNode {
    pub name: String,
}

impl BaseNode {
    pub fn new(name: impl Into<String>) -> Box<Self> {
        Box::new(Self { name: name.into() })
    }
}

impl Node for BaseNode {
    fn name(&self) -> &str { &self.name }
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
