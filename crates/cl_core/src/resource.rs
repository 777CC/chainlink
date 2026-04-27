use crate::Rid;

/// Base trait for all engine resources (textures, meshes, audio streams, …).
/// Resources have an optional file path and a stable RID.
pub trait Resource: Send + Sync {
    fn rid(&self) -> Rid;
    fn resource_path(&self) -> &str { "" }
}
