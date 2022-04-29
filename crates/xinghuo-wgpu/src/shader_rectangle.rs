use std::mem::size_of;

use wgpu::{util::DeviceExt, SurfaceConfiguration};

use crate::{
    shaders::ShaderManager,
    shape::{RectShape, Shape},
};

// #[repr(C)]
// #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// struct Vertex {
//     position: [f32; 2],
//     tex_coords: [f32; 2],
//     color: u32,
// }

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
    buffer: Option<wgpu::Buffer>,
    buffer_size: usize,
    vertexes: Vec<Vertex>,
}

impl Pipeline {
    pub fn new(
        device: &wgpu::Device,
        shaders: &ShaderManager,
        common_bindgroup_layout: &wgpu::BindGroupLayout,
        surface_config: &SurfaceConfiguration,
    ) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("rectangle render pipeline layout"),
            bind_group_layouts: &[], // common_bindgroup_layout
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("rectangle render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shaders.shader_rectangle,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shaders.shader_rectangle,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLAMPING
                // clamp_depth: false,
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
        });

        Pipeline {
            render_pipeline,
            buffer: None,
            buffer_size: 0,
            vertexes: Vec::new(),
        }
    }

    pub fn render(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        if let Some(buffer) = &self.buffer {
            tracing::info!("render rect");
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.push_debug_group("rectangle_pass");

            render_pass.set_pipeline(&self.render_pipeline);
            // render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);

            // for (texture_id, buffer) in self.buffers.iter() {
            //     let texture = self.textures.get(texture_id);
            //     if let Some((_texture, bind_group)) = texture {
            //         render_pass.set_bind_group(1, bind_group, &[]);
            //     }
            //     render_pass.set_index_buffer(buffer.index.buffer.slice(..), wgpu::IndexFormat::Uint32);
            //     render_pass.set_vertex_buffer(0, buffer.vertex.buffer.slice(..));
            //     render_pass.draw_indexed(0..(buffer.index.size / 4) as u32, 0, 0..1);
            // }

            render_pass.set_vertex_buffer(0, buffer.slice(..));
            render_pass.draw(0..3, 0..1);

            render_pass.pop_debug_group();
        }
    }

    pub fn add_shape(&mut self, device: &wgpu::Device, shape: RectShape) {
        if self.buffer.is_none() {
            let data: &[u8] = bytemuck::cast_slice(VERTICES);
            let buffer_size = data.len();
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("rect vertex buffer"),
                contents: data,
                usage: wgpu::BufferUsages::VERTEX,
            });
            self.buffer = Some(buffer);
            self.buffer_size = buffer_size;
        }
    }
}
