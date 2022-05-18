use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

use xinghuo_core::{
    event::*,
    id::{Id, IdPath},
    prelude::*,
    DomElement, Size,
};

#[derive(Default, Debug)]
pub struct Map {
    inner: HashMap<TypeId, Box<dyn Any>>,
    dirty: bool,
}

impl Map {
    pub fn begin(&mut self) {
        self.dirty = false;
    }

    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn get<T: Any>(&self) -> Option<&T> {
        self.inner
            .get(&TypeId::of::<T>())
            .map(|v| v.downcast_ref().unwrap())
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.inner
            .get_mut(&TypeId::of::<T>())
            .map(|v| v.downcast_mut().unwrap())
    }

    pub fn set<T: Any>(&mut self, value: T) {
        let v = self.inner.insert(TypeId::of::<T>(), Box::new(value));
        if v.is_some() {
            self.dirty = true;
        }
    }
}

impl Event {
    pub fn is_empty(&self) -> bool {
        let Self {
            onclick,
            onmouseenter,
            onmouseleave,
            onmousemove,
            onmouseout,
            onmouseover,
            onmouseup,
        } = self;
        onclick.is_none()
            && onmouseenter.is_none()
            && onmouseleave.is_none()
            && onmousemove.is_none()
            && onmouseout.is_none()
            && onmouseover.is_none()
            && onmouseup.is_none()
    }
}
macro_rules! debug_event_field {
    ($debug:ident, $($name:ident),*) => {
        $(
            if $name.is_some() {
                $debug.field(stringify!($name), &"Some");
            }
        )*
    };
}
impl Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            onclick,
            onmouseenter,
            onmouseleave,
            onmousemove,
            onmouseout,
            onmouseover,
            onmouseup,
        } = self;

        // let a = &format("{:?}", self.onclick.as_ref().map(|_| ()))[..4];
        let mut debug = f.debug_struct("Events");
        debug_event_field!(
            debug,
            onclick,
            onmouseenter,
            onmouseleave,
            onmousemove,
            onmouseout,
            onmouseover,
            onmouseup
        );
        debug.finish()
    }
}

pub trait DrawIface {
    fn rect(&mut self, rect: Rect<f32>, col: Color);
    fn text(&mut self, text: String, pos: Point2<f32>, size: f32, color: Color);
}

struct DrawDummy {}

impl DrawIface for DrawDummy {
    fn rect(&mut self, rect: Rect<f32>, col: Color) {
        println!("-- draw rect --> rect: {:?}, color: {:?}", &rect, &col);
    }

    fn text(&mut self, text: String, pos: Point2<f32>, size: f32, color: Color) {
        println!(
            "-- draw text --> text: {:?} pos: {:?} size: {:?} color: {:?}",
            &text, &pos, &size, &color
        );
    }
}

pub trait RenderObject: Debug {
    /// 计算布局, 获取到子节点, 设置子节点的形状
    fn layout(&mut self, ctx: &RenderCx) {
        if self.dom().is_dirty() {
            // 布局
            self.dom_mut().set_dirty(false);
        }
    }

    fn paint(&mut self, cx: &mut RenderCx) {
        let draw = cx.draw;
        // draw.rect(Rect::new());
    }

    fn dom(&self) -> &DomElement;

    fn dom_mut(&mut self) -> &mut DomElement;

    fn id_path(&self) -> &IdPath;
}

pub struct DataObject {
    pub id_path: IdPath,
    pub data: Box<dyn Any>,
}

pub struct EventObject {
    pub id_path: IdPath,
    pub event: Event,
}

pub struct DivBuilder {
    id_path: IdPath,
    dom: DomElement,
    event: Option<Event>,
    data: Option<Box<dyn Any>>,
}

#[derive(Default)]
pub struct Event {
    pub onclick: Option<Box<dyn FnMut(Click)>>,
    pub onmouseenter: Option<Box<dyn FnMut(MouseEnter)>>,
    pub onmouseleave: Option<Box<dyn FnMut(MouseLeave)>>,
    pub onmousemove: Option<Box<dyn FnMut(MouseMove)>>,
    pub onmouseout: Option<Box<dyn FnMut(MouseOut)>>,
    pub onmouseover: Option<Box<dyn FnMut(MouseOver)>>,
    pub onmouseup: Option<Box<dyn FnMut(MouseUp)>>,
}

impl DivBuilder {
    pub fn new(id_path: IdPath) -> Self {
        let dom = DomElement::new("div");
        Self {
            id_path,
            dom,
            event: None,
            data: None,
        }
    }

    pub fn onclick(mut self, onclick: impl FnMut(Click) + 'static) -> Self {
        if self.event.is_none() {
            self.event = Some(Event::default());
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

    pub fn data(mut self, data: impl Any) -> Self {
        let data: Box<dyn Any> = Box::new(data);
        self.data = Some(data);
        self
    }

    pub fn build(
        self,
    ) -> (
        rctree::Node<Box<dyn RenderObject>>,
        Option<rctree::Node<EventObject>>,
        Option<rctree::Node<DataObject>>,
    ) {
        let Self {
            id_path,
            dom,
            event,
            data,
        } = self;
        let render_object: Box<dyn RenderObject> = Box::new(Div {
            id_path: id_path.clone(),
            dom,
        });

        let render_node = rctree::Node::new(render_object);
        let event_node = if let Some(event) = event {
            Some(rctree::Node::new(EventObject {
                id_path: id_path.clone(),
                event,
            }))
        } else {
            None
        };
        let data_node = if let Some(data) = data {
            Some(rctree::Node::new(DataObject {
                id_path: id_path.clone(),
                data,
            }))
        } else {
            None
        };

        (render_node, event_node, data_node)
    }
}

#[derive(Debug)]
pub struct Div {
    id_path: IdPath,
    dom: DomElement,
}

impl Div {
    fn new(id_path: IdPath) -> Self {
        Self {
            id_path,
            dom: DomElement::new("div"),
        }
    }
}

impl RenderObject for Div {
    #[inline]
    fn dom(&self) -> &DomElement {
        &self.dom
    }

    #[inline]
    fn dom_mut(&mut self) -> &mut DomElement {
        &mut self.dom
    }

    #[inline]
    fn id_path(&self) -> &IdPath {
        &self.id_path
    }
}

#[derive(Default)]
pub struct RenderTree {
    inner: Option<rctree::Node<Box<dyn RenderObject>>>,
}

impl RenderTree {
    pub fn child(&mut self, node: impl RenderObject + 'static) {
        let node: Box<dyn RenderObject> = Box::new(node);
        let node = rctree::Node::new(node);
        if let Some(nodes) = self.inner.as_mut() {
            nodes.append(node);
        }
    }
}

struct Cx {
    id_path: IdPath,
}

impl Cx {
    pub fn new() -> Self {
        Self {
            id_path: Vec::new(),
        }
    }

    #[inline]
    pub fn push(&mut self, id: Id) {
        self.id_path.push(id);
    }

    #[inline]
    pub fn pop(&mut self) {
        self.id_path.pop();
    }

    #[inline]
    pub fn id_path(&self) -> &IdPath {
        &self.id_path
    }

    pub fn with_id<R, F: FnOnce(&mut Self) -> R>(&mut self, id: Id, f: F) -> R {
        self.push(id);
        let ret = f(self);
        self.pop();
        ret
    }
}

pub struct RenderCx<'a> {
    pub window_width: f32,
    pub window_height: f32,
    pub data_tree: &'a Option<rctree::Node<DataObject>>,
    pub draw: &'a Box<dyn DrawIface>,
}

pub struct App {
    pub event_tree: Option<rctree::Node<EventObject>>,
    pub data_tree: Option<rctree::Node<DataObject>>,
    pub render_tree: rctree::Node<Box<dyn RenderObject>>,
    pub draw: Box<dyn DrawIface>,
}

fn main() {
    let mut cx = Cx::new();

    let window_width = 800.0;
    let window_height = 600.0;

    fn merge_option_node<T>(
        node0: Option<rctree::Node<T>>,
        node1: Option<rctree::Node<T>>,
    ) -> Option<rctree::Node<T>> {
        if let Some(mut node0) = node0 {
            if let Some(node1) = node1 {
                node0.append(node1);
            }
            return Some(node0);
        }
        node1
    }

    // 创建根节点
    let (root_render_node, root_event_node, root_data_node) = cx.with_id(Id::next(), |cx| {
        let id_path = cx.id_path().clone();
        let (mut render_node, event_node, data_node) = DivBuilder::new(id_path)
            .width(window_width)
            .height(window_height)
            .onclick(|clicked| {
                println!("clicked: {:?}", &clicked);
            })
            .build();

        // 创建第一个 一级子节点
        let (child_render_node, child_event_node, child_data_node) = cx.with_id(Id::next(), |cx| {
            let id_path = cx.id_path().clone();
            let (mut render_node, event_node, data_node) = DivBuilder::new(id_path)
                .width(500.0)
                .height(500.0)
                .data(1)
                .build();

            // 创建第一个 二级子节点
            let (child_render_node, child_event_node, child_data_node) =
                cx.with_id(Id::next(), |cx| {
                    let id_path = cx.id_path().clone();

                    let (mut render_node, event_node, data_node) = DivBuilder::new(id_path)
                        .width(300.0)
                        .height(300.0)
                        .data(2)
                        .build();

                    // 创建第一个 三级子节点
                    let (child_render_node, child_event_node, child_data_node) =
                        cx.with_id(Id::next(), |cx| {
                            let id_path = cx.id_path().clone();
                            let (render_node, event_node, data_node) = DivBuilder::new(id_path)
                                .width(100.0)
                                .height(100.0)
                                .data(2)
                                .build();
                            (render_node, event_node, data_node)
                        });
                    render_node.append(child_render_node);
                    let event_node = merge_option_node(event_node, child_event_node);
                    let data_node = merge_option_node(data_node, child_data_node);

                    (render_node, event_node, data_node)
                });
            render_node.append(child_render_node);
            let event_node = merge_option_node(event_node, child_event_node);
            let data_node = merge_option_node(data_node, child_data_node);

            // 添加第一个 二级节点
            let (child_render_node, child_event_node, child_data_node) =
                cx.with_id(Id::next(), |cx| {
                    let id_path = cx.id_path().clone();
                    let (render_node, event_node, data_node) = DivBuilder::new(id_path)
                        .width(150.0)
                        .height(150.0)
                        .data(2)
                        .build();
                    (render_node, event_node, data_node)
                });
            render_node.append(child_render_node);
            let event_node = merge_option_node(event_node, child_event_node);
            let data_node = merge_option_node(data_node, child_data_node);
            (render_node, event_node, data_node)
        });
        render_node.append(child_render_node);
        let event_node = merge_option_node(event_node, child_event_node);
        let data_node = merge_option_node(data_node, child_data_node);
        (render_node, event_node, data_node)
    });

    let app = App {
        event_tree: root_event_node,
        data_tree: root_data_node,
        render_tree: root_render_node,
        draw: Box::new(DrawDummy {}),
    };

    let App {
        event_tree,
        data_tree,
        render_tree,
        draw,
    } = &app;

    let mut render_ctx = RenderCx {
        window_width,
        window_height,
        draw,
        data_tree,
    };
    // 遍历渲染树
    for node in render_tree.traverse() {
        match node {
            rctree::NodeEdge::Start(mut node) => {
                let mut node = node.borrow_mut();
                let dom = node.dom();
                println!(
                    "render node -- id_path: {:?} width: {:?} height: {:?}",
                    node.id_path(),
                    dom.width(),
                    dom.height()
                );
                node.layout(&mut render_ctx);
                node.paint(&mut render_ctx);
            }
            rctree::NodeEdge::End(node) => {
                let node = node.borrow();
                println!("render node end: {:?}", node.id_path());
            }
        }
    }

    // 遍历事件树
    if let Some(event_tree) = event_tree {
        for node in event_tree.traverse() {
            match node {
                rctree::NodeEdge::Start(node) => {
                    let node = node.borrow();
                    println!(
                        "event node -- id_path: {:?} event: {:?}",
                        &node.id_path, &node.event
                    );
                }
                _ => {}
            }
        }
    }

    // 遍历数据树
    if let Some(data_tree) = data_tree {
        for node in data_tree.traverse() {
            match node {
                rctree::NodeEdge::Start(node) => {
                    let node = node.borrow();
                    println!(
                        "data node -- id_path: {:?} data: {:?}",
                        &node.id_path, &node.data
                    );
                }
                _ => {}
            }
        }
    }
}
