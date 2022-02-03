use gui::Container;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::MouseButton;
use winit::event::WindowEvent;

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn process_event(&mut self, gui: &mut [Container], event: &Event<()>) {
        for container in gui {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::DroppedFile(path) => container.process_event(gui::Event::DroppedFile { path }),
                    WindowEvent::HoveredFile(path) => container.process_event(gui::Event::HoveredFile { path }),
                    WindowEvent::HoveredFileCancelled => container.process_event(gui::Event::HoveredFileCanceled),
                    WindowEvent::CursorMoved { position, .. } => container.process_event(gui::Event::CursorMoved { x: position.x as u32, y: position.y as u32 }),
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

                        container.process_event(gui::Event::MouseButton { button, pressed });
                    },
                    _ => (),
                },
                _ => (),
            }
        }
    }
}
