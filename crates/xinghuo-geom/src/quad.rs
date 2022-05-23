use crate::{Color, Rect};

/// A colored rectangle with a border.
///
/// This type can be directly uploaded to GPU memory.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Quad {
    pub rect: Rect,
    pub color: [f32; 4],
    pub border_color: [f32; 4],
    pub border_radius: f32,
    pub border_width: f32,
}

impl Quad {
    pub fn new(rect: Rect, color: Color) -> Self {
        Self {
            rect,
            color: color.to_value(),
            border_color: Default::default(),
            border_radius: Default::default(),
            border_width: Default::default(),
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for Quad {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for Quad {}
