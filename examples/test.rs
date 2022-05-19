use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

use xinghuo_core::{
    draw::DrawDummy,
    event::*,
    id::{Id, IdPath},
    prelude::{euclid::vec2, *},
    Convert, DomElement, Layout, Size, Style,
};

use rctree::Node as TreeNode;
use rctree::NodeEdge as TreeNodeEdge;

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

pub trait RenderObject: Debug {
    /// 根据当前节点的布局, 计算子节点样式
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

    fn paint(&mut self, cx: &mut RenderCx) {
        let dom = self.dom();
        cx.draw.rect(dom.area().convert_size(), Color::YELLOW);
        cx.draw.rect(dom.box_rect().convert_size(), Color::RED);
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

pub struct NodeBuilder {
    pub dom: DomElement,
    pub event: Option<Event>,
    pub data: Option<Box<dyn Any>>,
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

impl NodeBuilder {
    pub fn new(id_path: IdPath, tag: impl ToString) -> Self {
        let dom = DomElement::new(tag, id_path);
        Self {
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
        TreeNode<Box<dyn RenderObject>>,
        Option<TreeNode<EventObject>>,
        Option<TreeNode<DataObject>>,
    ) {
        let Self { dom, event, data } = self;
        let id_path = dom.id_path().clone();

        let render_object: Box<dyn RenderObject> = Box::new(Node { dom });

        let render_node = TreeNode::new(render_object);
        let event_node = if let Some(event) = event {
            Some(TreeNode::new(EventObject {
                id_path: id_path.clone(),
                event,
            }))
        } else {
            None
        };
        let data_node = if let Some(data) = data {
            Some(TreeNode::new(DataObject {
                id_path: id_path.clone(),
                data,
            }))
        } else {
            None
        };

        (render_node, event_node, data_node)
    }
}

fn div(id_path: IdPath) -> NodeBuilder {
    NodeBuilder::new(id_path, "div")
}

fn span(id_path: IdPath) -> NodeBuilder {
    let mut span = NodeBuilder::new(id_path, "span");
    span.dom.set_layout(Layout::Inline);
    span
}

#[derive(Debug)]
pub struct Node {
    dom: DomElement,
}

impl Node {
    fn new(id_path: IdPath) -> Self {
        Self {
            dom: DomElement::new("div", id_path),
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
    fn id_path(&self) -> &IdPath {
        self.dom.id_path()
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

pub struct LayoutCx {
    pub area: Box2<f32>,
    pub cursor: Box2<f32>,
}

pub struct RenderCx<'a> {
    pub window_width: f32,
    pub window_height: f32,
    pub data_tree: &'a mut Option<TreeNode<DataObject>>,
    pub draw: &'a mut Box<dyn DrawIface>,
}

pub struct App {
    pub event_tree: Option<TreeNode<EventObject>>,
    pub data_tree: Option<TreeNode<DataObject>>,
    pub render_tree: TreeNode<Box<dyn RenderObject>>,
    pub draw: Box<dyn DrawIface>,
}

impl App {
    pub fn run(&mut self) {}
}

pub struct AppBuilder {
    builder: NodeBuilder,
    draw: Box<dyn DrawIface>,
}

impl AppBuilder {
    pub fn new(builder: NodeBuilder) -> Self {
        Self {
            builder,
            draw: Box::new(DrawDummy::default()),
        }
    }
    pub fn with_draw(mut self, draw: impl DrawIface + 'static) -> Self {
        self.draw = Box::new(draw);
        self
    }
    pub fn build(self) -> App {
        let Self { builder, draw } = self;
        let (render_tree, event_tree, data_tree) = builder.build();
        App {
            event_tree,
            data_tree,
            render_tree,
            draw,
        }
    }
}

fn main() {
    let mut cx = Cx::new();

    let window_width = 800.0;
    let window_height = 600.0;

    fn merge_option_node<T>(
        node0: Option<TreeNode<T>>,
        node1: Option<TreeNode<T>>,
    ) -> Option<TreeNode<T>> {
        if let Some(mut node0) = node0 {
            if let Some(node1) = node1 {
                node0.append(node1);
            }
            return Some(node0);
        }
        node1
    }

    /*
        创建节点树: 渲染对象树, 事件树, 数据树
    */

    let (mut render_tree, mut event_tree, mut data_tree) = cx.with_id(Id::next(), |cx| {
        let id_path = cx.id_path().clone();
        let (mut render_node, event_node, data_node) = div(id_path)
            .width(window_width)
            .height(window_height)
            .onclick(|clicked| {
                println!("clicked: {:?}", &clicked);
            })
            .build();

        // 创建第一个 一级子节点
        let (child_render_node, child_event_node, child_data_node) = cx.with_id(Id::next(), |cx| {
            let id_path = cx.id_path().clone();
            let (mut render_node, event_node, data_node) =
                div(id_path).width(500.0).height(500.0).data(1).build();

            // 创建第一个 二级子节点
            let (child_render_node, child_event_node, child_data_node) =
                cx.with_id(Id::next(), |cx| {
                    let id_path = cx.id_path().clone();

                    let (mut render_node, event_node, data_node) =
                        div(id_path).width(300.0).height(300.0).data(2).build();

                    // 创建第一个 三级子节点
                    let (child_render_node, child_event_node, child_data_node) =
                        cx.with_id(Id::next(), |cx| {
                            let id_path = cx.id_path().clone();
                            let (render_node, event_node, data_node) =
                                span(id_path).width(100.0).height(100.0).data(2).build();
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
                    let (render_node, event_node, data_node) =
                        div(id_path).width(150.0).height(150.0).data(2).build();
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

    let mut draw: Box<dyn DrawIface> = Box::new(DrawDummy {});

    let mut render_ctx = RenderCx {
        window_width,
        window_height,
        draw: &mut draw,
        data_tree: &mut data_tree,
    };

    /*
        根节点布局, 强制到 屏幕尺寸, 且 块布局
    */
    {
        let mut root_node = render_tree.borrow_mut();
        let dom = root_node.dom_mut();
        dom.set_style(Style::default());
        dom.set_layout(Layout::Block);
        dom.set_width(window_width);
        dom.set_height(window_height);
        dom.set_area(Box2::new(
            point2(Size::new_value(0.0), Size::new_value(0.0)),
            point2(
                Size::new_value(window_width),
                Size::new_value(window_height),
            ),
        ));
    }

    /*
        执行布局
    */

    // 遍历渲染树
    // let cursor = ;
    // let mut layout_cx = LayoutCx {
    //     area: box2(point2(0., 0.), point2(window_width, window_height)),
    //     cursor: Box2::new(point2(0., 0.), point2(window_width, window_height)),
    // };

    // 在一个布局中, Cursor移动, 用于记录下一个 布局内节点 的起始位置, 布局结束后, Cursor会移动到下一个布局的开头
    // let mut cursor = Box2::new(
    //     point2(Size::zero(), Size::zero()),
    //     point2(
    //         Size::new_value(window_width),
    //         Size::new_value(window_height),
    //     ),
    // );
    for node in render_tree.traverse() {
        match node {
            TreeNodeEdge::Start(mut node) => {
                /*
                    根据当前节点多的布局, 计算并更新 所有子节点的布局
                */
                let dom = node.borrow().dom().clone();
                let parent_size = size2(dom.width(), dom.height());

                // 在一个布局中, Cursor移动, 用于记录下一个 布局内节点 的起始位置, 初始位置是 父节点的area
                let mut cursor = dom.area();

                match dom.layout() {
                    Layout::Inline => {}
                    Layout::InlineBlock => {}
                    Layout::Block => {
                        // 当前节点是Block, 则子节点的最大宽度和最大高度是确定的: 更新子节点的 宽度/高度/Area/父节点尺寸
                        for mut child in node.children() {
                            let mut child = child.borrow_mut();
                            let cdom = child.dom_mut();

                            // 更新盒子轮廓
                            cdom.set_margin(cdom.margin().update(parent_size.into()));
                            cdom.set_padding(cdom.padding().update(parent_size.into()));
                            cdom.set_border(cdom.border().update(parent_size.into()));

                            // 当前节点的宽度 = 父节点的宽度 - 当前节点的边沿宽度
                            cdom.set_width(parent_size.width - cdom.edge_width());
                            // 当前节点的高度 自定义的
                            cdom.set_height(cdom.height().update(parent_size.height));

                            // 更新有效区域
                            let start_point = cursor.min + cdom.left_top();
                            let end_point = start_point + vector2(cdom.width(), cdom.height());
                            cdom.set_area(box2(start_point, end_point));

                            /*
                                更新Cursor
                            */
                            // cursor向下移动
                            cursor.min.y += cdom.box_height();
                        }
                    }
                    Layout::RowFlex => {}
                    Layout::ColFlex => {}
                }

                let mut parent = node.parent();
                if parent.is_none() {
                    continue;
                }
                let parent_node = parent.as_mut().unwrap().borrow_mut();
                let mut node = node.borrow_mut();
                let parent_dom = parent_node.dom();
                let dom = node.dom_mut();
                println!(
                    "<{:?}> -- id_path: {:?} width: {:?} height: {:?}",
                    dom.tag(),
                    dom.id_path(),
                    dom.width(),
                    dom.height()
                );
                node.layout(parent_dom);
                node.paint(&mut render_ctx);
            }
            TreeNodeEdge::End(node) => {
                let node = node.borrow();
                let dom = node.dom();
                println!("</{}>: {:?}", dom.tag(), node.id_path());
            }
        }
    }

    // 遍历事件树
    if let Some(event_tree) = event_tree {
        for node in event_tree.traverse() {
            match node {
                TreeNodeEdge::Start(node) => {
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
                TreeNodeEdge::Start(node) => {
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
