//! Core math types — thin re-exports of `glam` plus engine-specific helpers.

pub use glam::{Mat4, Quat, Vec2, Vec3, Vec4};

// ─── Transform2D ────────────────────────────────────────────────────────────

/// A 2-D transform: position + rotation (radians) + non-uniform scale.
///
/// Mirrors Godot's `Transform2D` but stored as decomposed components so that
/// editing individual axes is ergonomic.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform2D {
    pub position: Vec2,
    /// Rotation in radians (counter-clockwise positive).
    pub rotation: f32,
    pub scale: Vec2,
}

impl Transform2D {
    pub const IDENTITY: Self = Self {
        position: Vec2::ZERO,
        rotation: 0.0,
        scale: Vec2::ONE,
    };

    pub fn new(position: Vec2, rotation: f32, scale: Vec2) -> Self {
        Self { position, rotation, scale }
    }

    /// Build an orthographic model matrix suitable for uploading to a shader.
    pub fn to_mat4(&self) -> Mat4 {
        let (sin, cos) = self.rotation.sin_cos();
        Mat4::from_cols(
            glam::Vec4::new(cos * self.scale.x, sin * self.scale.x, 0.0, 0.0),
            glam::Vec4::new(-sin * self.scale.y, cos * self.scale.y, 0.0, 0.0),
            glam::Vec4::new(0.0, 0.0, 1.0, 0.0),
            glam::Vec4::new(self.position.x, self.position.y, 0.0, 1.0),
        )
    }

    pub fn translated(self, delta: Vec2) -> Self {
        Self { position: self.position + delta, ..self }
    }

    pub fn rotated(self, angle: f32) -> Self {
        Self { rotation: self.rotation + angle, ..self }
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::IDENTITY
    }
}

// ─── Rect2 ──────────────────────────────────────────────────────────────────

/// Axis-aligned bounding rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect2 {
    pub position: Vec2,
    pub size: Vec2,
}

impl Rect2 {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            size: Vec2::new(w, h),
        }
    }

    /// Bottom-right corner (exclusive).
    #[inline]
    pub fn end(self) -> Vec2 {
        self.position + self.size
    }

    #[inline]
    pub fn center(self) -> Vec2 {
        self.position + self.size * 0.5
    }

    pub fn contains_point(self, p: Vec2) -> bool {
        let end = self.end();
        p.x >= self.position.x && p.y >= self.position.y
            && p.x < end.x && p.y < end.y
    }

    pub fn intersects(self, other: Rect2) -> bool {
        let a_end = self.end();
        let b_end = other.end();
        self.position.x < b_end.x
            && a_end.x > other.position.x
            && self.position.y < b_end.y
            && a_end.y > other.position.y
    }
}
