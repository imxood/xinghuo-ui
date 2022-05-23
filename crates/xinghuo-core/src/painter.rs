use xinghuo_geom::{color::Color, glam::Vec2, Quad};

pub trait Painter {
    fn rect(&mut self, rect: &[Quad]);
    fn text(&mut self, text: String, pos: Vec2, size: f32, color: Color);
    fn resize(&mut self, size: [f32; 2]);
    fn render(&mut self);
    fn size(&self) -> [f32; 2];
}

#[derive(Default)]
pub struct DummyPainter {
    size: [f32; 2],
}

impl Painter for DummyPainter {
    fn rect(&mut self, rect: &[Quad]) {
        println!("-- draw rect --> rect: {:?}", rect);
    }

    fn text(&mut self, text: String, pos: Vec2, size: f32, color: Color) {
        println!(
            "-- draw text --> text: {:?} pos: {:?} size: {:?} color: {:?}",
            &text, &pos, &size, &color
        );
    }

    fn resize(&mut self, size: [f32; 2]) {
        println!("resize event: size [{:?}]", &size);
    }

    fn render(&mut self) {}

    fn size(&self) -> [f32; 2] {
        self.size
    }
}
