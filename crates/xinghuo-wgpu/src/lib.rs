pub mod error;
use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Debug,
    iter,
    num::NonZeroU32,
};

use epaint::{textures::TexturesDelta, ClippedMesh, Color32, ImageData, Mesh, TextureId};
use error::BackendError;
use wgpu::{util::DeviceExt, Adapter, AdapterInfo, SurfaceError};

pub use wgpu;
use xinghuo_core::app::GpuDeviceInfo;
pub mod window;
// mod pipeline;
mod draw_pipe;
mod shader_rectangle;
mod shaders;
// mod shape;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    color: u32,
}

/// Information about the screen used for rendering.
pub struct ScreenDescriptor {
    /// Width of the window in physical pixel.
    pub physical_width: u32,
    /// Height of the window in physical pixel.
    pub physical_height: u32,
    /// HiDPI scale factor.
    pub scale_factor: f32,
}

impl ScreenDescriptor {
    fn logical_size(&self) -> (u32, u32) {
        let logical_width = self.physical_width as f32 / self.scale_factor;
        let logical_height = self.physical_height as f32 / self.scale_factor;
        (logical_width as u32, logical_height as u32)
    }
}

/// Uniform buffer used when rendering.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct UniformBuffer {
    screen_size: [f32; 2],
}

#[derive(Debug)]
struct SizedBuffer {
    buffer: wgpu::Buffer,
    size: usize,
}

struct TextureBuffer {
    vertex: SizedBuffer,
    index: SizedBuffer,
}

pub struct WgpuBackend {
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    // adapter: wgpu::Adapter,
    render_pipeline: wgpu::RenderPipeline,
    clear_color: wgpu::Color,
    uniform_buffer: SizedBuffer,
    uniform_bind_group: wgpu::BindGroup,
    textures: HashMap<TextureId, (wgpu::Texture, wgpu::BindGroup)>,
    buffers: HashMap<TextureId, TextureBuffer>,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    gpu_device_info: GpuDeviceInfo,
}

impl WgpuBackend {
    pub fn new(
        window: &impl raw_window_handle::HasRawWindowHandle,
        width: u32,
        height: u32,
    ) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let surface = unsafe { instance.create_surface(window) };

        let fut = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        });
        let adapter = pollster::block_on(fut).unwrap();

        let fut = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            // Some(&std::path::Path::new("trace")), // Trace path
            None,
        );

        let (device, queue) = pollster::block_on(fut).unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("egui_uniform_buffer"),
            contents: bytemuck::cast_slice(&[UniformBuffer {
                screen_size: [0.0, 0.0],
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let uniform_buffer = SizedBuffer {
            buffer: uniform_buffer,
            size: std::mem::size_of::<UniformBuffer>(),
        };

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("egui_uniform_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        has_dynamic_offset: false,
                        min_binding_size: None,
                        ty: wgpu::BufferBindingType::Uniform,
                    },
                    count: None,
                }],
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("egui_uniform_bind_group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer.buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("texture_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        // 创建 Shade
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/xinghuo.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: if config.format.describe().srgb {
                    "vs_main"
                } else {
                    "vs_conv_main"
                },
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Uint32],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLAMPING
                // clamp_depth: false,
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None,
        });

        let gpu_device_info = get_gpu_device_info(&adapter);

        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            clear_color: wgpu::Color {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
            textures: HashMap::new(),
            buffers: HashMap::new(),
            uniform_buffer,
            uniform_bind_group,
            texture_bind_group_layout,
            gpu_device_info,
        }
    }

    pub fn update_textures(&mut self, textures: &TexturesDelta) {
        for (texture_id, image_delta) in textures.set.iter() {
            let image_size = image_delta.image.size();
            let origin = match image_delta.pos {
                Some([x, y]) => wgpu::Origin3d {
                    x: x as u32,
                    y: y as u32,
                    z: 0,
                },
                None => wgpu::Origin3d::ZERO,
            };
            let alpha_srgb_pixels: Vec<Color32>;

            let image_data: &[u8] = match &image_delta.image {
                ImageData::Color(img) => bytemuck::cast_slice(img.pixels.as_slice()),
                ImageData::Alpha(img) => {
                    alpha_srgb_pixels = img.srgba_pixels(1.0).collect();
                    bytemuck::cast_slice(alpha_srgb_pixels.as_slice())
                }
            };

            let image_size = wgpu::Extent3d {
                width: image_size[0] as u32,
                height: image_size[1] as u32,
                depth_or_array_layers: 1,
            };

            let image_data_layout = wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * image_size.width),
                rows_per_image: None,
            };

            let label_base = match texture_id {
                TextureId::Managed(m) => format!("egui_image_{}", m),
                TextureId::User(u) => format!("egui_user_image_{}", u),
            };

            match self.textures.entry(*texture_id) {
                Entry::Occupied(mut ent) => match image_delta.pos {
                    None => {
                        let (texture, bind_group) = create_texture_and_bind_group(
                            &self.device,
                            &self.queue,
                            &label_base,
                            origin,
                            image_data,
                            image_data_layout,
                            image_size,
                            &self.texture_bind_group_layout,
                        );

                        let (texture, _) = ent.insert((texture, bind_group));
                        // texture.destroy();
                    }
                    Some(_) => {
                        let texture = &ent.get().0;
                        self.queue.write_texture(
                            wgpu::ImageCopyTexture {
                                texture,
                                mip_level: 0,
                                origin,
                                aspect: wgpu::TextureAspect::All,
                            },
                            image_data,
                            image_data_layout,
                            image_size,
                        );
                    }
                },
                Entry::Vacant(v) => {
                    let (texture, bind_group) = create_texture_and_bind_group(
                        &self.device,
                        &self.queue,
                        &label_base,
                        origin,
                        image_data,
                        image_data_layout,
                        image_size,
                        &self.texture_bind_group_layout,
                    );

                    v.insert((texture, bind_group));
                }
            }
        }
        for texture_id in textures.free.iter() {
            let (texture, _binding) = self
                .textures
                .remove(&texture_id)
                .ok_or_else(|| {
                    BackendError::InvalidTextureId(format!(
                        "Attempted to remove an unknown texture {:?}",
                        texture_id
                    ))
                })
                .unwrap();
            // texture.destroy();
        }
    }

    fn update_uniform_buffer(&mut self, screen_descriptor: &ScreenDescriptor) {
        let (logical_width, logical_height) = screen_descriptor.logical_size();
        let buffer = [UniformBuffer {
            screen_size: [logical_width as f32, logical_height as f32],
        }];
        let data = bytemuck::cast_slice(&buffer);
        self.queue
            .write_buffer(&self.uniform_buffer.buffer, 0, data);
    }

    fn update_vertex_index_buffer(&mut self, mesh: &Mesh) {
        let index_data: &[u8] = bytemuck::cast_slice(&mesh.indices);
        let vertex_data: &[u8] = bytemuck::cast_slice(&mesh.vertices);

        if !self.buffers.contains_key(&mesh.texture_id) {
            let buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("egui_index_buffer"),
                    contents: index_data,
                    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                });
            let index_buf = SizedBuffer {
                buffer,
                size: index_data.len(),
            };
            let buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("egui_vertex_buffer"),
                    contents: vertex_data,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
            let vertex_buf = SizedBuffer {
                buffer,
                size: vertex_data.len(),
            };
            let buffer = TextureBuffer {
                index: index_buf,
                vertex: vertex_buf,
            };
            self.buffers.insert(mesh.texture_id, buffer);
            return;
        }

        let buffer = self.buffers.get_mut(&mesh.texture_id).unwrap();
        let index_buffer = &mut buffer.index;
        let vertex_buffer = &mut buffer.vertex;

        if index_buffer.size != index_data.len() {
            index_buffer.size = index_data.len();
            index_buffer.buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(format!("egui_index_buffer").as_str()),
                        contents: bytemuck::cast_slice(index_data),
                        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                    });
        } else {
            self.queue.write_buffer(&index_buffer.buffer, 0, index_data);
        }

        if vertex_buffer.size != vertex_data.len() {
            vertex_buffer.size = vertex_data.len();
            vertex_buffer.buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(format!("egui_vertex_buffer").as_str()),
                        contents: bytemuck::cast_slice(vertex_data),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });
        } else {
            self.queue
                .write_buffer(&vertex_buffer.buffer, 0, vertex_data);
        }
    }

    pub fn update_buffers(&mut self, meshes: &[ClippedMesh], screen_descriptor: &ScreenDescriptor) {
        self.update_uniform_buffer(screen_descriptor);

        for ClippedMesh(_, mesh) in meshes.iter() {
            self.update_vertex_index_buffer(mesh);
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> core::result::Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.push_debug_group("egui_pass");

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);

            for (texture_id, buffer) in self.buffers.iter() {
                let texture = self.textures.get(texture_id);
                if let Some((_texture, bind_group)) = texture {
                    render_pass.set_bind_group(1, bind_group, &[]);
                }
                render_pass
                    .set_index_buffer(buffer.index.buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.set_vertex_buffer(0, buffer.vertex.buffer.slice(..));
                render_pass.draw_indexed(0..(buffer.index.size / 4) as u32, 0, 0..1);
            }

            render_pass.pop_debug_group();
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();
        Ok(())
    }

    pub fn gpu_device_info(&self) -> GpuDeviceInfo {
        self.gpu_device_info.clone()
    }
}

/// Create a texture and bind group from existing data
fn create_texture_and_bind_group(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    label_base: &str,
    origin: wgpu::Origin3d,
    image_data: &[u8],
    image_data_layout: wgpu::ImageDataLayout,
    image_size: wgpu::Extent3d,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
) -> (wgpu::Texture, wgpu::BindGroup) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(format!("{}_texture", label_base).as_str()),
        size: image_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    });

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin,
            aspect: wgpu::TextureAspect::All,
        },
        image_data,
        image_data_layout,
        image_size,
    );

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some(format!("{}_sampler", label_base).as_str()),
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(format!("{}_texture_bind_group", label_base).as_str()),
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(
                    &texture.create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    });

    (texture, bind_group)
}

pub fn get_gpu_device_info(adapter: &Adapter) -> GpuDeviceInfo {
    let AdapterInfo {
        name,
        vendor,
        device,
        device_type,
        backend,
    } = adapter.get_info();

    GpuDeviceInfo {
        name,
        vendor,
        device,
        device_type: format!("{:?}", device_type),
        backend: format!("{:?}", backend),
    }
}
