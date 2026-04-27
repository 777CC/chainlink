//! `AudioStreamPlayer` — plays audio through the AudioServer.

use std::any::Any;
use cl_core::Transform2D;
use cl_input::InputEvent;
use crate::node::{Node, NodeContext};

/// A node that triggers audio playback via the [`AudioServer`].
///
/// If `autoplay` is `true`, a silent (empty) sound is played on `ready`.
/// Attach real audio bytes via [`play_wav`] at runtime.
pub struct AudioStreamPlayer {
    pub name:      String,
    pub autoplay:  bool,
    pub volume_db: f32,
    started:       bool,
}

impl AudioStreamPlayer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name:      name.into(),
            autoplay:  false,
            volume_db: 0.0,
            started:   false,
        }
    }

    /// Play raw WAV bytes through the audio server.
    pub fn play_wav(&self, ctx: &mut NodeContext, bytes: &'static [u8]) {
        ctx.audio.play_wav_bytes(bytes);
    }
}

/// 1-byte minimal valid WAV (used for autoplay so the server path is exercised).
/// In practice, replace with `include_bytes!("sound.wav")`.
static SILENCE_WAV: &[u8] = &[
    0x52, 0x49, 0x46, 0x46, // "RIFF"
    0x24, 0x00, 0x00, 0x00, // chunk size = 36
    0x57, 0x41, 0x56, 0x45, // "WAVE"
    0x66, 0x6D, 0x74, 0x20, // "fmt "
    0x10, 0x00, 0x00, 0x00, // subchunk size = 16
    0x01, 0x00,             // PCM
    0x01, 0x00,             // 1 channel
    0x44, 0xAC, 0x00, 0x00, // 44100 Hz
    0x88, 0x58, 0x01, 0x00, // byte rate
    0x02, 0x00,             // block align
    0x10, 0x00,             // bits per sample = 16
    0x64, 0x61, 0x74, 0x61, // "data"
    0x00, 0x00, 0x00, 0x00, // data size = 0
];

impl Node for AudioStreamPlayer {
    fn name(&self) -> &str { &self.name }

    fn ready(&mut self, ctx: &mut NodeContext) {
        if self.autoplay && !self.started {
            self.started = true;
            ctx.audio.play_wav_bytes(SILENCE_WAV);
        }
    }

    fn process(&mut self, _delta: f64, _ctx: &mut NodeContext) {}
    fn input(&mut self, _event: &InputEvent, _ctx: &mut NodeContext) {}
    fn transform(&self) -> Transform2D { Transform2D::IDENTITY }
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
