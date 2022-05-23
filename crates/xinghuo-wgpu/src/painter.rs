use winit::window::Window;
use xinghuo_core::prelude::*;

use crate::quad;

pub struct WindowPainter {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: [f32; 2],
    /// 绘制四边形
    quad_pipeline: quad::Pipeline,
}

impl Painter for WindowPainter {
    fn rect(&mut self, rect: &[Quad]) {
        if let Ok(output) = self.surface.get_current_texture() {
            self.quad_pipeline
                .render(&self.device, output, &self.queue, rect)
                .unwrap();
        }
    }

    fn text(&mut self, text: String, pos: Vec2, size: f32, color: Color) {
        todo!()
    }

    fn resize(&mut self, new_size: [f32; 2]) {
        if new_size[0] > 0.0 && new_size[1] > 0.0 {
            self.size = new_size;
            self.config.width = new_size[0] as u32;
            self.config.height = new_size[1] as u32;
            self.surface.configure(&self.device, &self.config);
            self.quad_pipeline
                .resize(&self.device, &self.queue, &self.config, &new_size);
        }
    }

    fn render(&mut self) {}

    fn size(&self) -> [f32; 2] {
        self.size
    }
}

impl WindowPainter {
    pub fn new(window: &Window) -> Self {
        pollster::block_on(async {
            let size = window.inner_size();

            // The instance is a handle to our GPU
            let instance = wgpu::Instance::new(wgpu::Backends::all());
            for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
                tracing::info!("adapter info: {:?}", adapter.get_info());
            }
            let surface = unsafe { instance.create_surface(window) };
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .unwrap();

            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                    },
                    // Some(&std::path::Path::new("trace")), // Trace path
                    None,
                )
                .await
                .unwrap();

            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_preferred_format(&adapter).unwrap(),
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::Fifo,
            };
            surface.configure(&device, &config);

            let quad_pipeline = quad::Pipeline::new(&device, &queue, config.clone(), size);

            Self {
                surface,
                device,
                queue,
                config,
                size: [size.width as f32, size.height as f32],
                quad_pipeline,
            }
        })
    }

    // fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    //     let output = self.surface.get_current_texture()?;
    //     self.quad_pipeline.render(&self.device, output, &self.queue)
    // }
}
