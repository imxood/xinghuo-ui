use crate::context::Context;

#[derive(Debug, Clone)]
pub struct GpuDeviceInfo {
    /// Adapter name
    pub name: String,
    /// Vendor PCI id of the adapter
    pub vendor: usize,
    /// PCI id of the adapter
    pub device: usize,
    /// Type of device
    pub device_type: String,
    /// Backend used for device
    pub backend: String,
}

pub trait App {
    fn name(&mut self) -> &'static str;
    fn setup(&mut self, ctx: &Context, boot_ctx: &BootContext);
    fn update(&mut self, ctx: &Context);
    fn view(&mut self, ctx: &Context);
}

/// 保存关于 window启动时 的上下文, 用于得到 window 更多的信息
pub struct BootContext {
    pub gpu_device_info: GpuDeviceInfo,
}
