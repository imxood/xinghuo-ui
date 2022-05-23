use std::{any::Any, fmt::Debug};

use geom::color::Color;

use crate::{
    event::{Click, EventListener},
    id::Id,
    prelude::*,
    Convert, DomElement, Layout, Size,
};

pub struct LayoutCx {
    pub area: Box2,
    pub cursor: Box2,
}

pub struct RenderCx<'a> {
    pub window_width: f32,
    pub window_height: f32,
    pub data_tree: &'a mut Option<TreeNode<DataObject>>,
    pub painter: &'a mut Box<dyn Painter>,
}

pub struct DataObject {
    pub node: TreeNode<Box<dyn RenderObject>>,
    pub data: Box<dyn Any>,
}

pub struct EventObject {
    pub node: TreeNode<Box<dyn RenderObject>>,
    pub event: EventListener,
}

pub trait RenderObject: Debug {
    /// 根据当前节点的布局, 计算子节点布局,
    fn layout(&mut self, parent: &DomElement) {
        let dom = self.dom_mut();
        if dom.is_dirty() {
            // 布局
            match parent.layout() {
                Layout::Inline => {}
                Layout::InlineBlock => {}
                Layout::Block => {}
                Layout::RowFlex => todo!(),
                Layout::ColFlex => todo!(),
            }
            dom.set_dirty(false);
        }
    }

    fn paint(&mut self, painter: &mut Box<dyn Painter>) {
        let dom = self.dom();
        let rect = &[Quad::new(dom.ava_box().to_rect(), dom.background_color())];
        println!("paint rect {:?}", rect);
        painter.rect(rect);
    }

    fn dom(&self) -> &DomElement;

    fn dom_mut(&mut self) -> &mut DomElement;

    fn node_id(&self) -> Id;
}

pub struct Element {
    pub dom: DomElement,
    pub event: Option<EventListener>,
    pub data: Option<Box<dyn Any>>,
    pub children: Vec<Element>,
}

impl Element {
    pub fn new(tag: impl ToString) -> Self {
        let dom = DomElement::new(tag);
        Self {
            dom,
            event: None,
            data: None,
            children: Vec::new(),
        }
    }

    pub fn onclick(mut self, onclick: impl FnMut(Click) + 'static) -> Self {
        if self.event.is_none() {
            self.event = Some(EventListener::default());
        }
        if let Some(event) = self.event.as_mut() {
            event.onclick = Some(Box::new(onclick));
        }
        self
    }

    pub fn width(mut self, width: impl Into<Size>) -> Self {
        self.dom.set_width(width);
        self
    }

    pub fn height(mut self, height: impl Into<Size>) -> Self {
        self.dom.set_height(height);
        self
    }

    pub fn background_color(mut self, background_color: impl Into<Color>) -> Self {
        self.dom.set_background_color(background_color);
        self
    }

    pub fn data(mut self, data: impl Any) -> Self {
        let data: Box<dyn Any> = Box::new(data);
        self.data = Some(data);
        self
    }

    pub fn child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, mut children: Vec<Self>) -> Self {
        self.children.append(&mut children);
        self
    }

    pub fn build(
        self,
    ) -> (
        TreeNode<Box<dyn RenderObject>>,
        Option<TreeNode<EventObject>>,
        Option<TreeNode<DataObject>>,
    ) {
        let Self {
            dom,
            event,
            data,
            children,
        } = self;

        // let node_id = dom.node_id();

        // 渲染节点
        let render_object: Box<dyn RenderObject> = Box::new(Node { dom });
        let mut render_node = TreeNode::new(render_object);

        // 事件节点
        let mut event_node = if let Some(event) = event {
            Some(TreeNode::new(EventObject {
                node: render_node.clone(),
                event,
            }))
        } else {
            None
        };

        // 数据节点
        let mut data_node = if let Some(data) = data {
            Some(TreeNode::new(DataObject {
                node: render_node.clone(),
                data,
            }))
        } else {
            None
        };

        // build 子节点
        for child in children {
            let (child_render_node, child_event_node, child_data_node) = child.build();

            render_node.append(child_render_node);

            if let Some(event_node) = &mut event_node {
                if let Some(child_event_node) = child_event_node {
                    event_node.append(child_event_node);
                }
            } else {
                event_node = child_event_node;
            }
            if let Some(data_node) = &mut data_node {
                if let Some(child_data_node) = child_data_node {
                    data_node.append(child_data_node);
                }
            } else {
                data_node = child_data_node;
            }
        }

        (render_node, event_node, data_node)
    }
}

#[derive(Debug)]
pub struct Node {
    dom: DomElement,
}

impl Node {
    fn new() -> Self {
        Self {
            dom: DomElement::new("div"),
        }
    }
}

impl RenderObject for Node {
    #[inline]
    fn dom(&self) -> &DomElement {
        &self.dom
    }

    #[inline]
    fn dom_mut(&mut self) -> &mut DomElement {
        &mut self.dom
    }

    #[inline]
    fn node_id(&self) -> Id {
        self.dom.node_id()
    }
}

#[derive(Default)]
pub struct RenderTree {
    inner: Option<TreeNode<Box<dyn RenderObject>>>,
}

impl RenderTree {
    pub fn child(&mut self, node: impl RenderObject + 'static) {
        let node: Box<dyn RenderObject> = Box::new(node);
        let node = TreeNode::new(node);
        if let Some(nodes) = self.inner.as_mut() {
            nodes.append(node);
        }
    }
}
