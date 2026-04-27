//! **Platformer** — a simple side-scrolling platformer demo for the Chainlink Game Engine.
//!
//! Controls
//! ─────────
//!   A / D   — move left / right
//!   Space   — jump (only when near the ground)
//!
//! Scene layout
//! ─────────────
//!   PlatformerRoot
//!   ├─ Player       (RigidBody2D — teal box)
//!   ├─ Ground       (StaticBody2D — grey slab at bottom)
//!   ├─ Platform1    (StaticBody2D — floating platform, left)
//!   ├─ Platform2    (StaticBody2D — floating platform, right)
//!   ├─ HealthBar    (ProgressBar — green bar, top-left)
//!   └─ ScoreBar     (ProgressBar — yellow bar, top-left, below health)

use std::any::Any;

use cl_core::{Color, Vec2};
use cl_engine::Engine;
use cl_input::{InputEvent, Key};
use cl_scene::node::{Node, NodeContext, SceneCommand};
use cl_scene::nodes::{ProgressBar, RigidBody2D, StaticBody2D};

// ── Constants ─────────────────────────────────────────────────────────────────

const W: f32 = 1280.0;
const H: f32 = 720.0;

const PLAYER_W:      f32 = 40.0;
const PLAYER_H:      f32 = 56.0;
const PLAYER_SPEED:  f32 = 320.0;
const JUMP_IMPULSE:  f32 = -15_000.0; // upward (negative Y in screen-space)
const GROUND_THRESH: f32 = 8.0;       // pixels — within this of ground = "on ground"

// Ground / platform geometry (StaticBody2D positions are center-based)
const GROUND_Y: f32     = H - 30.0;
const PLATFORM1_Y: f32  = H - 220.0;
const PLATFORM2_Y: f32  = H - 380.0;

// ── Player node ───────────────────────────────────────────────────────────────

struct Player {
    body:    RigidBody2D,
    key_a:   bool,
    key_d:   bool,
    key_spc: bool,
    health:  f32,  // 0.0 – 1.0
}

impl Player {
    fn new() -> Box<Self> {
        let mut body = RigidBody2D::new("PlayerBody")
            .with_size(Vec2::new(PLAYER_W, PLAYER_H))
            .with_color(Color::CYAN);
        // Lock rotation so the box doesn't tumble
        body.gravity_scale = 1.0;
        Box::new(Self { body, key_a: false, key_d: false, key_spc: false, health: 1.0 })
    }
}

impl Node for Player {
    fn name(&self) -> &str { "Player" }

    fn ready(&mut self, ctx: &mut NodeContext) {
        self.body.ready(ctx);
        // Spawn near the top-centre
        if let Some(rid) = self.body.body_rid {
            ctx.physics_2d.body_set_position(rid, Vec2::new(W * 0.5, H * 0.25));
        }
    }

    fn process(&mut self, delta: f64, ctx: &mut NodeContext) {
        let dt = delta as f32;

        if let Some(rid) = self.body.body_rid {
            let pos = ctx.physics_2d.body_get_position(rid);
            let vel = ctx.physics_2d.body_get_linear_velocity(rid);

            // ── Horizontal movement ───────────────────────────────────────────
            let dir = if self.key_a { -1.0 } else if self.key_d { 1.0 } else { 0.0 };
            let new_vx = dir * PLAYER_SPEED;
            ctx.physics_2d.body_set_linear_velocity(rid, Vec2::new(new_vx, vel.y));

            // ── Jump ──────────────────────────────────────────────────────────
            // "On ground" heuristic: bottom of player close to ground surface
            let player_bottom = pos.y + PLAYER_H * 0.5;
            let on_ground = (player_bottom - GROUND_Y).abs() < GROUND_THRESH
                || (player_bottom - PLATFORM1_Y).abs() < GROUND_THRESH
                || (player_bottom - PLATFORM2_Y).abs() < GROUND_THRESH;

            if self.key_spc && on_ground {
                ctx.physics_2d.body_apply_central_impulse(rid, Vec2::new(0.0, JUMP_IMPULSE));
            }

            // ── Wrap / clamp horizontally ─────────────────────────────────────
            if pos.x < -PLAYER_W {
                ctx.physics_2d.body_set_position(rid, Vec2::new(W + PLAYER_W, pos.y));
            } else if pos.x > W + PLAYER_W {
                ctx.physics_2d.body_set_position(rid, Vec2::new(-PLAYER_W, pos.y));
            }

            // ── Draw player ───────────────────────────────────────────────────
            let draw_pos = ctx.physics_2d.body_get_position(rid);
            ctx.draw_rect(
                draw_pos - Vec2::new(PLAYER_W * 0.5, PLAYER_H * 0.5),
                Vec2::new(PLAYER_W, PLAYER_H),
                Color::CYAN,
            );

            // ── Tick health (shrinks over time as a demo) ─────────────────────
            self.health = (self.health - dt * 0.02).max(0.0);
        }
    }

    fn input(&mut self, event: &InputEvent, _ctx: &mut NodeContext) {
        match event {
            InputEvent::KeyPressed  { key: Key::A,     .. } => self.key_a   = true,
            InputEvent::KeyReleased { key: Key::A          } => self.key_a   = false,
            InputEvent::KeyPressed  { key: Key::D,     .. } => self.key_d   = true,
            InputEvent::KeyReleased { key: Key::D          } => self.key_d   = false,
            InputEvent::KeyPressed  { key: Key::Space, .. } => self.key_spc = true,
            InputEvent::KeyReleased { key: Key::Space       } => self.key_spc = false,
            _ => {}
        }
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

// ── Root node (wires everything together) ─────────────────────────────────────

struct PlatformerRoot;

impl PlatformerRoot {
    fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl Node for PlatformerRoot {
    fn name(&self) -> &str { "PlatformerRoot" }

    fn ready(&mut self, ctx: &mut NodeContext) {
        // Spawn children via deferred AddChild so they get their own ready() call
        ctx.commands.push(SceneCommand::AddChild {
            parent: ctx.this_id,
            node: Player::new(),
        });
        ctx.commands.push(SceneCommand::AddChild {
            parent: ctx.this_id,
            node: Box::new(StaticBody2D::new(
                "Ground",
                Vec2::new(W * 0.5, GROUND_Y),
                Vec2::new(W, 60.0),
                Color::GRAY,
            )),
        });
        ctx.commands.push(SceneCommand::AddChild {
            parent: ctx.this_id,
            node: Box::new(StaticBody2D::new(
                "Platform1",
                Vec2::new(W * 0.25, PLATFORM1_Y),
                Vec2::new(300.0, 20.0),
                Color::rgba(0.6, 0.4, 0.2, 1.0),
            )),
        });
        ctx.commands.push(SceneCommand::AddChild {
            parent: ctx.this_id,
            node: Box::new(StaticBody2D::new(
                "Platform2",
                Vec2::new(W * 0.75, PLATFORM2_Y),
                Vec2::new(300.0, 20.0),
                Color::rgba(0.6, 0.4, 0.2, 1.0),
            )),
        });
        ctx.commands.push(SceneCommand::AddChild {
            parent: ctx.this_id,
            node: Box::new(ProgressBar {
                name:       "HealthBar".into(),
                position:   Vec2::new(20.0, 20.0),
                size:       Vec2::new(200.0, 18.0),
                value:      1.0,
                bg_color:   Color::DARK_GRAY,
                fill_color: Color::GREEN,
                visible:    true,
            }),
        });
    }

    fn process(&mut self, _delta: f64, _ctx: &mut NodeContext) {}

    fn input(&mut self, _event: &InputEvent, _ctx: &mut NodeContext) {}
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    let engine = Engine::builder()
        .title("Platformer — Chainlink Game Engine")
        .size(W as u32, H as u32)
        .build();

    engine.run(|tree, _input| {
        tree.set_root_scene(PlatformerRoot::new());
    });
}
