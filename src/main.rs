use wgpu::Features;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, Size},
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{self, Window, WindowButtons},
};

struct App {
    window: Option<Window>,
    size: Size,
}

impl Default for App {
    fn default() -> Self {
        App {
            window: None,

            size: LogicalSize::new(1280, 720).into(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(
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
    window: &'a Window,
}

impl<'a> State<'a> {
    async fn new(window: &'a Window) -> State<'a> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).expect(
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
            window,
        }
    }

    fn resize(&mut self, size: Size) {
        todo!()
    }

    fn input(&mut self, event: WindowEvent) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
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
