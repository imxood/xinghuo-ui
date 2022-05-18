#![feature(derive_default_enum)]

// #[macro_use]
// pub mod elements;
pub mod app;
pub mod context;
pub mod error;
pub mod event;
mod layer;
// pub mod macros;
pub mod id;
mod memory;
pub mod node;

use rctree::Node;
use std::fmt::Debug;

pub mod prelude {
    pub use crate::event::*;
    pub use crate::NodeBuilder;
    // pub use crate::ParentNode;
    pub use crate::Value;

    // pub use crate::ui_element;
    pub use euclid;
    pub use lyon;
    pub use xinghuo_macro::ui_view;

    pub use paste;
    pub use rctree;

    pub use xinghuo_geom as geom;

    pub use geom::*;
}

pub use euclid::default::Point2D as Point2;
pub use euclid::default::Size2D as Size2;
pub use euclid::default::Vector2D as Vector2;

pub use euclid::default::Box2D;
pub type Transform<S> = euclid::default::Transform2D<S>;
pub type Rotation<S> = euclid::default::Rotation2D<S>;
pub type Translation<S> = euclid::Translation2D<S, euclid::UnknownUnit, euclid::UnknownUnit>;
pub use euclid::default::Scale;
pub use euclid::Angle;

#[inline]
pub fn vector2<S>(x: S, y: S) -> Vector2<S> {
    Vector2::new(x, y)
}

#[inline]
pub fn point2<S>(x: S, y: S) -> Point2<S> {
    Point2::new(x, y)
}

#[inline]
pub fn size2<S>(w: S, h: S) -> Size2<S> {
    Size2::new(w, h)
}

use crate::prelude::*;

pub trait DomElementBuilder: GlobalEventHandler {
    fn id(self, id: &'static str) -> Self;
}

pub trait NodeBuilder {
    fn build(self) -> Node<DomElement>;
}

impl NodeBuilder for &str {
    fn build(self) -> Node<DomElement> {
        rctree::Node::new(DomElement::new(self))
    }
}

#[derive(Debug)]
pub enum Layout {
    /// 无法设置宽和高
    Inline,
    /// 可以设置宽和高,
    InlineBlock,
    /// 可以设置高度, 但宽为 父节点宽度的 100%
    Block,
    /// 行排列, 子级 会按照一定规则 分割 父节点宽度, 高度最大为父节点的100%
    RowFlex,
    /// 列排列, 子级 会按照一定规则 分割 父节点高度, 宽度最大为父节点的100%
    ColFlex,
}

#[derive(Debug, Default)]
pub struct Style {
    /* 盒子模型: */
    /* 总元素的宽度 = margin-left + border-left + width + padding-left + padding-right + border-right + margin-right */
    /* 总元素的高度 = margin-top + border-top + width + padding-top + padding-bottom + border-bottom + margin-bottom */
    pub width: Size,
    pub height: Size,
    pub padding: Quaternion,
    pub margin: Quaternion,

    pub border_radius: Quaternion,
    pub border_color: Color,
    pub background_color: Color,
}

#[derive(Debug)]
pub struct DomElement {
    pub tag: String,
    pub id: Option<String>,
    pub class: Option<String>,
    layout: Option<Layout>,
    style: Style,
    dirty: bool,
}

impl DomElement {
    pub fn new(tag: impl ToString) -> Self {
        Self {
            id: None,
            class: None,
            tag: tag.to_string(),
            layout: None,
            style: Style::default(),
            dirty: true,
        }
    }

    #[inline]
    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = Some(layout);
        self.dirty = true;
    }

    #[inline]
    pub fn set_width(&mut self, width: impl Into<Size>) {
        self.style.width = width.into();
        self.dirty = true;
    }

    #[inline]
    pub fn set_height(&mut self, height: impl Into<Size>) {
        self.style.height = height.into();
        self.dirty = true;
    }

    #[inline]
    pub fn set_padding(&mut self, padding: impl Into<Quaternion>) {
        self.style.padding = padding.into();
        self.dirty = true;
    }

    #[inline]
    pub fn set_margin(&mut self, margin: impl Into<Quaternion>) {
        self.style.margin = margin.into();
        self.dirty = true;
    }

    #[inline]
    pub fn set_border_radius(&mut self, border_radius: impl Into<Quaternion>) {
        self.style.border_radius = border_radius.into();
        self.dirty = true;
    }

    #[inline]
    pub fn set_border_color(&mut self, border_color: impl Into<Color>) {
        self.style.border_color = border_color.into();
        self.dirty = true;
    }

    #[inline]
    pub fn set_background_color(&mut self, background_color: impl Into<Color>) {
        self.style.background_color = background_color.into();
        self.dirty = true;
    }

    #[inline]
    pub fn width(&self) -> Size {
        self.style.width
    }

    #[inline]
    pub fn height(&self) -> Size {
        self.style.height
    }

    #[inline]
    pub fn padding(&self) -> Quaternion {
        self.style.padding
    }

    #[inline]
    pub fn margin(&self) -> Quaternion {
        self.style.margin
    }

    #[inline]
    pub fn border_radius(&self) -> Quaternion {
        self.style.border_radius
    }

    #[inline]
    pub fn border_color(&self) -> Color {
        self.style.border_color
    }

    #[inline]
    pub fn background_color(&self) -> Color {
        self.style.background_color
    }

    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    #[inline]
    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Quaternion(f32, f32, f32, f32),
    Binary(f32, f32),
    Single(f32),
}

impl Default for Value {
    fn default() -> Self {
        Self::Single(0.0)
    }
}

impl Value {
    pub fn value(&self) -> (f32, f32, f32, f32) {
        match *self {
            Value::Quaternion(top, right, bottom, left) => (top, right, bottom, left),
            Value::Binary(top_bottom, left_right) => {
                (top_bottom, left_right, top_bottom, left_right)
            }
            Value::Single(top_right_bottom_left) => (
                top_right_bottom_left,
                top_right_bottom_left,
                top_right_bottom_left,
                top_right_bottom_left,
            ),
        }
    }
}

impl From<(f32, f32, f32, f32)> for Value {
    fn from(v: (f32, f32, f32, f32)) -> Self {
        Self::Quaternion(v.0, v.1, v.2, v.3)
    }
}

impl From<(f32, f32)> for Value {
    fn from(v: (f32, f32)) -> Self {
        Self::Quaternion(v.0, v.0, v.1, v.1)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Size(pub f32);

impl From<&str> for Size {
    fn from(s: &str) -> Self {
        let n = if s.ends_with("%") {
            s[..s.len() - 1].parse::<f32>().unwrap_or(0.0) / 100.0
        } else {
            s.parse::<f32>().unwrap_or(0.0)
        };
        Self(n)
    }
}

impl From<f32> for Size {
    fn from(n: f32) -> Self {
        Self(n)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Quaternion(pub [f32; 4]);

impl From<&str> for Quaternion {
    fn from(s: &str) -> Self {
        // 上 右 下 左
        let mut numbers = [0., 0., 0., 0.];
        let values = s
            .split_whitespace()
            .map(|c| c.parse::<f32>().unwrap_or(0.0))
            .collect::<Vec<_>>();
        if values.len() == 1 {
            // 上下左右 一样
            numbers[0] = values[0];
            numbers[1] = values[0];
            numbers[2] = values[0];
            numbers[3] = values[0];
        } else if values.len() == 2 {
            // 上下 一样, 左右 一样
            numbers[0] = values[0];
            numbers[1] = values[1];
            numbers[2] = values[0];
            numbers[3] = values[1];
        } else if values.len() == 3 {
            // 上 左右 下
            numbers[0] = values[0];
            numbers[1] = values[1];
            numbers[2] = values[1];
            numbers[3] = values[2];
        } else {
            // 上 右 下 左
            numbers[0] = values[0];
            numbers[1] = values[1];
            numbers[2] = values[2];
            numbers[3] = values[3];
        }

        Self(numbers)
    }
}

impl From<f32> for Quaternion {
    fn from(n: f32) -> Self {
        Self([n, n, n, n])
    }
}

// #[derive(Debug)]
// pub enum BlockType {
//     Block,
//     InlineBlock,
//     Inline,
// }

// impl Default for BlockType {
//     fn default() -> Self {
//         Self::Block
//     }
// }

// impl From<&str> for BlockType {
//     fn from(s: &str) -> Self {
//         match s {
//             "inline" | "Inline" => Self::Inline,
//             "block" | "Block" => Self::Block,
//             "inline_block" | "inline-block" | "InlineBlock" => Self::InlineBlock,
//             _ => Self::Block,
//         }
//     }
// }
