/*
    2022.0523 -- box2 和 rect 移植自 euclid crate
*/

pub mod box2;
pub mod color;
pub mod quad;
pub mod rect;

pub use glam::Vec2;

pub use box2::Box2;
pub use color::Color;
pub use quad::Quad;
pub use rect::Rect;

/// 重新导出 glam
pub use glam;

#[inline]
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

#[inline]
pub fn point2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

#[inline]
pub fn box2(min: Vec2, max: Vec2) -> Box2 {
    Box2::new(min, max)
}

pub fn rect(p: Vec2, s: Vec2) -> Rect {
    Rect::new(p, s)
}
