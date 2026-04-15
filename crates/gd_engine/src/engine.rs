//! `Engine` — the main host object, tying together winit, wgpu, scene, and input.

use std::sync::Arc;

use gd_input::InputServer;
use gd_render::Renderer;
use gd_scene::SceneTree;
use log::{error, info};
use pollster::block_on;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{input_bridge::handle_window_event, time::GameTime};

// ── EngineBuilder ─────────────────────────────────────────────────────────────

pub struct EngineBuilder {
    title:  String,
    width:  u32,
    height: u32,
}

impl EngineBuilder {
    fn new() -> Self {
        Self { title: "Godot-Rust Engine".into(), width: 1280, height: 720 }
    }

    pub fn title(mut self, t: impl Into<String>) -> Self { self.title = t.into(); self }
    pub fn size(mut self, w: u32, h: u32) -> Self { self.width = w; self.height = h; self }

    pub fn build(self) -> Engine {
        Engine { title: self.title, width: self.width, height: self.height }
    }
}

// ── Engine ────────────────────────────────────────────────────────────────────

pub struct Engine {
    title:  String,
    width:  u32,
    height: u32,
}

impl Engine {
    pub fn builder() -> EngineBuilder { EngineBuilder::new() }

    pub fn run<F>(self, setup: F)
    where
        F: FnOnce(&mut SceneTree, &mut InputServer),
    {
        env_logger::try_init().ok();

        // ── Event loop + window ───────────────────────────────────────────────
        // EventLoop::new() returns Result in winit 0.29
        let event_loop = EventLoop::new().expect("Failed to create event loop");

        let window = Arc::new(
            WindowBuilder::new()
                .with_title(&self.title)
                .with_inner_size(LogicalSize::new(self.width, self.height))
                .with_resizable(true)
                // winit 0.29: EventLoop<T> derefs to EventLoopWindowTarget<T>
                .build(&event_loop)
                .expect("Failed to create window"),
        );
        info!("Window created: {}×{}", self.width, self.height);

        // ── wgpu surface (static lifetime via Arc<Window>) ────────────────────
        let wgpu_instance = wgpu::Instance::default();
        let surface = wgpu_instance
            .create_surface(Arc::clone(&window))
            .expect("Failed to create wgpu surface");

        let init_size = {
            let s = window.inner_size();
            (s.width.max(1), s.height.max(1))
        };

        // ── Renderer + scene ──────────────────────────────────────────────────
        let mut renderer = block_on(Renderer::new(wgpu_instance, surface, init_size));
        info!("Renderer initialised");

        let mut tree  = SceneTree::new();
        let mut input = InputServer::new();

        setup(&mut tree, &mut input);
        tree.ready_all();

        let mut time = GameTime::new();

        // ── Event loop ────────────────────────────────────────────────────────
        // winit 0.29: run() takes FnMut(Event<T>, &EventLoopWindowTarget<T>)
        // Control flow is managed via elwt methods, not a &mut ControlFlow arg.
        event_loop
            .run(move |event, elwt| {
                elwt.set_control_flow(ControlFlow::Poll);

                match event {
                    // Close
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested, ..
                    } => {
                        info!("Window closed — shutting down");
                        elwt.exit();
                    }

                    // Resize
                    Event::WindowEvent {
                        event: WindowEvent::Resized(new_size), ..
                    } => {
                        renderer.resize((new_size.width.max(1), new_size.height.max(1)));
                    }

                    // Tick + render once all OS events for this frame are drained.
                    // In winit 0.29 this is `AboutToWait` (renamed from MainEventsCleared).
                    Event::AboutToWait => {
                        time.tick();

                        let events: Vec<_> = input.events.clone();
                        let draw_queue = tree.process(time.delta, &events);
                        input.flush();

                        match renderer.render(draw_queue) {
                            Ok(()) => {}
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                let sz = renderer.size;
                                renderer.resize(sz);
                            }
                            Err(e) => error!("Render error: {e}"),
                        }
                    }

                    // Forward all other window events to the input bridge
                    Event::WindowEvent { ref event, .. } => {
                        handle_window_event(&mut input, event);
                    }

                    _ => {}
                }
            })
            .expect("Event loop error");
    }
}
