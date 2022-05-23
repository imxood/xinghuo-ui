use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

use xinghuo_core::{
    app::{App, AppBuilder},
    element::{Element, RenderCx},
    event::*,
    id::{Id, IdPath},
    painter::DrawDummy,
    prelude::{euclid::vec2, *},
    Convert, DomElement, Layout, Size, Style,
};

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

fn div() -> Element {
    Element::new("div")
}

fn span() -> Element {
    let mut span = Element::new("span");
    span.dom.set_layout(Layout::Inline);
    span
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

pub struct Window {
    window_width: f32,
    window_height: f32,
    title: String,
}

impl Window {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            window_width: 800.,
            window_height: 600.,
            title: title.into(),
        }
    }
    pub fn run(self, app_builder: AppBuilder) {}
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

    let mut app = AppBuilder::new(
        div()
            .width(window_width)
            .height(window_height)
            .children(vec![header(), center(), footer()]),
    );

    let App {
        mut event_tree,
        mut data_tree,
        mut render_tree,
        painter,
    } = app.build();

    let mut painter: Box<dyn Painter> = Box::new(DummyPainter {});

    let mut render_ctx = RenderCx {
        window_width,
        window_height,
        painter: &mut painter,
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
        dom.set_ava_box(Box2::new(vec2(0.0, 0.0), vec2(window_width, window_width)));
    }

    /*
        执行布局
    */

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
                    "<{:?}> -- node_id: {:?} width: {:?} height: {:?}",
                    dom.tag(),
                    dom.node_id(),
                    dom.width(),
                    dom.height()
                );
                node.layout(parent_dom);
                node.paint(&mut render_ctx);
            }
            TreeNodeEdge::End(node) => {
                let node = node.borrow();
                let dom = node.dom();
                println!("</{}>: {:?}", dom.tag(), node.node_id());
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
                        "event node -- node_id: {:?} event: {:?}",
                        &node.node.borrow().node_id(),
                        &node.event
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
                        "data node -- node_id: {:?} data: {:?}",
                        &node.node.borrow().node_id(),
                        &node.data
                    );
                }
                _ => {}
            }
        }
    }
}

/*
    创建节点树: 渲染对象树, 事件树, 数据树
*/

fn header() -> Element {
    div().width("100%").height("30").onclick(|clicked| {
        println!("{:?}", &clicked);
    })
}

fn center() -> Element {
    div().width("100%").height("100%").onclick(|clicked| {
        println!("{:?}", &clicked);
    })
}

fn footer() -> Element {
    div().width("100%").height("30").onclick(|clicked| {
        println!("{:?}", &clicked);
    })
}
