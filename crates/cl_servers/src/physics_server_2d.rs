//! `PhysicsServer2D` — rapier2d integration, mirroring Godot's PhysicsServer2D.
//!
//! Bodies are identified by [`Rid`]s.  Under the hood every body maps to a
//! rapier `RigidBodyHandle` and zero or more `ColliderHandle`s.

use std::collections::HashMap;

use cl_core::{Rid, Vec2};
use rapier2d::prelude::*;

// ─── Internal body entry ─────────────────────────────────────────────────────

struct BodyEntry {
    rb_handle:        RigidBodyHandle,
    collider_handles: Vec<ColliderHandle>,
}

// ─── PhysicsServer2D ─────────────────────────────────────────────────────────

/// Wraps a rapier2d physics world and exposes a RID-based API.
pub struct PhysicsServer2D {
    // rapier pipeline components
    gravity:            Vector<Real>,
    integration_params: IntegrationParameters,
    physics_pipeline:   PhysicsPipeline,
    island_manager:     IslandManager,
    broad_phase:        DefaultBroadPhase,
    narrow_phase:       NarrowPhase,
    rigid_body_set:     RigidBodySet,
    collider_set:       ColliderSet,
    impulse_joint_set:  ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver:         CCDSolver,

    // RID → rapier handle mapping
    bodies: HashMap<Rid, BodyEntry>,
}

// ── Conversion helpers ────────────────────────────────────────────────────────

#[inline]
fn to_rapier(v: Vec2) -> Vector<Real> { vector![v.x, v.y] }

#[inline]
fn from_rapier(v: &Vector<Real>) -> Vec2 { Vec2::new(v.x, v.y) }

// ─────────────────────────────────────────────────────────────────────────────

impl PhysicsServer2D {
    /// Create a new physics world.  Call once before using any other method.
    pub fn new() -> Self {
        Self {
            gravity:             vector![0.0, 0.0],
            integration_params:  IntegrationParameters::default(),
            physics_pipeline:    PhysicsPipeline::new(),
            island_manager:      IslandManager::new(),
            broad_phase:         DefaultBroadPhase::new(),
            narrow_phase:        NarrowPhase::new(),
            rigid_body_set:      RigidBodySet::new(),
            collider_set:        ColliderSet::new(),
            impulse_joint_set:   ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver:          CCDSolver::new(),
            bodies:              HashMap::new(),
        }
    }

    // ── Body creation ─────────────────────────────────────────────────────────

    /// Create a dynamic (movable) rigid body and return its RID.
    pub fn body_create_dynamic(&mut self, position: Vec2, linvel: Vec2) -> Rid {
        let rb = RigidBodyBuilder::dynamic()
            .translation(vector![position.x, position.y])
            .linvel(to_rapier(linvel))
            .build();
        let handle = self.rigid_body_set.insert(rb);
        let rid = Rid::generate();
        self.bodies.insert(rid, BodyEntry { rb_handle: handle, collider_handles: Vec::new() });
        rid
    }

    /// Create a static (immovable) rigid body and return its RID.
    pub fn body_create_static(&mut self, position: Vec2) -> Rid {
        let rb = RigidBodyBuilder::fixed()
            .translation(vector![position.x, position.y])
            .build();
        let handle = self.rigid_body_set.insert(rb);
        let rid = Rid::generate();
        self.bodies.insert(rid, BodyEntry { rb_handle: handle, collider_handles: Vec::new() });
        rid
    }

    // ── Collider attachment ───────────────────────────────────────────────────

    /// Attach a rectangular box collider to a body.
    pub fn body_add_collider_rect(
        &mut self,
        body: Rid,
        half_extents: Vec2,
        restitution:  f32,
        friction:     f32,
    ) {
        if let Some(entry) = self.bodies.get_mut(&body) {
            let col = ColliderBuilder::cuboid(half_extents.x, half_extents.y)
                .restitution(restitution)
                .friction(friction)
                .build();
            let ch = self.collider_set.insert_with_parent(
                col,
                entry.rb_handle,
                &mut self.rigid_body_set,
            );
            entry.collider_handles.push(ch);
        }
    }

    /// Attach a circular collider to a body.
    pub fn body_add_collider_circle(
        &mut self,
        body:        Rid,
        radius:      f32,
        restitution: f32,
        friction:    f32,
    ) {
        if let Some(entry) = self.bodies.get_mut(&body) {
            let col = ColliderBuilder::ball(radius)
                .restitution(restitution)
                .friction(friction)
                .build();
            let ch = self.collider_set.insert_with_parent(
                col,
                entry.rb_handle,
                &mut self.rigid_body_set,
            );
            entry.collider_handles.push(ch);
        }
    }

    // ── Body queries / mutations ──────────────────────────────────────────────

    /// Teleport a body to a new position.
    pub fn body_set_position(&mut self, body: Rid, pos: Vec2) {
        if let Some(entry) = self.bodies.get(&body) {
            if let Some(rb) = self.rigid_body_set.get_mut(entry.rb_handle) {
                rb.set_translation(vector![pos.x, pos.y], true);
            }
        }
    }

    /// Read the current world-space position of a body.
    pub fn body_get_position(&self, body: Rid) -> Vec2 {
        self.bodies.get(&body)
            .and_then(|e| self.rigid_body_set.get(e.rb_handle))
            .map(|rb| from_rapier(rb.translation()))
            .unwrap_or(Vec2::ZERO)
    }

    /// Set the linear velocity of a body directly.
    pub fn body_set_linear_velocity(&mut self, body: Rid, vel: Vec2) {
        if let Some(entry) = self.bodies.get(&body) {
            if let Some(rb) = self.rigid_body_set.get_mut(entry.rb_handle) {
                rb.set_linvel(to_rapier(vel), true);
            }
        }
    }

    /// Read the current linear velocity of a body.
    pub fn body_get_linear_velocity(&self, body: Rid) -> Vec2 {
        self.bodies.get(&body)
            .and_then(|e| self.rigid_body_set.get(e.rb_handle))
            .map(|rb| from_rapier(rb.linvel()))
            .unwrap_or(Vec2::ZERO)
    }

    /// Apply a central impulse (immediate velocity change ∝ 1/mass) to a body.
    pub fn body_apply_central_impulse(&mut self, body: Rid, impulse: Vec2) {
        if let Some(entry) = self.bodies.get(&body) {
            if let Some(rb) = self.rigid_body_set.get_mut(entry.rb_handle) {
                rb.apply_impulse(to_rapier(impulse), true);
            }
        }
    }

    /// Destroy a body and all its colliders.
    pub fn free_body(&mut self, body: Rid) {
        if let Some(entry) = self.bodies.remove(&body) {
            // Remove colliders first
            for ch in &entry.collider_handles {
                self.collider_set.remove(
                    *ch,
                    &mut self.island_manager,
                    &mut self.rigid_body_set,
                    true,
                );
            }
            self.rigid_body_set.remove(
                entry.rb_handle,
                &mut self.island_manager,
                &mut self.collider_set,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                true,
            );
        }
    }

    // ── Simulation step ───────────────────────────────────────────────────────

    /// Advance the simulation by `delta_seconds`.
    ///
    /// `gravity` is in world-units per second² (positive Y = down on screen).
    pub fn step(&mut self, delta_seconds: f32, gravity: Vec2) {
        self.gravity = to_rapier(gravity);
        self.integration_params.dt = delta_seconds;

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_params,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        );
    }
}

impl Default for PhysicsServer2D {
    fn default() -> Self { Self::new() }
}
