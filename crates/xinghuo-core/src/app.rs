use crate::element::*;
use crate::painter::DummyPainter;
// use crate::context::Context;
use crate::prelude::*;
use crate::TreeNode;

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

// pub trait App {
//     fn name(&mut self) -> &'static str;
//     fn setup(&mut self, ctx: &Context, boot_ctx: &BootContext);
//     fn update(&mut self, ctx: &Context);
//     fn view(&mut self, ctx: &Context);
// }

/// 保存关于 window启动时 的上下文, 用于得到 window 更多的信息
pub struct BootContext {
    pub gpu_device_info: GpuDeviceInfo,
}

pub struct App {
    pub event_tree: Option<TreeNode<EventObject>>,
    pub data_tree: Option<TreeNode<DataObject>>,
    pub render_tree: TreeNode<Box<dyn RenderObject>>,
    pub painter: Box<dyn Painter>,
}

impl App {
    pub fn render(&mut self) {
        let App {
            event_tree,
            data_tree,
            render_tree,
            painter,
        } = self;

        let window_size = painter.size();
        /*
            根节点布局, 强制到 屏幕尺寸, 且 块布局
        */
        {
            let mut root_node = render_tree.borrow_mut();
            let dom = root_node.dom_mut();
            dom.set_style(Style::default());
            dom.set_layout(Layout::Block);
            dom.set_width(window_size[0]);
            dom.set_height(window_size[1]);
            dom.set_ava_box(Box2::new(vec2(0.0, 0.0), window_size.into()));
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
                    let parent_size = dom.size();

                    // 在一个布局中, Cursor移动, 用于记录下一个 布局内节点 的起始位置, 初始位置是 父节点的area
                    let mut cursor = dom.ava_box();

                    match dom.layout() {
                        Layout::Inline => {}
                        Layout::InlineBlock => {}
                        Layout::Block => {
                            // 当前节点是Block, 则子节点的最大宽度和最大高度是确定的: 更新子节点的 宽度/高度/Area/父节点尺寸
                            for mut child in node.children() {
                                let mut child = child.borrow_mut();
                                let cdom = child.dom_mut();

                                // 更新盒子轮廓
                                cdom.set_margin(cdom.margin().update(parent_size));
                                cdom.set_padding(cdom.padding().update(parent_size));
                                cdom.set_border_width(cdom.border_width().update(parent_size));

                                // 当前节点的宽度 = 父节点的宽度 - 当前节点的边沿宽度
                                cdom.set_width(parent_size.x - cdom.edge_width());
                                // 根据父节点高度 计算当前节点的高度
                                cdom.update_height(parent_size.y);
                                // cdom.set_height();

                                // 更新有效区域
                                let start_point = cursor.min + cdom.left_top();
                                let end_point = start_point + vec2(cdom.width(), cdom.height());
                                cdom.set_ava_box(box2(start_point, end_point));

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
                    node.paint(painter);
                }
                TreeNodeEdge::End(node) => {
                    // let node = node.borrow();
                    // let dom = node.dom();
                    // println!("</{}>: {:?}", dom.tag(), node.node_id());
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
    pub fn resize(&mut self, size: [f32; 2]) {
        self.painter.resize(size);
    }
}

pub struct AppBuilder {
    element: Element,
    painter: Box<dyn Painter>,
}

impl AppBuilder {
    pub fn new(b: Element) -> Self {
        Self {
            element: b,
            painter: Box::new(DummyPainter::default()),
        }
    }

    pub fn with_draw(mut self, painter: impl Painter + 'static) -> Self {
        self.painter = Box::new(painter);
        self
    }

    pub fn build(self) -> App {
        let Self { element, painter } = self;
        let (render_tree, event_tree, data_tree) = element.build();
        App {
            render_tree,
            event_tree,
            data_tree,
            painter,
        }
    }
}

// WindowBuilder
