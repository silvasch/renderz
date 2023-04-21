use std::time::Instant;

use winit::event::*;
use winit::event_loop::ControlFlow;
use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::Color;
use crate::{RenderObject, RenderObjectsManager};
use crate::{Renderer, RenderzError};

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
        let mut last_time = Instant::now();

        self.event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.renderer.window().id() => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        self.renderer.resize_window(*physical_size)
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.renderer.resize_window(**new_inner_size)
                    }
                    _ => {}
                },
                Event::RedrawRequested(window_id) if window_id == self.renderer.window().id() => {
                    let delta_time: f32 = (last_time.elapsed().as_nanos()) as f32 / 1000000.0;
                    self.render_objects_manager.update(delta_time);
                    last_time = Instant::now();
                    let (vertices, indices) = self
                        .render_objects_manager
                        .to_vertices(self.renderer.window_size());
                    match self.renderer.render(&vertices, &indices) {
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
    is_window_resizable: bool,
    initial_window_size: Option<(u32, u32)>,
    min_window_size: (u32, u32),
    background_color: Color,
    render_objects: Vec<Box<dyn RenderObject>>,
}

impl AppBuilder {
    pub(crate) fn new() -> Self {
        Self {
            is_window_resizable: true,
            initial_window_size: None,
            min_window_size: (800, 600),
            background_color: Color::WHITE,
            render_objects: vec![],
        }
    }

    pub fn with_render_object(mut self, render_object: Box<dyn RenderObject>) -> Self {
        self.render_objects.push(render_object);
        self
    }

    pub fn is_resizable(mut self, value: bool) -> Self {
        self.is_window_resizable = value;
        self
    }

    pub fn with_initial_window_size(mut self, initial_window_size: (u32, u32)) -> Self {
        self.initial_window_size = Some(initial_window_size);
        self
    }

    pub fn with_min_window_size(mut self, min_window_size: (u32, u32)) -> Self {
        self.min_window_size = min_window_size;
        self
    }

    pub fn with_background_color(mut self, background_color: Color) -> Self {
        self.background_color = background_color;
        self
    }

    pub fn build(self) -> Result<App, RenderzError> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_min_inner_size(winit::dpi::PhysicalSize::new(
                self.min_window_size.0,
                self.min_window_size.1,
            ))
            .build(&event_loop)
            .map_err(|_| RenderzError::WinitWindowCreationError)
            .expect("err should be handled by map_err");

        window.set_resizable(self.is_window_resizable);
        if let Some(initial_size) = self.initial_window_size {
            let initial_window_size: winit::dpi::PhysicalSize<u32> = initial_size.into();
            window.set_inner_size(initial_window_size);
        }

        let renderer = pollster::block_on(Renderer::new(window, self.background_color))?;

        Ok(App {
            renderer,
            event_loop,
            render_objects_manager: RenderObjectsManager::new(self.render_objects),
        })
    }
}
