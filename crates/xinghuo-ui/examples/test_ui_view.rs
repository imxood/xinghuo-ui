#![feature(associated_type_bounds)]

use std::time::Duration;

use xinghuo_ui::elements::{div, p};
use xinghuo_ui::{prelude::*, DomNode};

fn test() {
    // fn click(evt: Click) {
    //     println!("recv event: {:?}", &evt);
    // }
    // div().onclick(click).child(div()).child(
    //     div()
    //         .child(p())
    //         .child(p())
    //         .child(p())
    //         .child(p())
    //         .child(p())
    //         .child(p()),
    // );
}

ui_element! {
    <s>

    attributes {
        width(f32)
        height(f32)
        margin(Value)
        padding(Value)
        border(Value)
        border_color(u32)
        background_color(u32)
    }
}

fn main() {
    let width = 200.0;
    let root = ui_view! {
        <div
            width=width
            height=80.0
            margin=Value::Single(5.)
            padding=Value::Single(10.)
            background_color=0xffff0000 onclick=|click|{
                println!("click: {:?}", &click);
            }
        >
            <p>
                <p>
                    <div />
                </p>
            </p>
            <div margin=Value::Single(5.) />
            <div onclick=|click| {} margin=Value::Single(5.) />
        </div>
    };

    let root_node = root.node_ref();
    let mut i = 0;

    loop {
        for node in root_node.traverse() {
            match node {
                rctree::NodeEdge::Start(node) => {
                    println!("{:2} start node: {:#?}", i, &node);
                    i += 1;
                }
                rctree::NodeEdge::End(_) => {
                    // println!("{:2} end node: {:#?}", i, &node);
                }
            }
        }
        std::thread::sleep(Duration::from_millis(100));
        break;
    }

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

    //
}
