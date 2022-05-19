// #![feature(derive_default_enum)]
#![feature(float_minimum_maximum)]
// #[macro_use]
// pub mod elements;
pub mod app;
pub mod context;
pub mod error;
pub mod event;
mod layer;
// pub mod macros;
pub mod draw;
pub mod id;
mod memory;
pub mod node;

use id::IdPath;
use rctree::Node;
use std::fmt::Debug;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;

pub mod prelude {
    pub use crate::draw::DrawIface;
    pub use crate::event::*;
    pub use crate::NodeBuilder;
    pub use crate::Value;

    // pub use crate::ui_element;
    pub use euclid;
    pub use lyon;
    pub use paste;
    pub use rctree;
    pub use xinghuo_macro::ui_view;

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

#[derive(Debug, Clone, Copy)]
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

impl Default for Layout {
    fn default() -> Self {
        Self::Block
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Style {
    /* 盒子模型: */
    /* 总元素的宽度 = margin-left + border-left + width + padding-left + padding-right + border-right + margin-right */
    /* 总元素的高度 = margin-top + border-top + width + padding-top + padding-bottom + border-bottom + margin-bottom */
    pub width: Size,
    pub height: Size,

    pub padding: Quat,
    pub margin: Quat,
    pub border: Quat,

    pub border_radius: Quat,
    pub border_color: Color,
    pub background_color: Color,
}

#[derive(Debug, Clone)]
pub struct DomElement {
    /// 用于标记 Dom 在 Tree 上的位置
    id_path: IdPath,
    /// 节点名称
    tag: String,
    /// 节点Id
    id: String,
    /// 节点类属
    class: Vec<String>,
    /// 子节点布局
    layout: Layout,
    /// 节点样式
    style: Style,
    /// 节点样式 changed 标志
    dirty: bool,
    /// 节点在屏幕坐标系中的位置, 宽度和高度是 有效的宽度和高度, 不是盒子的宽度和高度
    area: Box2<Size>,
    /// 父节点尺寸
    parent_size: Size2<f32>,
}

impl DomElement {
    pub fn new(tag: impl ToString, id_path: IdPath) -> Self {
        Self {
            id_path,
            id: String::new(),
            class: Vec::new(),
            tag: tag.to_string(),
            layout: Layout::default(),
            style: Style::default(),
            dirty: true,
            area: Box2::default(),
            parent_size: Size2::default(),
        }
    }

    #[inline]
    pub fn id_path(&self) -> &IdPath {
        &self.id_path
    }

    #[inline]
    pub fn tag(&self) -> &String {
        &self.tag
    }

    #[inline]
    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
        self.dirty = true;
    }

    #[inline]
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
        self.dirty = true;
    }

    #[inline]
    pub fn set_parent_size(&mut self, parent_size: Size2<f32>) {
        self.parent_size = parent_size;
        self.dirty = true;
    }

    #[inline]
    pub fn set_area(&mut self, area: Box2<Size>) {
        self.area = area;
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
    pub fn set_padding(&mut self, padding: impl Into<Quat>) {
        self.style.padding = padding.into();
        self.dirty = true;
    }

    #[inline]
    pub fn set_margin(&mut self, margin: impl Into<Quat>) {
        self.style.margin = margin.into();
        self.dirty = true;
    }

    #[inline]
    pub fn set_border(&mut self, border: impl Into<Quat>) {
        self.style.border = border.into();
        self.dirty = true;
    }

    #[inline]
    pub fn set_border_radius(&mut self, border_radius: impl Into<Quat>) {
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
    pub fn layout(&self) -> Layout {
        self.layout
    }

    #[inline]
    pub fn parent_size(&self) -> Size2<f32> {
        self.parent_size
    }

    #[inline]
    pub fn area(&self) -> Box2<Size> {
        self.area
    }

    #[inline]
    pub fn width(&self) -> Size {
        self.style.width
    }

    #[inline]
    pub fn height(&self) -> Size {
        self.style.height
    }

    pub fn edge_width(&self) -> Size {
        let margin = self.margin();
        let border = self.border();
        let padding = self.padding();
        margin.left()
            + margin.right()
            + border.left()
            + border.right()
            + padding.left()
            + padding.right()
    }

    pub fn edge_height(&self) -> Size {
        let margin = self.margin();
        let border = self.border();
        let padding = self.padding();
        margin.top()
            + margin.bottom()
            + border.top()
            + border.bottom()
            + padding.top()
            + padding.bottom()
    }

    /// 盒子 左上角坐标
    pub fn left_top(&self) -> Vector2<Size> {
        let margin = self.margin();
        let border = self.border();
        let padding = self.padding();
        vector2(
            margin.left() + border.left() + padding.left(),
            margin.top() + border.top() + padding.top(),
        )
    }

    /// 盒子 右下角坐标
    pub fn right_bottom(&self) -> Vector2<Size> {
        self.left_top() + vector2(self.width(), self.height())
        // vector2(
        //     margin.left() + border.left() + padding.left(),
        //     margin.right() + border.right() + padding.right(),
        // )
    }

    /// box坐标
    #[inline]
    pub fn box_rect(&self) -> Box2<Size> {
        let start = self.area.min - self.left_top();
        let end = start + vector2(self.box_width(), self.box_height());
        box2(start, end)
    }

    #[inline]
    pub fn box_width(&self) -> Size {
        self.width() + self.edge_width()
    }

    #[inline]
    pub fn box_height(&self) -> Size {
        self.height() + self.edge_height()
    }

    #[inline]
    pub fn padding(&self) -> Quat {
        self.style.padding
    }

    #[inline]
    pub fn margin(&self) -> Quat {
        self.style.margin
    }

    #[inline]
    pub fn border(&self) -> Quat {
        self.style.border
    }

    #[inline]
    pub fn border_radius(&self) -> Quat {
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
    Quat(f32, f32, f32, f32),
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
            Value::Quat(top, right, bottom, left) => (top, right, bottom, left),
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
        Self::Quat(v.0, v.1, v.2, v.3)
    }
}

impl From<(f32, f32)> for Value {
    fn from(v: (f32, f32)) -> Self {
        Self::Quat(v.0, v.0, v.1, v.1)
    }
}

/// 支持的格式: "10%" 表示10%比例， "10.0" 表示10.0px
/// 百分比是相对于 父节点尺寸 (DomElement.parent_size)
#[derive(Debug, Clone, Copy)]
enum InnerSize {
    Percent(f32),
    Number(f32),
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    // 参数值
    param: InnerSize,
    // 计算值
    value: f32,
}

impl Size {
    #[inline]
    pub fn zero() -> Self {
        Self::new_value(0.0)
    }

    #[inline]
    pub fn new_value(value: f32) -> Self {
        Self {
            param: InnerSize::Number(value),
            value: value,
        }
    }

    #[inline]
    pub fn new_percent(percent: f32) -> Self {
        Self {
            param: InnerSize::Percent(percent),
            value: 0.0,
        }
    }

    pub fn update(mut self, max_value: impl Into<Self>) -> Self {
        let max_value = max_value.into().value();
        let value = match self.param {
            InnerSize::Number(n) => {
                if max_value == 0.0 {
                    n
                } else {
                    max_value.minimum(n)
                }
            }
            InnerSize::Percent(p) => max_value.minimum(p * max_value),
        };
        self.value = value;
        self
    }

    #[inline]
    pub fn value(&self) -> f32 {
        self.value
    }
}

impl Default for Size {
    fn default() -> Self {
        Self {
            param: InnerSize::Number(0.0),
            value: 0.0,
        }
    }
}

impl From<&str> for Size {
    fn from(s: &str) -> Self {
        if s.ends_with("%") {
            let percent = s[..s.len() - 1].parse::<f32>().unwrap_or(0.0) / 100.0;
            Self::new_percent(percent)
        } else {
            let num = s.parse::<f32>().unwrap_or(0.0);
            Self::new_value(num)
        }
    }
}

impl From<f32> for Size {
    fn from(num: f32) -> Self {
        Self::new_value(num)
    }
}

impl Into<f32> for Size {
    fn into(self) -> f32 {
        self.value
    }
}

impl Add for Size {
    type Output = Size;
    fn add(self, other: Self) -> Self::Output {
        Self::new_value(self.value() + other.value())
    }
}

impl Sub for Size {
    type Output = Size;
    fn sub(self, other: Self) -> Self::Output {
        Self::new_value(self.value() - other.value())
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

pub trait Convert {
    fn convert_size(self) -> Box2<f32>;
}

impl Convert for Box2<Size> {
    fn convert_size(self) -> Box2<f32> {
        box2(
            point2(self.min.x.into(), self.min.y.into()),
            point2(self.max.x.into(), self.max.y.into()),
        )
    }
}

/// 表示尺寸: 上右下左
/// 支持的格式: 1个/2个/3个/4个 Size 类型的数据组成的字符串.
/// 如: "50.1% 10.0 20.0 10.0" 表示 上: 50%, 右: 10.0px, 下: 20.0px, 左: 10.0px
/// 如: "10.0 20.0" 表示 上下为: 10.0, 左右: 10.0
#[derive(Debug, Default, Clone, Copy)]
pub struct Quat(pub [Size; 4]);

impl Quat {
    pub fn update(mut self, max_size: [Size; 2]) -> Self {
        self.0[0] = self.0[0].update(max_size[0]);
        self.0[1] = self.0[1].update(max_size[1]);
        self.0[2] = self.0[2].update(max_size[0]);
        self.0[3] = self.0[3].update(max_size[1]);
        self
    }

    #[inline]
    pub fn value(&self) -> [Size; 4] {
        self.0
    }

    #[inline]
    pub fn top(&self) -> Size {
        self.0[0]
    }

    #[inline]
    pub fn right(&self) -> Size {
        self.0[1]
    }

    #[inline]
    pub fn bottom(&self) -> Size {
        self.0[2]
    }

    #[inline]
    pub fn left(&self) -> Size {
        self.0[3]
    }
}

impl From<&str> for Quat {
    fn from(s: &str) -> Self {
        // 上 右 下 左
        let mut numbers = [Size::default(); 4];
        let values = s
            .split_whitespace()
            .map(|c| Size::from(c))
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

impl From<f32> for Quat {
    fn from(n: f32) -> Self {
        let num = Size::from(n);
        Self([num; 4])
    }
}
