use std::{collections::HashMap, iter};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopWindowTarget},
    window::{Window, WindowBuilder, WindowId},
};

use crate::{
    draw_pipe::DrawPipe,
    shaders::ShaderManager,
    shape::{RectShape, Shape},
};

struct ViewportDesc {
    pub window: Window,
    pub background: wgpu::Color,
    pub surface: wgpu::Surface,
}

impl ViewportDesc {
    fn new(window: Window, background: wgpu::Color, instance: &wgpu::Instance) -> Self {
        let surface = unsafe { instance.create_surface(&window) };
        Self {
            window,
            background,
            surface,
        }
    }

    fn build(self, adapter: &wgpu::Adapter, device: &wgpu::Device) -> Viewport {
        let size = self.window.inner_size();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface.get_preferred_format(adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        self.surface.configure(device, &config);

        Viewport { desc: self, config }
    }
}

struct Viewport {
    desc: ViewportDesc,
    config: wgpu::SurfaceConfiguration,
}

impl Viewport {
    fn surface_config(&self) -> wgpu::SurfaceConfiguration {
        self.config.clone()
    }

    fn window_id(&self) -> WindowId {
        self.desc.window.id()
    }

    fn resize(&mut self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.desc.surface.configure(device, &self.config);
    }

    fn get_current_texture(&mut self) -> wgpu::SurfaceTexture {
        self.desc
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture")
    }

    pub fn render(&mut self, shapes: Vec<Shape>, shared: &mut SharedState) {
        let output = self.desc.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let device = &shared.device;
        let queue = &shared.queue;
        let draw_pipe = &mut shared.draw_pipe;

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        for shape in shapes {
            match shape {
                Shape::Noop => todo!(),
                Shape::Vec(_) => todo!(),
                Shape::Circle(_) => todo!(),
                Shape::LineSegment { points, stroke } => todo!(),
                Shape::Path(_) => todo!(),
                Shape::Rect(shape) => {
                    draw_pipe.square_pipeline.add_shape(device, shape);
                    draw_pipe.square_pipeline.render(&mut encoder, &view);
                }
            }
        }

        queue.submit(iter::once(encoder.finish()));
        output.present();
    }
}

pub struct SharedState {
    pub scale_factor: f64,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub draw_pipe: DrawPipe,
}

struct EventHandle {
    viewports: HashMap<WindowId, Viewport>,
    shared: SharedState,
}

impl EventHandle {
    pub fn new(viewports: HashMap<WindowId, Viewport>, shared: SharedState) -> Self {
        Self { viewports, shared }
    }

    pub fn handle(
        &mut self,
        event: Event<CustomEvent>,
        _event_loop_window_target: &EventLoopWindowTarget<CustomEvent>,
        control_flow: &mut ControlFlow,
    ) {
        let viewports = &mut self.viewports;
        let shared = &mut self.shared;
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Recreate the swap chain with the new size
                if let Some(viewport) = viewports.get_mut(&window_id) {
                    viewport.resize(&shared.device, size);
                    // On macos the window needs to be redrawn manually after resizing
                    viewport.desc.window.request_redraw();
                }
            }
            Event::RedrawRequested(window_id) => {
                if let Some(viewport) = viewports.get_mut(&window_id) {
                    let mut shape = RectShape::default();
                    shape.rect.pos = Point2 { x: 0.0, y: 0.0 };
                    shape.rect.size = Vector2 { x: 100.0, y: 100.0 };
                    let shapes = vec![Shape::Rect(shape)];

                    viewport.render(shapes, shared);
                    // let frame = viewport.get_current_texture();
                    // let view = frame
                    //     .texture
                    //     .create_view(&wgpu::TextureViewDescriptor::default());
                    // let mut encoder = shared
                    //     .device
                    //     .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    // {
                    //     let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    //         label: None,
                    //         color_attachments: &[wgpu::RenderPassColorAttachment {
                    //             view: &view,
                    //             resolve_target: None,
                    //             ops: wgpu::Operations {
                    //                 load: wgpu::LoadOp::Clear(viewport.desc.background),
                    //                 store: true,
                    //             },
                    //         }],
                    //         depth_stencil_attachment: None,
                    //     });
                    // }

                    // shared.queue.submit(Some(encoder.finish()));
                    // frame.present();
                }
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
                ..
            } => {
                viewports.remove(&window_id);
                if viewports.is_empty() {
                    *control_flow = ControlFlow::Exit
                }
            }
            _ => {}
        }
    }
}

pub fn run_native(title: impl Into<String>) {
    let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event().build();

    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(winit::dpi::PhysicalSize::new(1024, 720))
        .build(&event_loop)
        .unwrap();

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let desc = ViewportDesc::new(
        window,
        wgpu::Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        },
        &instance,
    );

    let fut = instance.request_adapter(&wgpu::RequestAdapterOptions {
        // Request an adapter which can render to our surface
        compatible_surface: Some(&desc.surface),
        ..Default::default()
    });

    let adapter = pollster::block_on(fut).expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let fut = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::downlevel_defaults(),
        },
        None,
    );

    let (device, queue) = pollster::block_on(fut).expect("Failed to create device");

    let viewport = desc.build(&adapter, &device);
    let surface_config = viewport.surface_config();

    let scale_factor = find_scale_factor(&event_loop);
    let mut viewports = HashMap::new();
    viewports.insert(viewport.window_id(), viewport);

    // 构建 绘画管道
    let draw_pipe = DrawPipe::new(&device, &surface_config);

    let shared = SharedState {
        instance,
        adapter,
        device,
        queue,
        scale_factor,
        draw_pipe,
    };

    let mut event_handle = EventHandle::new(viewports, shared);
    event_loop.run(move |event, event_loop_window_target, control_flow| {
        event_handle.handle(event, event_loop_window_target, control_flow);
    });
}

enum CustomEvent {}

fn find_scale_factor<T>(el: &EventLoop<T>) -> f64 {
    if let Some(monitor) = el.primary_monitor() {
        return monitor.scale_factor();
    }
    if let Some(monitor) = el.available_monitors().next() {
        return monitor.scale_factor();
    }
    1.0
}
