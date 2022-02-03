use std::path::PathBuf;
use std::str::FromStr;

use gui::Container;
use gui::Layout;
use gui::LayoutPadding;
use gui::Size;
use gui::View;
use gui::Widget;
use wgpu::Backends;
use wgpu::CommandEncoderDescriptor;
use wgpu::DeviceDescriptor;
use wgpu::Features;
use wgpu::Instance;
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
    let mut resolution = PhysicalSize::new(1280, 720);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("example")
        .with_inner_size(resolution)
        .build(&event_loop)?;
    let mut scale_factor = window.scale_factor();

    let instance = Instance::new(Backends::PRIMARY);

    let surface = unsafe { instance.create_surface(&window) };

    let adapter = instance.request_adapter(&RequestAdapterOptions {
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

    let mut surface_configuration = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(&adapter).unwrap(),
        width: resolution.width,
        height: resolution.height,
        present_mode: PresentMode::Mailbox,
    };
    surface.configure(&device, &surface_configuration);

    let gui = vec![
        Container {
            screen_position: [0.0, 0.0],
            pixel_position: [0, 0],
            pivot: [0.0, 0.0],
            view: View::Simple {
                width: Size::Dynamic(1.0),
                height: Size::Fixed(16),
                color: [255, 255, 255, 255],
            },
            layout: Layout::Horizontal {
                widgets: vec![],
                padding: LayoutPadding::Static(0),
            },
        },
        Container {
            screen_position: [0.0, 0.0],
            pixel_position: [16, 32],
            pivot: [0.0, 0.0],
            view: View::default(),
            layout: Layout::Free(Widget::Text { label: "text".into(), text: "Hello world!".into() }),
        },
        Container {
            screen_position: [0.1, 0.1],
            pixel_position: [0, 0],
            pivot: [0.0, 0.0],
            view: View::Texture {
                path: PathBuf::from_str("./generic_54.png").unwrap(),
                color: Some([255, 255, 255, 200]),
            },
            layout: Layout::Free(Widget::Text { label: "text2".into(), text: "Hello other worlds!".into() }),
        },
    ]; 

    let mut gui_renderer = gui_wgpu::Renderer::from_gui(&device, &queue, surface_configuration.format, resolution.into(), scale_factor, &gui).unwrap();
    let gui_event_handler = gui_winit::EventHandler::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // TODO: add back
        //gui_event_handler.process_event(&mut gui, &event);

        match &event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ScaleFactorChanged { scale_factor: sf, new_inner_size: size } => {
                    if size.width == 0 || size.height == 0 { return; }
                    scale_factor = *sf;
                    resolution = **size;
                    surface_configuration.width = resolution.width;
                    surface_configuration.height = resolution.height;
                    surface.configure(&device, &surface_configuration);
                    gui_renderer.resize(&queue, resolution.into(), scale_factor, &gui);
                },
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    if size.width == 0 || size.height == 0 { return; }
                    resolution = *size;
                    surface_configuration.width = resolution.width;
                    surface_configuration.height = resolution.height;
                    surface.configure(&device, &surface_configuration);
                    gui_renderer.resize(&queue, resolution.into(), scale_factor, &gui);
                },
                _ => (),
            },
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
}
