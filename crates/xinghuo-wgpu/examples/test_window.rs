use xinghuo_core::{app::AppBuilder, element::Element, Layout};
use xinghuo_wgpu::window::Window;

fn main() {
    init_log();
    Window::new(AppBuilder::new(div().children(vec![
        header(),
        center(),
        footer(),
    ])))
    .title("hello")
    .size([800, 600])
    .run();
}

/*
    创建节点树: 渲染对象树, 事件树, 数据树
*/

fn div() -> Element {
    Element::new("div")
}

fn span() -> Element {
    let mut span = Element::new("span");
    span.dom.set_layout(Layout::Inline);
    span
}

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

/// 初始化日志
pub fn init_log() {
    std::env::set_var("RUST_LOG", "xinghuo_wgpu=INFO");

    // 在linux系统上, 使用gl驱动, 默认的Vulkan驱动会在屏幕关闭后 出现程序"Timeout"退出(2022-0405)
    if cfg!(target_os = "linux") {
        std::env::set_var("WGPU_BACKEND", "gl");
    }

    tracing_subscriber::fmt::init();
}
