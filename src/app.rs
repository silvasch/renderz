use winit::event::*;
use winit::event_loop::ControlFlow;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::{renderer::Renderer, RenderzError};

pub struct App {
    renderer: Renderer,
    event_loop: EventLoop<()>,
}

impl App {
    pub fn new() -> AppBuilder {
        AppBuilder::new()
    }

    pub fn run(self) -> Result<(), RenderzError> {
        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.renderer.window().id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                },
                _ => {},
            });
    }
}

pub struct AppBuilder {}

impl AppBuilder {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub fn build(self) -> App {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let renderer = Renderer::new(window);

        App {
            renderer,
            event_loop,
        }
    }
}
