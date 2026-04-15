//! **Pong** — a complete two-player pong game built on the Chainlink Game Engine.
//!
//! Controls
//! ─────────
//!   Player 1 (left):   W / S
//!   Player 2 (right):  Up / Down arrow
//!   Restart after goal: R

use std::any::Any;

use chainlink_core::{Color, Vec2};
use chainlink_engine::Engine;
use chainlink_input::{InputEvent, Key};
use chainlink_scene::node::{Node, NodeContext};

// ── Constants ─────────────────────────────────────────────────────────────────

const W: f32 = 1280.0;
const H: f32 = 720.0;

const PADDLE_W:   f32 = 18.0;
const PADDLE_H:   f32 = 120.0;
const PADDLE_SPD: f32 = 500.0;
const PADDLE_MARGIN: f32 = 24.0;

const BALL_SIZE: f32 = 18.0;
const BALL_SPD:  f32 = 420.0;

const NET_SEGMENT_H: f32 = 24.0;
const NET_SEGMENT_W: f32 = 6.0;
const NET_GAP:       f32 = 16.0;
const NET_X:         f32 = W * 0.5 - NET_SEGMENT_W * 0.5;

// ── Pong root node ────────────────────────────────────────────────────────────

struct PongGame {
    p1_y:    f32,
    p2_y:    f32,

    ball_pos: Vec2,
    ball_vel: Vec2,

    score_p1: u32,
    score_p2: u32,

    scored: bool, // true during the brief pause after a goal
    scored_timer: f32,
}

impl PongGame {
    fn new() -> Box<Self> {
        Box::new(Self {
            p1_y:    H * 0.5 - PADDLE_H * 0.5,
            p2_y:    H * 0.5 - PADDLE_H * 0.5,
            ball_pos: Vec2::new(W * 0.5 - BALL_SIZE * 0.5, H * 0.5 - BALL_SIZE * 0.5),
            ball_vel: Vec2::new(BALL_SPD, BALL_SPD * 0.6),
            score_p1: 0,
            score_p2: 0,
            scored:   false,
            scored_timer: 0.0,
        })
    }

    fn reset_ball(&mut self, dir: f32) {
        self.ball_pos = Vec2::new(W * 0.5 - BALL_SIZE * 0.5, H * 0.5 - BALL_SIZE * 0.5);
        self.ball_vel = Vec2::new(BALL_SPD * dir, BALL_SPD * 0.6);
    }

}

// We need to track held keys because `process()` doesn't receive the
// InputServer.  We'll store pressed state directly.
struct PongRoot {
    game: PongGame,
    key_w:    bool,
    key_s:    bool,
    key_up:   bool,
    key_down: bool,
}

impl PongRoot {
    fn new() -> Box<Self> {
        Box::new(Self {
            game:     *PongGame::new(),  // unbox
            key_w:    false,
            key_s:    false,
            key_up:   false,
            key_down: false,
        })
    }
}

impl Node for PongRoot {
    fn name(&self) -> &str { "PongRoot" }

    fn input(&mut self, event: &InputEvent, _ctx: &mut NodeContext) {
        match event {
            InputEvent::KeyPressed  { key: Key::W,    .. } => self.key_w    = true,
            InputEvent::KeyReleased { key: Key::W         } => self.key_w    = false,
            InputEvent::KeyPressed  { key: Key::S,    .. } => self.key_s    = true,
            InputEvent::KeyReleased { key: Key::S         } => self.key_s    = false,
            InputEvent::KeyPressed  { key: Key::Up,   .. } => self.key_up   = true,
            InputEvent::KeyReleased { key: Key::Up        } => self.key_up   = false,
            InputEvent::KeyPressed  { key: Key::Down, .. } => self.key_down  = true,
            InputEvent::KeyReleased { key: Key::Down       } => self.key_down  = false,
            _ => {}
        }
    }

    fn process(&mut self, delta: f64, ctx: &mut NodeContext) {
        let dt = delta as f32;
        let g  = &mut self.game;

        // ── Scored pause ──────────────────────────────────────────────────────
        if g.scored {
            g.scored_timer -= dt;
            if g.scored_timer <= 0.0 { g.scored = false; }
            self.draw_scene(ctx);
            return;
        }

        // ── Paddle movement ───────────────────────────────────────────────────
        let p1_dir = if self.key_w { -1.0 } else if self.key_s { 1.0 } else { 0.0 };
        let p2_dir = if self.key_up { -1.0 } else if self.key_down { 1.0 } else { 0.0 };

        g.p1_y = (g.p1_y + p1_dir * PADDLE_SPD * dt).clamp(0.0, H - PADDLE_H);
        g.p2_y = (g.p2_y + p2_dir * PADDLE_SPD * dt).clamp(0.0, H - PADDLE_H);

        // ── Ball movement ─────────────────────────────────────────────────────
        g.ball_pos += g.ball_vel * dt;

        // Top/bottom walls
        if g.ball_pos.y <= 0.0 {
            g.ball_pos.y  = 0.0;
            g.ball_vel.y  = g.ball_vel.y.abs();
        }
        if g.ball_pos.y + BALL_SIZE >= H {
            g.ball_pos.y  = H - BALL_SIZE;
            g.ball_vel.y  = -g.ball_vel.y.abs();
        }

        // ── Paddle collisions ─────────────────────────────────────────────────
        // Player 1 paddle
        let p1_x = PADDLE_MARGIN;
        if g.ball_vel.x < 0.0
            && g.ball_pos.x <= p1_x + PADDLE_W
            && g.ball_pos.x + BALL_SIZE >= p1_x
            && g.ball_pos.y + BALL_SIZE >= g.p1_y
            && g.ball_pos.y <= g.p1_y + PADDLE_H
        {
            let hit_pos  = (g.ball_pos.y + BALL_SIZE * 0.5) - (g.p1_y + PADDLE_H * 0.5);
            let norm_hit = (hit_pos / (PADDLE_H * 0.5)).clamp(-1.0, 1.0);
            let speed    = g.ball_vel.length() * 1.05; // slight speed-up
            let angle    = norm_hit * std::f32::consts::FRAC_PI_3; // ±60°
            g.ball_vel   = Vec2::new(angle.cos(), angle.sin()) * speed;
            g.ball_pos.x = p1_x + PADDLE_W + 1.0;
        }

        // Player 2 paddle
        let p2_x = W - PADDLE_MARGIN - PADDLE_W;
        if g.ball_vel.x > 0.0
            && g.ball_pos.x + BALL_SIZE >= p2_x
            && g.ball_pos.x <= p2_x + PADDLE_W
            && g.ball_pos.y + BALL_SIZE >= g.p2_y
            && g.ball_pos.y <= g.p2_y + PADDLE_H
        {
            let hit_pos  = (g.ball_pos.y + BALL_SIZE * 0.5) - (g.p2_y + PADDLE_H * 0.5);
            let norm_hit = (hit_pos / (PADDLE_H * 0.5)).clamp(-1.0, 1.0);
            let speed    = g.ball_vel.length() * 1.05;
            let angle    = norm_hit * std::f32::consts::FRAC_PI_3;
            // Reverse x direction
            g.ball_vel   = Vec2::new(-angle.cos(), angle.sin()) * speed;
            g.ball_pos.x = p2_x - BALL_SIZE - 1.0;
        }

        // ── Scoring ───────────────────────────────────────────────────────────
        if g.ball_pos.x + BALL_SIZE < 0.0 {
            g.score_p2   += 1;
            g.scored      = true;
            g.scored_timer = 1.0;
            g.reset_ball(1.0);
        }
        if g.ball_pos.x > W {
            g.score_p1   += 1;
            g.scored      = true;
            g.scored_timer = 1.0;
            g.reset_ball(-1.0);
        }

        self.draw_scene(ctx);
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

impl PongRoot {
    fn draw_scene(&self, ctx: &mut NodeContext) {
        let g = &self.game;

        // ── Net (dashed centre line) ──────────────────────────────────────────
        let mut y = 0.0_f32;
        while y < H {
            ctx.draw_rect(
                Vec2::new(NET_X, y),
                Vec2::new(NET_SEGMENT_W, NET_SEGMENT_H),
                Color::GRAY,
            );
            y += NET_SEGMENT_H + NET_GAP;
        }

        // ── Paddles ───────────────────────────────────────────────────────────
        ctx.draw_rect(
            Vec2::new(PADDLE_MARGIN, g.p1_y),
            Vec2::new(PADDLE_W, PADDLE_H),
            Color::WHITE,
        );
        ctx.draw_rect(
            Vec2::new(W - PADDLE_MARGIN - PADDLE_W, g.p2_y),
            Vec2::new(PADDLE_W, PADDLE_H),
            Color::WHITE,
        );

        // ── Ball ──────────────────────────────────────────────────────────────
        ctx.draw_rect(g.ball_pos, Vec2::splat(BALL_SIZE), Color::YELLOW);

        // ── Score pips (simple pixel-art digits) ─────────────────────────────
        draw_score(ctx, g.score_p1, W * 0.25);
        draw_score(ctx, g.score_p2, W * 0.75);
    }
}

// ── Score digit renderer (block-pixel style) ──────────────────────────────────

fn draw_score(ctx: &mut NodeContext, score: u32, center_x: f32) {
    let digit = (score % 10) as usize;
    let block = 12.0_f32;
    let ox    = center_x - block * 1.5;
    let oy    = 40.0_f32;

    // 7-segment-style bitmask for digits 0-9.
    // Each digit is a 3×5 grid of blocks.
    const DIGITS: [[u16; 5]; 10] = [
        [0b111, 0b101, 0b101, 0b101, 0b111], // 0
        [0b010, 0b010, 0b010, 0b010, 0b010], // 1
        [0b111, 0b001, 0b111, 0b100, 0b111], // 2
        [0b111, 0b001, 0b111, 0b001, 0b111], // 3
        [0b101, 0b101, 0b111, 0b001, 0b001], // 4
        [0b111, 0b100, 0b111, 0b001, 0b111], // 5
        [0b111, 0b100, 0b111, 0b101, 0b111], // 6
        [0b111, 0b001, 0b001, 0b001, 0b001], // 7
        [0b111, 0b101, 0b111, 0b101, 0b111], // 8
        [0b111, 0b101, 0b111, 0b001, 0b111], // 9
    ];

    for (row, &mask) in DIGITS[digit].iter().enumerate() {
        for col in 0..3usize {
            if mask & (1 << (2 - col)) != 0 {
                ctx.draw_rect(
                    Vec2::new(ox + col as f32 * block, oy + row as f32 * block),
                    Vec2::splat(block - 2.0),
                    Color::WHITE,
                );
            }
        }
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    let engine = Engine::builder()
        .title("Pong — Chainlink Game Engine")
        .size(W as u32, H as u32)
        .build();

    engine.run(|tree, _input| {
        tree.set_root_scene(PongRoot::new());
    });
}
