use wgpu::{Adapter, AdapterInfo};

pub use wgpu;
use xinghuo_core::app::GpuDeviceInfo;

pub mod error;
pub mod painter;
pub mod quad;
pub mod window;
// mod draw_pipe;
// pub mod font;
// mod shader_rectangle;
// mod shaders;

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
