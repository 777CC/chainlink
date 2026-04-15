//! WGSL shader source and pipeline creation helper.

/// WGSL shader for the instanced quad pipeline.
///
/// Group 0 binding 0 — camera uniform (4×4 orthographic view-projection).
/// Vertex attributes — position (vec2), plus per-instance model matrix (4×vec4)
/// and color (vec4).
pub const SHADER_SRC: &str = r#"
// ── Camera uniform ────────────────────────────────────────────────────────────
struct Camera {
    view_proj: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: Camera;

// ── Vertex input ──────────────────────────────────────────────────────────────
struct VertexInput {
    @location(0) position: vec2<f32>,
    // Per-instance model matrix (4 columns)
    @location(1) model_col0: vec4<f32>,
    @location(2) model_col1: vec4<f32>,
    @location(3) model_col2: vec4<f32>,
    @location(4) model_col3: vec4<f32>,
    // Per-instance color
    @location(5) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let model = mat4x4<f32>(in.model_col0, in.model_col1, in.model_col2, in.model_col3);
    var out: VertexOutput;
    out.clip_position = camera.view_proj * model * vec4<f32>(in.position, 0.0, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;
