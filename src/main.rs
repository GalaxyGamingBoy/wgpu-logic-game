use std::sync::Arc;

use pollster::FutureExt;
use wgpu::{Features, RenderPassDescriptor};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalSize, Size},
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{self, Window, WindowButtons},
};

struct App {
    window: Option<Arc<Window>>,
    state: Option<State<'static>>,
    size: Size,
}

impl Default for App {
    fn default() -> Self {
        App {
            window: None,
            state: None,
            size: LogicalSize::new(1280, 720).into(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("[WGPU] Logic Game")
                        .with_resizable(false)
                        .with_enabled_buttons(WindowButtons::CLOSE)
                        .with_inner_size(self.size),
                )
                .expect("An error occured while creating the window"),
        );

        self.window = Some(window.clone());
        self.state = Some(State::new(window).block_on());
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if window_id != self.window.as_ref().unwrap().id() {
            return;
        }

        if self.state.as_mut().unwrap().input(event.clone()) {
            return;
        }

        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::Resized(inner_size) => self.state.as_mut().unwrap().resize(inner_size),
            WindowEvent::RedrawRequested => {
                let state = self.state.as_mut().unwrap();

                state.update();
                match state.render() {
                    Ok(_) => println!("RENDER"),
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size.to_physical(1.0)),
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => {}
        }
    }
}

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: Size,
}

impl<'a> State<'a> {
    async fn new(window: Arc<Window>) -> State<'a> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).expect(
            format!(
                "An error occured while creating a surface with the instance {:?}",
                instance
            )
            .as_str(),
        );

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect(
                format!(
                    "An error occured while requesting an adapter with the instance {:?}",
                    instance
                )
                .as_str(),
            );

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    ..Default::default()
                },
                None,
            )
            .await
            .expect(
                format!(
                    "An error occured while requesting a device from the adapter {:?}",
                    adapter
                )
                .as_str(),
            );

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|s| s.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_capabilities.alpha_modes[0],
            format: surface_format,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        };

        Self {
            size: Size::Physical(size.clone()),
            config,
            device,
            queue,
            surface,
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width <= 0 || size.height <= 0 {
            return;
        }

        self.size = size.into();
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn input(&mut self, _event: WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("WGPU Render Command Encoder"),
            });

        {
            encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("WGPU Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.2,
                            b: 0.2,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

#[pollster::main]
async fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().expect("An error occured while creating the event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();

    event_loop
        .run_app(&mut app)
        .expect("An error occured while running the app");
}
