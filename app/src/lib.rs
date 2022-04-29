use backend::{ScreenDescriptor, WgpuBackend, wgpu};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn window_app() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("定制UI")
        .build(&event_loop)
        .unwrap();

    // window.set_inner_size(PhysicalSize::new(1550, 900));

    let PhysicalSize { width, height } = window.inner_size();
    let scale_factor = window.scale_factor() as f32;

    let screen_descriptor = ScreenDescriptor {
        physical_width: width,
        physical_height: height,
        scale_factor,
    };

    let mut render = WgpuBackend::new(&window, width, height);

    let image_data = include_bytes!("./assets/happy-tree.png");
    let image = image::load_from_memory(image_data).unwrap();
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    let img = epaint::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

    let img = epaint::ImageDelta::full(img);

    let mut texture_delta = epaint::textures::TexturesDelta::default();
    texture_delta.set.insert(epaint::TextureId::default(), img);

    render.update_textures(&texture_delta);

    let mut tessellator = epaint::Tessellator::from_options(epaint::TessellationOptions::default());
    let mut clip_rect_mesh = epaint::Mesh::default();

    tessellator.tessellate_shape(
        [4096, 64],
        epaint::Shape::Circle(epaint::CircleShape::filled(
            epaint::pos2(50.0, 50.0),
            50.0,
            epaint::Color32::LIGHT_BLUE,
        )),
        &mut clip_rect_mesh,
    );

    let clipped_mesh = epaint::ClippedMesh(
        epaint::Rect::from_min_max(epaint::pos2(0.0, 0.0), epaint::pos2(200.0, 200.0)),
        clip_rect_mesh,
    );
    let clipped_meshes = [clipped_mesh];

    render.update_buffers(&clipped_meshes, &screen_descriptor);

    event_loop.run(move |event, _, control_flow| {
        tracing::info!("{:?}", event);
        let mut redraw = |size: Option<[u32; 2]>| {
            if let Some(size) = size {
                render.resize(size[0], size[1]);
            }
            match render.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Outdated) => {
                    // This error occurs when the app is minimized on Windows.
                    // Silently return here to prevent spamming the console with:
                    // "The underlying surface has changed, and therefore the swap chain must be updated"
                    return;
                }
                Err(e) => {
                    tracing::error!("render with error: {}", e);
                    return;
                }
            };
        };
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
                ..
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    redraw(Some([physical_size.width, physical_size.height]));
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    redraw(Some([new_inner_size.width, new_inner_size.height]));
                }
                _ => {}
            },
            Event::RedrawRequested(_) | Event::RedrawEventsCleared => {
                redraw(None);
            }
            _ => {}
        }
    });
}

pub fn init_log() {
    std::env::set_var("RUST_LOG", "INFO");

    // 在linux系统上, 使用gl驱动, 默认的Vulkan驱动会在屏幕关闭后 出现程序"Timeout"退出(2022-0405)
    if cfg!(target_os = "linux") {
        std::env::set_var("WGPU_BACKEND", "gl");
    }

    tracing_subscriber::fmt::init();
}

#[cfg(target_os = "android")]
#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
fn android_entry() {
    android_init();
    window_app();
}

#[cfg(target_os = "android")]
fn android_init() {
    use std::time::Duration;
    use ndk_glue::Event;

    init_log();
    loop {  
        if let Some(event) = ndk_glue::poll_events() {
            tracing::info!("event: {:?}", &event);
            if event == Event::WindowCreated {
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}