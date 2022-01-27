use gui::Widget;
use wgpu::Backends;
use wgpu::CommandEncoderDescriptor;
use wgpu::DeviceDescriptor;
use wgpu::Features;
use wgpu::Limits;
use wgpu::PowerPreference;
use wgpu::PresentMode;
use wgpu::RequestAdapterOptions;
use wgpu::SurfaceConfiguration;
use wgpu::SurfaceError;
use wgpu::TextureUsages;
use wgpu::TextureViewDescriptor;
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new();
    let resolution = PhysicalSize::new(1280, 720);
    let window = WindowBuilder::new()
        .with_title("example")
        .with_inner_size(resolution)
        .build(&event_loop)?;

    let wgpu_instance = wgpu::Instance::new(Backends::PRIMARY);

    let surface = unsafe { wgpu_instance.create_surface(&window) };

    let adapter = wgpu_instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    }).await.unwrap();

    let (device, queue) = adapter.request_device(
        &DeviceDescriptor {
            label: Some("device"),
            features: Features::empty(),
            limits: Limits::default(),
        },
        None,
    ).await?;

    let surface_configuration = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(&adapter).unwrap(),
        width: resolution.width,
        height: resolution.height,
        present_mode: PresentMode::Mailbox,
    };
    surface.configure(&device, &surface_configuration);

    let mut gui = gui::Instance::new();
    gui.add_widget(Widget::Window {
        label: "window".to_string(),
        widgets: vec![],
    });

    let gui_renderer = gui_wgpu::Renderer::new(&device, surface_configuration.format);
    let gui_event_handler = gui_winit::EventHandler::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // TODO: add back
        //gui_event_handler.process_event(&mut gui, &event);

        match &event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                let surface = match surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(e) => match e {
                        SurfaceError::Timeout => return,
                        SurfaceError::Outdated => return,
                        _ => panic!(),
                    },
                };

                let surface_view = surface.texture.create_view(&TextureViewDescriptor::default());

                let mut command_encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());

                gui_renderer.render(&mut command_encoder, &surface_view, &gui);
                queue.submit([command_encoder.finish()]);
                surface.present();
            },
            _ => (),
        }
    });

    // Code here is never ran!
}
