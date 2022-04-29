use wgpu::BindGroupLayout;

use crate::{shader_rectangle, shaders::ShaderManager};

pub struct DrawPipe {
    pub square_pipeline: shader_rectangle::Pipeline,
    shaders: ShaderManager,
    common_bindgroup_layout: wgpu::BindGroupLayout,
}
impl DrawPipe {
    pub fn new(device: &wgpu::Device, surface_config: &wgpu::SurfaceConfiguration) -> Self {
        // 加载着色器
        let shaders = ShaderManager::new(&device);

        // 通用绑定组布局
        let common_bindgroup_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("common bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        // 绘制矩形
        let square_pipeline = shader_rectangle::Pipeline::new(
            device,
            &shaders,
            &common_bindgroup_layout,
            surface_config,
        );
        Self {
            square_pipeline,
            shaders,
            common_bindgroup_layout,
        }
    }
}
