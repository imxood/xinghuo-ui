use wgpu::ShaderModule;

pub struct Pipeline {
    device: wgpu::Device,
    queue: wgpu::Queue,
    shaders: ShaderManager,
}

impl Pipeline {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        let shaders = ShaderManager::new(&device);
        Self {
            device,
            queue,
            shaders,
        }
    }
}
