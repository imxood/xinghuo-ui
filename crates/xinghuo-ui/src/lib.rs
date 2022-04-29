#![feature(derive_default_enum)]

#[macro_use]
pub mod elements;
pub mod event;
pub mod macros;
pub mod node;
mod memory;
pub mod context;
pub mod app;
mod layer;

use rctree::Node;
use std::{
    any::Any,
    cell::{Ref, RefMut},
    fmt::Debug,
};

pub mod prelude {
    pub use crate::event::*;
    pub use crate::NodeBuilder;
    pub use crate::ParentNode;
    pub use crate::Value;

    pub use crate::ui_element;
    pub use xinghuo_macro::ui_view;
}

use crate::prelude::*;

pub trait DomElementBuilder: GlobalEventHandler {
    fn id(self, id: &'static str) -> Self;
}

pub trait ParentNode {
    fn child<T: NodeBuilder>(self, node: T) -> Self;
}

pub trait DomNode {
    fn dom_ref(&self) -> Ref<DomElement>;
    fn dom_mut(&mut self) -> RefMut<DomElement>;
    fn node_ref(&self) -> &Node<DomElement>;
    fn node_mut(&mut self) -> &mut Node<DomElement>;
}

pub trait NodeBuilder {
    fn build(self) -> Node<DomElement>;
}

#[derive(Debug, Default)]
pub struct Style {
    /* 盒子模型: */
    /* 总元素的宽度 = margin-left + border-left + width + padding-left + padding-right + border-right + margin-right */
    /* 总元素的高度 = margin-top + border-top + width + padding-top + padding-bottom + border-bottom + margin-bottom */
    pub width: f32,
    pub height: f32,
    pub padding: Value,
    pub margin: Value,
    pub border: Value,

    pub border_color: u32,
    pub background_color: u32,
}

#[derive(Default)]
pub struct Events {
    pub onclick: Option<Box<dyn FnMut(Click)>>,
    pub onmouseenter: Option<Box<dyn FnMut(MouseEnter)>>,
    pub onmouseleave: Option<Box<dyn FnMut(MouseLeave)>>,
    pub onmousemove: Option<Box<dyn FnMut(MouseMove)>>,
    pub onmouseout: Option<Box<dyn FnMut(MouseOut)>>,
    pub onmouseover: Option<Box<dyn FnMut(MouseOver)>>,
    pub onmouseup: Option<Box<dyn FnMut(MouseUp)>>,
}

impl Debug for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn has_some<T: Any>(v: &Option<T>) -> &str {
            match v {
                Some(_) => "Some",
                None => "None",
            }
        }
        // let a = &format("{:?}", self.onclick.as_ref().map(|_| ()))[..4];
        f.debug_struct("Events")
            .field("onclick", &has_some(&self.onclick))
            .field("onmouseenter", &has_some(&self.onmouseenter))
            .field("onmouseleave", &has_some(&self.onmouseleave))
            .field("onmousemove", &has_some(&self.onmousemove))
            .field("onmouseout", &has_some(&self.onmouseout))
            .field("onmouseover", &has_some(&self.onmouseover))
            .field("onmouseup", &has_some(&self.onmouseup))
            .finish()
    }
}

#[derive(Debug)]
pub struct DomElement {
    pub id: &'static str,
    pub node_id: u64,
    pub tag: String,
    pub style: Style,
    pub events: Events,
}

impl DomElement {
    pub fn new(tag: impl ToString) -> Self {
        // let node_id = get_memory_mut_or
        Self {
            id: "",
            node_id: 0,
            tag: tag.to_string(),
            style: Default::default(),
            events: Default::default(),
        }
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
