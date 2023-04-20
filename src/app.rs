use winit::event::*;
use winit::event_loop::ControlFlow;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::renderer::{RenderObject, RenderObjectsManager};
use crate::Color;
use crate::{renderer::Renderer, RenderzError};

pub struct App {
    render_objects_manager: RenderObjectsManager,
    renderer: Renderer,
    event_loop: EventLoop<()>,
}

impl App {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> AppBuilder {
        AppBuilder::new()
    }

    pub fn run(mut self) -> Result<(), RenderzError> {
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
                    WindowEvent::Resized(physical_size) => self.renderer.resize(*physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.renderer.resize(**new_inner_size)
                    }
                    _ => {}
                },
                Event::RedrawRequested(window_id) if window_id == self.renderer.window().id() => {
                    self.render_objects_manager.update();
                    match self.renderer.render() {
                        Ok(_) => {}
                        Err(RenderzError::WgpuSurfaceLost) => self.renderer.reconfigure(),
                        Err(RenderzError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => self.renderer.window().request_redraw(),
                _ => {}
            });
    }
}

pub struct AppBuilder {
    is_resizable: bool,
    initial_size: Option<(u32, u32)>,
    background_color: Color,
    render_objects: Vec<Box<dyn RenderObject>>,
}

impl AppBuilder {
    pub(crate) fn new() -> Self {
        Self {
            is_resizable: true,
            initial_size: None,
            background_color: Color::WHITE,
            render_objects: vec![],
        }
    }

    pub fn with_render_object(mut self, render_object: Box<dyn RenderObject>) -> Self {
        self.render_objects.push(render_object);
        self
    }

    pub fn is_resizable(mut self, value: bool) -> Self {
        self.is_resizable = value;
        self
    }

    pub fn with_initial_size(mut self, initial_size: (u32, u32)) -> Self {
        self.initial_size = Some(initial_size);
        self
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn build(self) -> Result<App, RenderzError> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .build(&event_loop)
            .map_err(|_| RenderzError::WinitWindowCreationError)
            .expect("err should be handled by map_err");

        window.set_resizable(self.is_resizable);
        if let Some(initial_size) = self.initial_size {
            let initial_size: winit::dpi::PhysicalSize<u32> = initial_size.into();
            window.set_inner_size(initial_size);
        }

        let renderer = pollster::block_on(Renderer::new(window, self.background_color))?;

        Ok(App {
            renderer,
            event_loop,
            render_objects_manager: RenderObjectsManager::new(self.render_objects),
        })
    }
}
