//! `RenderingServer` — RID-based 2-D canvas API, mirroring Godot's RenderingServer.
//!
//! The scene layer records draw commands via RIDs; the render backend
//! (cl_render) consumes them each frame via `collect_draw_calls()`.

use std::collections::HashMap;
use cl_core::{Color, Rect2, Rid, Transform2D, Vec2};

// ─── Draw primitives ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum DrawCommand {
    Rect   { rect: Rect2, color: Color },
    Circle { center: Vec2, radius: f32, color: Color },
    Line   { from: Vec2, to: Vec2, color: Color, width: f32 },
}

// ─── Canvas item ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CanvasItem {
    pub transform: Transform2D,
    pub commands:  Vec<DrawCommand>,
    pub visible:   bool,
}

impl CanvasItem {
    fn new() -> Self {
        Self {
            transform: Transform2D::IDENTITY,
            commands:  Vec::new(),
            visible:   true,
        }
    }
}

// ─── RenderingServer ─────────────────────────────────────────────────────────

/// Manages a flat list of canvas items identified by [`Rid`]s.
///
/// Nodes call the mutating methods to describe what should be drawn;
/// the renderer calls [`collect_draw_calls`] to consume the results.
pub struct RenderingServer {
    items: HashMap<Rid, CanvasItem>,
}

impl RenderingServer {
    pub fn new() -> Self {
        Self { items: HashMap::new() }
    }

    // ── Canvas-item lifecycle ─────────────────────────────────────────────────

    /// Allocate a new canvas item and return its RID.
    pub fn canvas_item_create(&mut self) -> Rid {
        let rid = Rid::generate();
        self.items.insert(rid, CanvasItem::new());
        rid
    }

    /// Set the 2-D transform for a canvas item.
    pub fn canvas_item_set_transform(&mut self, rid: Rid, transform: Transform2D) {
        if let Some(item) = self.items.get_mut(&rid) {
            item.transform = transform;
        }
    }

    /// Set visibility for a canvas item.
    pub fn canvas_item_set_visible(&mut self, rid: Rid, visible: bool) {
        if let Some(item) = self.items.get_mut(&rid) {
            item.visible = visible;
        }
    }

    // ── Draw commands ─────────────────────────────────────────────────────────

    /// Append a filled rectangle draw command to a canvas item.
    pub fn canvas_item_add_rect(&mut self, rid: Rid, rect: Rect2, color: Color) {
        if let Some(item) = self.items.get_mut(&rid) {
            item.commands.push(DrawCommand::Rect { rect, color });
        }
    }

    /// Append a filled circle draw command to a canvas item.
    pub fn canvas_item_add_circle(&mut self, rid: Rid, center: Vec2, radius: f32, color: Color) {
        if let Some(item) = self.items.get_mut(&rid) {
            item.commands.push(DrawCommand::Circle { center, radius, color });
        }
    }

    /// Append a line draw command to a canvas item.
    pub fn canvas_item_add_line(
        &mut self,
        rid: Rid,
        from: Vec2,
        to: Vec2,
        color: Color,
        width: f32,
    ) {
        if let Some(item) = self.items.get_mut(&rid) {
            item.commands.push(DrawCommand::Line { from, to, color, width });
        }
    }

    /// Remove all draw commands from a canvas item (keep the item itself).
    pub fn canvas_item_clear(&mut self, rid: Rid) {
        if let Some(item) = self.items.get_mut(&rid) {
            item.commands.clear();
        }
    }

    /// Destroy a canvas item entirely.
    pub fn free_rid(&mut self, rid: Rid) {
        self.items.remove(&rid);
    }

    // ── Frame collection ──────────────────────────────────────────────────────

    /// Return all (transform, commands) pairs for the current frame.
    ///
    /// The renderer iterates this list and issues actual GPU draw calls.
    pub fn collect_draw_calls(&self) -> Vec<(Transform2D, &[DrawCommand])> {
        self.items
            .values()
            .filter(|item| item.visible)
            .map(|item| (item.transform, item.commands.as_slice()))
            .collect()
    }
}

impl Default for RenderingServer {
    fn default() -> Self { Self::new() }
}
