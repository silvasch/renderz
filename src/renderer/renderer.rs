use winit::window::Window;

use crate::{Color, RenderzError};

pub struct Renderer {
    window: Window,

    surface: wgpu::Surface,
    queue: wgpu::Queue,
    device: wgpu::Device,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    background_color: Color,
}

impl Renderer {
    pub async fn new(window: Window, background_color: Color) -> Result<Self, RenderzError> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }
            .or(Err(RenderzError::WgpuSurfaceCreationError))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(RenderzError::WgpuAdapterCreationError)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .or(Err(RenderzError::WgpuDeviceCreationError))?;

        let surface_capabilies = surface.get_capabilities(&adapter);
        let format = surface_capabilies
            .formats
            .iter()
            .copied()
            .find(|v| v.describe().srgb)
            .unwrap_or(surface_capabilies.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilies.present_modes[0],
            alpha_mode: surface_capabilies.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Ok(Self {
            window,

            surface,
            device,
            queue,
            config,
            size,

            background_color,
        })
    }

    pub fn render(&mut self) -> Result<(), RenderzError> {
        let output = match self.surface.get_current_texture() {
            Ok(output) => output,
            Err(e) => return Err(e.into()),
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.background_color.to_wgpu_color()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn reconfigure(&mut self) {
        self.resize(self.size);
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
