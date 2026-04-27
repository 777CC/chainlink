//! `AudioServer` — audio playback via rodio (or a silent stub when unavailable).
//!
//! Opening the default audio device can fail in headless / CI environments.
//! When it does, the server logs a warning and silently no-ops every method.

// ── Rodio backend (compiled only when the `audio` feature is enabled) ─────────

#[cfg(feature = "audio")]
mod backend {
    use log::warn;
    use rodio::{OutputStream, OutputStreamHandle, Sink};
    use std::io::Cursor;
    use std::sync::{Arc, Mutex};

    pub struct AudioBackend {
        // Keep the stream alive for as long as the server lives
        _stream:       OutputStream,
        stream_handle: OutputStreamHandle,
        sinks:         Arc<Mutex<Vec<Sink>>>,
        master_volume: f32,
    }

    impl AudioBackend {
        pub fn try_new() -> Option<Self> {
            match OutputStream::try_default() {
                Ok((stream, handle)) => Some(Self {
                    _stream:       stream,
                    stream_handle: handle,
                    sinks:         Arc::new(Mutex::new(Vec::new())),
                    master_volume: 1.0,
                }),
                Err(e) => {
                    warn!("AudioServer: could not open audio device ({e}); audio will be silent");
                    None
                }
            }
        }

        pub fn play_wav_bytes(&self, bytes: &'static [u8]) {
            let cursor = Cursor::new(bytes);
            let sink = match Sink::try_new(&self.stream_handle) {
                Ok(s) => s,
                Err(e) => { warn!("AudioServer: Sink::try_new failed: {e}"); return; }
            };
            match rodio::Decoder::new(cursor) {
                Ok(source) => {
                    sink.set_volume(self.master_volume);
                    sink.append(source);
                    if let Ok(mut sinks) = self.sinks.lock() {
                        // Remove finished sinks
                        sinks.retain(|s| !s.empty());
                        sinks.push(sink);
                    }
                }
                Err(e) => { warn!("AudioServer: decode failed: {e}"); }
            }
        }

        pub fn set_master_volume(&mut self, linear: f32) {
            self.master_volume = linear.max(0.0);
            if let Ok(sinks) = self.sinks.lock() {
                for s in sinks.iter() {
                    s.set_volume(self.master_volume);
                }
            }
        }

        pub fn stop_all(&self) {
            if let Ok(mut sinks) = self.sinks.lock() {
                for s in sinks.drain(..) {
                    s.stop();
                }
            }
        }

        pub fn is_playing(&self) -> bool {
            self.sinks.lock()
                .map(|sinks| sinks.iter().any(|s| !s.empty()))
                .unwrap_or(false)
        }
    }
}

// ─── AudioServer ─────────────────────────────────────────────────────────────

/// Audio playback server.
///
/// On platforms where opening the audio device fails (e.g. headless CI),
/// all methods silently no-op and [`is_playing`] always returns `false`.
pub struct AudioServer {
    #[cfg(feature = "audio")]
    backend: Option<backend::AudioBackend>,
}

impl AudioServer {
    /// Try to open the default audio output device.
    pub fn new() -> Self {
        #[cfg(feature = "audio")]
        {
            Self { backend: backend::AudioBackend::try_new() }
        }
        #[cfg(not(feature = "audio"))]
        {
            warn!("AudioServer: compiled without audio support; all audio is a no-op");
            Self {}
        }
    }

    /// Play raw WAV bytes (typically embedded with `include_bytes!`).
    pub fn play_wav_bytes(&self, bytes: &'static [u8]) {
        #[cfg(feature = "audio")]
        if let Some(b) = &self.backend { b.play_wav_bytes(bytes); }
        let _ = bytes;
    }

    /// Set the master volume (0.0 = silent, 1.0 = full, >1.0 = amplify).
    pub fn set_master_volume(&mut self, linear: f32) {
        #[cfg(feature = "audio")]
        if let Some(b) = &mut self.backend { b.set_master_volume(linear); }
        let _ = linear;
    }

    /// Stop all currently playing sounds.
    pub fn stop_all(&self) {
        #[cfg(feature = "audio")]
        if let Some(b) = &self.backend { b.stop_all(); }
    }

    /// Returns `true` if at least one sound is currently playing.
    pub fn is_playing(&self) -> bool {
        #[cfg(feature = "audio")]
        if let Some(b) = &self.backend { return b.is_playing(); }
        false
    }
}

impl Default for AudioServer {
    fn default() -> Self { Self::new() }
}
