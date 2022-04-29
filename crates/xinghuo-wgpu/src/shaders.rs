use wgpu::ShaderModule;

pub struct ShaderManager {
    pub shader_rectangle: ShaderModule,
}

macro_rules! load_shader {
    ($device:ident, $label:expr, $path:expr) => {
        $device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some($label),
            source: wgpu::ShaderSource::Wgsl(include_str!($path).into()),
        })
    };
}

impl ShaderManager {
    pub fn new(device: &wgpu::Device) -> Self {
        let shader_rectangle = load_shader!(device, "", "./shaders/shader_rectangle.wgsl");
        Self { shader_rectangle }
    }
}
