use std::{
    cell::{Ref, RefMut},
    fmt::Debug,
    time::Duration,
};

// use xinghuo_core::elements::{div, p};
use xinghuo_core::{prelude::*, BlockType, Context, DomElement, Quaternion, Size};

// ui_element! {
//     <div>

//     block_type: Block

//     // style_attr: border_color(Color)
//     // style_attr: background_color(Color)

//     // style_attr_with_into: width(Size)
//     // style_attr_with_into: height(Size)
//     // style_attr_with_into: margin(Quaternion)
//     // style_attr_with_into: padding(Quaternion)
//     // style_attr_with_into: border_radius(Quaternion)
// }

pub trait ParentNode {
    fn child(self, node: Box<dyn AnyRenderObject>) -> Self;
}

pub trait AnyRenderObject: RenderObject {
    fn as_any(&self) -> &dyn std::any::Any;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    // fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T>;
    // fn downcast_ref<T: 'static>(&mut self) -> Option<&T>;
}

pub trait Data {
    type DataType;

    fn data(&self) -> &Self::DataType;
    fn data_mut(&mut self) -> &mut Self::DataType;
}

pub trait DrawIface {
    fn rect(&mut self, rect: Rect<f32>, col: Color);
    fn text(&mut self, text: String, pos: Point2<f32>, size: f32, color: Color);
}

pub trait RenderObject: Data {
    /// 计算布局, 获取到子节点, 设置子节点的形状
    fn layout(&mut self, ctx: &Context);
    /// 先画自己, 再画子节点
    fn paint(&mut self, ctx: &mut Context, draw: Box<dyn DrawIface>);
    /// 块类型
    fn block_type(&self) -> BlockType;
    /// 判断是否需要绘画
    fn is_dirty(&self) -> bool {
        true
    }
    ///
    fn set_dirty(&mut self, dirty: bool);
}

pub fn div<T: Debug>(data: T) -> Div<T> {
    Div::new(data)
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
        fn has_some<T: std::any::Any>(v: &Option<T>) -> &str {
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

pub struct Div<T: Debug> {
    data: T,
    dom: DomElement,
    events: Events,
    pub nodes: Vec<Box<dyn AnyRenderObject>>,
}

impl<T: Debug> Debug for Div<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Div")
            // .field("inner", &self.inner)
            .field("data", &self.data)
            .finish()
    }
}

impl<T: Debug> Data for Div<T> {
    type DataType = T;

    fn data(&self) -> &Self::DataType {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Self::DataType {
        &mut self.data
    }
}

impl<T: Debug> Div<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            dom: DomElement::new("Div"),
            events: Events::default(),
            nodes: Vec::default(),
        }
    }

    // fn data(&self) -> &T {
    //     &self.data
    // }

    // fn data_mut(&mut self) -> &mut T {
    //     &mut self.data
    // }
}

impl<T: 'static + RenderObject> AnyRenderObject for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    // fn downcast_mut<N: 'static>(&mut self) -> Option<&mut N> {
    //     self.as_any_mut().downcast_mut()
    // }

    // fn downcast_ref<N: 'static>(&mut self) -> Option<&N> {
    //     self.as_any().downcast_ref()
    // }
}

impl<T: Debug> ParentNode for Div<T> {
    fn child(mut self, node: Box<dyn AnyRenderObject>) -> Self {
        self.nodes.push(node);
        self
    }
}

impl<T: Debug> RenderObject for Div<T> {
    fn layout(&mut self, ctx: &Context) {}

    fn paint(&mut self, ctx: &mut Context, draw: Box<dyn DrawIface>) {
        if self.is_dirty() {
            self.layout(ctx);
        }
    }

    fn block_type(&self) -> BlockType {
        BlockType::Block
    }

    fn set_dirty(&mut self, dirty: bool) {}
}

#[derive(Debug, PartialEq)]
enum Direction {
    MinToMax,
    MaxToMin,
    Center,
}

#[derive(Debug)]
struct Layout {
    // 得到的所有区域
    clip_rect: Rect<f32>,
    // 定位下一次要分配的起始位置
    cursor: Rect<f32>,
    // hor_align: Direction,
    // ver_align: Direction,
}

impl Layout {
    // 分配一个块元素, 自动切换到新行, 从 clip_rect 中分配 100% 的宽度 和 指定高度
    pub fn allocate_block(&mut self, height: f32) {}

    // 分配一个内联元素, 从 clip_rect 中分配指定的宽度, 高度由子节点决定
    pub fn allocate_inline(&mut self, width: f32) {}

    // 分配一个内联块元素, 从 clip_rect 中分配 指定的宽度和高度.
    pub fn allocate_inline_block(&mut self, width: f32, height: f32) {}
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

// trait Backend {
//     fn draw_handle() -> Box<dyn DrawIface>;
// }

fn main() {
    fn click(evt: Click) {
        println!("recv event: {:?}", &evt);
    }

    let div_node = div(1);
    let mut div_node = div_node.child(Box::new(div([1])));

    let node0 = &mut div_node.nodes[0];

    fn to_raw<T: 'static>(root0: &mut dyn std::any::Any) -> Option<&mut T> {
        root0.downcast_mut()
    }

    let child0: &mut Div<[i32; 1]> = to_raw(node0.as_any_mut()).unwrap();

    // let mut root: Box<dyn AnyRenderObject> = Box::new(node);
    // let root0 = div_node.as_any_mut().downcast_mut::<Div<i32>>().unwrap();

    println!("child0: {:?}", child0);

    // let root = ui_view! {
    //     <div width=200.0 height=200.0 onclick=click border="1.0 1.0 1.0" background_color=1>
    //         <div margin="1.0 1.0" width=100.0 height=100.0>
    //             <div width=50.0 height=50.0>"hello"</div>
    //         </div>
    //         <div margin="1.0" width=100.0 height=100.0>
    //             <div width=50.0 height=50.0>"hello"</div>
    //         </div>
    //     </div>
    // };

    // (*self.widget).as_any_mut().downcast_mut()

    // let root = root.as_a

    // let root_node = root0.node_ref();
    // let mut i = 0;

    // for node in root_node.traverse() {
    //     match node {
    //         rctree::NodeEdge::Start(node) => {
    //             println!("{:2} start node: {:#?}", i, &node);
    //             i += 1;
    //         }
    //         rctree::NodeEdge::End(_) => {
    //             // println!("{:2} end node: {:#?}", i, &node);
    //         }
    //     }
    // }
    // std::thread::sleep(Duration::from_millis(100));

    // let dom = root.dom_ref();
    // println!("dom: {:#?}", &dom);

    // 事件列表

    // Document, 创建节点， 删除节点

    // 当执行 Click事件 时
    //      操作 Dom 节点 (添加节点Id 到 相应的节点操作队列)
    //      发起 http请求时,

    // 处理事件队列
    //      捕获所有事件, 计算渲染队列, 局部渲染

    //      渲染线程 Read<Document>

    //
    //      互斥锁: 事件数据
    //
    // 运行线程 (事件处理) , Write<Document>, 维护 异步请求 队列,
    // 请求线程 用于异步
    //
    // 事件线程 与 请求线程

    /*
        UI架构
    */

    // 处理 Command 队列

    // 生成一个 View tree, 绑定数据 和 事件处理, 节点的增删改查, 通过 Command 实现对高层级的节点删除或修改. 触发重绘
    // 同时生成一个 Render tree, 用于 渲染

    // 事件发生时, 对 View tree 遍历, 每一个节点处理事件, 处理函数的bool返回值: true为已处理, 不继续向下传递
}
