use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop);

    let wgpu_instance = wgpu::Instance::new(Backends::PRIMARY);

    let surface = unsafe { instance.create_surface(&window) };

    let adapter = wgpu_instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    }).await?;

    let (device, queue) = adapter.request_device(
        &DeviceDescriptor { label: Some("device"), features: Features::empty(), limits: Limits::default() },
        None,
    ).await?;

    let surface_configuration = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: match surface.get_preferred_format(&adapter) {
            Some(format) => format,
            None => return Err(Error::IncompatibleSurface),
        },
        width: resolution.width,
        height: resolution.height,
        present_mode: PresentMode::Mailbox,
    };
    surface.configure(&device, &surface_configuration);

    let mut gui_instance = gui::instance::Instance::new();
    let mut gui_renderer = gui_wgpu::renderer::Renderer::new();
    let mut gui_event_handler = gui_winit::event_handler::EventHandler::new();

    event_loop.run(|event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        gui_event_handler.process_event(&mut gui_instance, &event);

        match &event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => todo!(),
            Event::RedrawRequested(_) => gui_renderer.render(&gui_instance),
        }
    });

    // Code here is never ran!
}
