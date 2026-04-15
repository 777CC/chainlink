//! `Renderer` — wgpu-backed 2-D renderer.
//!
//! Initialised once with a `wgpu::Surface`; call [`Renderer::render`] every
//! frame with the draw queue produced by the scene tree.

use bytemuck::cast_slice;
use gd_core::{Color, Vec2};
use gd_scene::node::DrawRect;
use glam::Mat4;
use log::debug;
use wgpu::util::DeviceExt;

use crate::{
    pipeline::SHADER_SRC,
    vertex::{Instance, Vertex, QUAD_INDICES, QUAD_VERTICES},
};

// ── Camera uniform ────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn orthographic(width: f32, height: f32) -> Self {
        // Map pixel coords: top-left (0,0) → bottom-right (w,h)
        let mat = Mat4::orthographic_rh(0.0, width, height, 0.0, -1000.0, 1000.0);
        Self { view_proj: mat.to_cols_array_2d() }
    }
}

// ── Renderer ──────────────────────────────────────────────────────────────────

/// The main GPU renderer.
pub struct Renderer {
    pub surface:       wgpu::Surface<'static>,
    pub device:        wgpu::Device,
    pub queue:         wgpu::Queue,
    pub config:        wgpu::SurfaceConfiguration,
    pub size:          (u32, u32),

    pipeline:          wgpu::RenderPipeline,
    vertex_buf:        wgpu::Buffer,
    index_buf:         wgpu::Buffer,
    instance_buf:      wgpu::Buffer,
    camera_buf:        wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    pub clear_color: Color,
    instance_cap: usize,
}

const INITIAL_INSTANCE_CAP: usize = 1024;

impl Renderer {
    /// Create the renderer.
    ///
    /// `instance` must be the same `wgpu::Instance` that was used to create
    /// `surface`.  `size` is the initial framebuffer size in pixels.
    pub async fn new(
        instance: wgpu::Instance,
        surface: wgpu::Surface<'static>,
        size: (u32, u32),
    ) -> Self {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference:       wgpu::PowerPreference::HighPerformance,
                compatible_surface:     Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("No suitable GPU adapter found");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label:             Some("gd_render device"),
                    required_features: wgpu::Features::empty(),
                    required_limits:   wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create wgpu device");

        let surface_caps   = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage:        wgpu::TextureUsages::RENDER_ATTACHMENT,
            format:       surface_format,
            width:        size.0,
            height:       size.1,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode:   surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // ── Buffers ───────────────────────────────────────────────────────────

        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("quad_vertices"),
            contents: cast_slice(&QUAD_VERTICES),
            usage:    wgpu::BufferUsages::VERTEX,
        });

        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("quad_indices"),
            contents: cast_slice(&QUAD_INDICES),
            usage:    wgpu::BufferUsages::INDEX,
        });

        let instance_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label:              Some("instance_buf"),
            size:               (INITIAL_INSTANCE_CAP * std::mem::size_of::<Instance>()) as u64,
            usage:              wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // ── Camera uniform ────────────────────────────────────────────────────

        let camera_uniform = CameraUniform::orthographic(size.0 as f32, size.1 as f32);
        let camera_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("camera_buf"),
            contents: cast_slice(&[camera_uniform]),
            usage:    wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   Some("camera_bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding:    0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty:         wgpu::BindingType::Buffer {
                    ty:                 wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size:   None,
                },
                count: None,
            }],
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   Some("camera_bg"),
            layout:  &camera_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding:  0,
                resource: camera_buf.as_entire_binding(),
            }],
        });

        // ── Pipeline ──────────────────────────────────────────────────────────

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label:  Some("quad_shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER_SRC.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                Some("pipeline_layout"),
            bind_group_layouts:   &[&camera_bgl],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:  Some("quad_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module:      &shader,
                entry_point: "vs_main",
                buffers:     &[Vertex::layout(), Instance::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module:      &shader,
                entry_point: "fs_main",
                targets:     &[Some(wgpu::ColorTargetState {
                    format:     config.format,
                    blend:      Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology:           wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face:         wgpu::FrontFace::Ccw,
                cull_mode:          None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample:   wgpu::MultisampleState::default(),
            multiview:     None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
            vertex_buf,
            index_buf,
            instance_buf,
            camera_buf,
            camera_bind_group,
            clear_color: Color::DARK_GRAY,
            instance_cap: INITIAL_INSTANCE_CAP,
        }
    }

    // ── Resize ────────────────────────────────────────────────────────────────

    pub fn resize(&mut self, new_size: (u32, u32)) {
        if new_size.0 == 0 || new_size.1 == 0 { return; }
        self.size = new_size;
        self.config.width  = new_size.0;
        self.config.height = new_size.1;
        self.surface.configure(&self.device, &self.config);

        let cam = CameraUniform::orthographic(new_size.0 as f32, new_size.1 as f32);
        self.queue.write_buffer(&self.camera_buf, 0, cast_slice(&[cam]));
    }

    // ── Render ────────────────────────────────────────────────────────────────

    /// Draw the list of rects produced by the scene tree this frame.
    pub fn render(&mut self, mut draw_queue: Vec<DrawRect>) -> Result<(), wgpu::SurfaceError> {
        draw_queue.sort_by_key(|r| r.z_index);

        let output  = self.surface.get_current_texture()?;
        let view    = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut enc = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("frame_encoder"),
        });

        let instances: Vec<Instance> = draw_queue
            .iter()
            .map(|r| {
                let model = Mat4::from_scale_rotation_translation(
                    glam::Vec3::new(r.size.x, r.size.y, 1.0),
                    glam::Quat::IDENTITY,
                    glam::Vec3::new(r.position.x, r.position.y, 0.0),
                );
                Instance {
                    model: model.to_cols_array_2d(),
                    color: r.color.to_array(),
                }
            })
            .collect();

        if !instances.is_empty() {
            if instances.len() > self.instance_cap {
                self.instance_cap = instances.len().next_power_of_two();
                self.instance_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
                    label:              Some("instance_buf"),
                    size:               (self.instance_cap * std::mem::size_of::<Instance>()) as u64,
                    usage:              wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                debug!("Renderer: grew instance buffer to {} instances", self.instance_cap);
            }
            self.queue.write_buffer(&self.instance_buf, 0, cast_slice(&instances));
        }

        let cc = self.clear_color;
        {
            let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label:             Some("main_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view:           &view,
                    resolve_target: None,
                    ops:            wgpu::Operations {
                        load:  wgpu::LoadOp::Clear(wgpu::Color {
                            r: cc.r as f64,
                            g: cc.g as f64,
                            b: cc.b as f64,
                            a: cc.a as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes:         None,
                occlusion_query_set:      None,
            });

            if !instances.is_empty() {
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, &self.camera_bind_group, &[]);
                pass.set_vertex_buffer(0, self.vertex_buf.slice(..));
                pass.set_vertex_buffer(1, self.instance_buf.slice(..));
                pass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
                pass.draw_indexed(0..6, 0, 0..instances.len() as u32);
            }
        }

        self.queue.submit(std::iter::once(enc.finish()));
        output.present();
        Ok(())
    }

    pub fn viewport_size(&self) -> Vec2 {
        Vec2::new(self.size.0 as f32, self.size.1 as f32)
    }
}
