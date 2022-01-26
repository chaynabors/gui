use gui::Command;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::MouseButton;
use winit::event::WindowEvent;

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn process_event(&mut self, instance: &mut gui::Instance, event: &Event<()>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => instance.process_command(Command::Resized { width: size.width, height: size.height }),
                WindowEvent::DroppedFile(path) => instance.process_command(Command::DroppedFile { path }),
                WindowEvent::HoveredFile(path) => instance.process_command(Command::HoveredFile { path }),
                WindowEvent::HoveredFileCancelled => instance.process_command(Command::HoveredFileCanceled),
                WindowEvent::CursorMoved { position, .. } => instance.process_command(Command::CursorMoved { x: position.x as u32, y: position.y as u32 }),
                WindowEvent::MouseInput { state, button, .. } => {
                    let button = match button {
                        MouseButton::Left => 0,
                        MouseButton::Right => 1,
                        MouseButton::Middle => 2,
                        MouseButton::Other(n) => *n,
                    };

                    let pressed = match state {
                        ElementState::Pressed => true,
                        ElementState::Released => false,
                    };

                    instance.process_command(Command::MouseButton { button, pressed });
                },
                WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {
                    instance.process_command(Command::ScaleFactorChanged { scale_factor: *scale_factor });
                    instance.process_command(Command::Resized { width: new_inner_size.width, height: new_inner_size.height });
                },
                _ => (),
            },
            Event::RedrawRequested(_) => instance.process_command(Command::Redraw),
            _ => (),
        }
    }
}
