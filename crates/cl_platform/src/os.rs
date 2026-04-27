//! `OS` — operating-system queries (name, executable path, env vars, ticks).

/// OS-level utilities mirroring Godot's `OS` singleton.
pub struct OS;

impl OS {
    /// Short name of the host OS.
    pub fn name() -> &'static str {
        if cfg!(target_os = "linux")        { "Linux"   }
        else if cfg!(target_os = "windows") { "Windows" }
        else if cfg!(target_os = "macos")   { "macOS"   }
        else                                { "Unknown" }
    }

    /// Path to the current executable (best-effort; returns empty on error).
    pub fn exe_path() -> std::path::PathBuf {
        std::env::current_exe().unwrap_or_default()
    }

    /// Read an environment variable.
    pub fn get_env(var: &str) -> Option<String> {
        std::env::var(var).ok()
    }

    /// Milliseconds since the Unix epoch.
    pub fn ticks_msec() -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    }
}
