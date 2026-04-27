// Opaque resource handle (u64 counter-based), same as Godot's RID.
// Used by servers to identify GPU objects, physics bodies, audio buses, etc.
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rid(u64);

impl Rid {
    pub fn new(id: u64) -> Self { Self(id) }
    pub fn id(self) -> u64 { self.0 }
    pub fn is_valid(self) -> bool { self.0 != 0 }
    pub const INVALID: Self = Self(0);
}

// Thread-safe counter for issuing unique RIDs
static NEXT_RID: AtomicU64 = AtomicU64::new(1);

impl Rid {
    pub fn generate() -> Self { Self(NEXT_RID.fetch_add(1, Ordering::Relaxed)) }
}
