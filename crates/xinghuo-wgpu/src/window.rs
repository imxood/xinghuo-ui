use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window,
};
use xinghuo_core::{
    app::{App, AppBuilder},
    prelude::{glam::vec2, Vec2},
};

use crate::painter::WindowPainter;

pub enum CustomEvent {}

pub struct Window {
    app_builder: AppBuilder,
    window_builder: window::WindowBuilder,
}

impl Window {
    pub fn new(app_builder: AppBuilder) -> Self {
        Self {
            app_builder,
            window_builder: window::WindowBuilder::new(),
        }
        // let window =
        //     .with_title(title)
        //     .with_inner_size(winit::dpi::PhysicalSize::new(1024, 720))
        //     .build(&event_loop)
        //     .unwrap();
    }

    pub fn size(mut self, size: [u32; 2]) -> Self {
        self.window_builder = self
            .window_builder
            .with_inner_size(winit::dpi::PhysicalSize::new(size[0], size[1]));
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.window_builder = self.window_builder.with_title(title.into());
        self
    }

    pub fn run(self) {
        let Self {
            app_builder,
            window_builder,
        } = self;

        let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event().build();
        let window = window_builder.build(&event_loop).unwrap();

        let app = app_builder.with_draw(WindowPainter::new(&window)).build();

        run_native(window, event_loop, app);
    }
}

fn run_native(window: window::Window, event_loop: EventLoop<CustomEvent>, mut app: App) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(size),
                ..
            } => {
                app.resize([size.width as f32, size.height as f32]);
            }

            Event::RedrawRequested(window_id) => {
                app.render();
            }

            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            _ => {}
        }
    });
}
