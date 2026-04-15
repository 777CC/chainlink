//! GPU vertex and instance types.

use bytemuck::{Pod, Zeroable};

/// A single vertex of the unit quad.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
}

impl Vertex {
    /// Vertex buffer layout descriptor.
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode:    wgpu::VertexStepMode::Vertex,
            attributes:   &[wgpu::VertexAttribute {
                offset:          0,
                shader_location: 0,
                format:          wgpu::VertexFormat::Float32x2,
            }],
        }
    }
}

/// Per-instance data uploaded once per `DrawRect`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Instance {
    /// Column-major 4×4 model matrix.
    pub model: [[f32; 4]; 4],
    /// RGBA color.
    pub color: [f32; 4],
}

impl Instance {
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode:    wgpu::VertexStepMode::Instance,
            attributes: &[
                // model col 0  (location 1)
                wgpu::VertexAttribute { offset: 0,  shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                // model col 1  (location 2)
                wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
                // model col 2  (location 3)
                wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
                // model col 3  (location 4)
                wgpu::VertexAttribute { offset: 48, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                // color        (location 5)
                wgpu::VertexAttribute { offset: 64, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
            ],
        }
    }
}

/// Unit quad vertices: two triangles covering `[0,0]–[1,1]`.
pub const QUAD_VERTICES: [Vertex; 4] = [
    Vertex { position: [0.0, 0.0] },
    Vertex { position: [1.0, 0.0] },
    Vertex { position: [1.0, 1.0] },
    Vertex { position: [0.0, 1.0] },
];

/// Index buffer for the unit quad (two CCW triangles).
pub const QUAD_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];
