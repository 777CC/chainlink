//! `Engine` — the main host object, tying together winit, wgpu, scene, and input.

use std::sync::Arc;

use cl_input::InputServer;
use cl_platform::DisplayServer;
use cl_render::Renderer;
use cl_scene::SceneTree;
use cl_servers::{AudioServer, PhysicsServer2D, RenderingServer};
use log::{error, info};
use pollster::block_on;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use crate::{input_bridge::handle_window_event, time::GameTime};

const GRAVITY: cl_core::Vec2 = cl_core::Vec2::new(0.0, 980.0);

// ── EngineBuilder ─────────────────────────────────────────────────────────────

pub struct EngineBuilder {
    title:  String,
    width:  u32,
    height: u32,
}

impl EngineBuilder {
    fn new() -> Self {
        Self { title: "Chainlink Game Engine".into(), width: 1280, height: 720 }
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
        F: FnOnce(&mut SceneTree, &mut InputServer) + 'static,
    {
        env_logger::try_init().ok();

        let event_loop = EventLoop::new().expect("Failed to create event loop");
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = EngineApp {
            title:  self.title,
            width:  self.width,
            height: self.height,
            setup:  Some(Box::new(setup)),
            state:  None,
        };

        event_loop.run_app(&mut app).expect("Event loop error");
    }
}

// ── Initialized render + scene state ─────────────────────────────────────────

struct AppState {
    window:           Arc<Window>,
    renderer:         Renderer,
    rendering_server: RenderingServer,
    physics_server:   PhysicsServer2D,
    audio_server:     AudioServer,
    display_server:   DisplayServer,
    tree:             SceneTree,
    input:            InputServer,
    time:             GameTime,
}

// ── ApplicationHandler ────────────────────────────────────────────────────────

struct EngineApp {
    title:  String,
    width:  u32,
    height: u32,
    setup:  Option<Box<dyn FnOnce(&mut SceneTree, &mut InputServer)>>,
    state:  Option<AppState>,
}

impl ApplicationHandler for EngineApp {
    /// Called once the OS is ready (after app launch on macOS, immediately on
    /// desktop Linux/Windows).  Create the window and all GPU/engine state here.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(self.width, self.height))
            .with_resizable(true);

        let window = Arc::new(
            event_loop.create_window(attrs).expect("Failed to create window"),
        );
        info!("Window created: {}×{}", self.width, self.height);

        let wgpu_instance = wgpu::Instance::default();
        let surface = wgpu_instance
            .create_surface(Arc::clone(&window))
            .expect("Failed to create wgpu surface");

        let init_size = {
            let s = window.inner_size();
            (s.width.max(1), s.height.max(1))
        };

        let renderer = block_on(Renderer::new(wgpu_instance, surface, init_size));
        info!("Renderer initialised");

        let mut rendering_server = RenderingServer::new();
        let mut physics_server   = PhysicsServer2D::new();
        let mut audio_server     = AudioServer::new();
        let display_server       = DisplayServer::new(self.width, self.height, &self.title);

        let mut tree  = SceneTree::new();
        let mut input = InputServer::new();

        if let Some(setup) = self.setup.take() {
            setup(&mut tree, &mut input);
        }

        tree.ready_all(
            &mut rendering_server,
            &mut physics_server,
            &mut audio_server,
            &display_server,
            &input,
        );

        self.state = Some(AppState {
            window,
            renderer,
            rendering_server,
            physics_server,
            audio_server,
            display_server,
            tree,
            input,
            time: GameTime::new(),
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = self.state.as_mut() else { return };

        match &event {
            WindowEvent::CloseRequested => {
                info!("Window closed — shutting down");
                event_loop.exit();
            }

            WindowEvent::Resized(new_size) => {
                let sz = (new_size.width.max(1), new_size.height.max(1));
                state.renderer.resize(sz);
                state.display_server.window_size = sz;
            }

            WindowEvent::RedrawRequested => {
                state.time.tick();

                let events: Vec<_> = state.input.events.clone();
                let draw_queue = state.tree.process(
                    state.time.delta,
                    &events,
                    &mut state.rendering_server,
                    &mut state.physics_server,
                    &mut state.audio_server,
                    &state.display_server,
                    &state.input,
                );
                state.input.flush();

                state.physics_server.step(state.time.delta as f32, GRAVITY);

                match state.renderer.render(draw_queue) {
                    Ok(()) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let sz = state.renderer.size;
                        state.renderer.resize(sz);
                    }
                    Err(e) => error!("Render error: {e}"),
                }
            }

            other => handle_window_event(&mut state.input, other),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = self.state.as_ref() {
            state.window.request_redraw();
        }
    }
}
