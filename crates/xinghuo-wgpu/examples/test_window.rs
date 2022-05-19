fn main() {
    init_log();
    run_native("星火UI");
}


pub fn init_log() {
    std::env::set_var("RUST_LOG", "xinghuo_wgpu=INFO");

    // 在linux系统上, 使用gl驱动, 默认的Vulkan驱动会在屏幕关闭后 出现程序"Timeout"退出(2022-0405)
    if cfg!(target_os = "linux") {
        std::env::set_var("WGPU_BACKEND", "gl");
    }

    tracing_subscriber::fmt::init();
}
