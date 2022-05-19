use xinghuo_geom::{Color, Point2, Box2};

pub trait DrawIface {
    fn rect(&mut self, rect: Box2<f32>, col: Color);
    fn text(&mut self, text: String, pos: Point2<f32>, size: f32, color: Color);
}

#[derive(Default)]
pub struct DrawDummy {}

impl DrawIface for DrawDummy {
    fn rect(&mut self, rect: Box2<f32>, col: Color) {
        println!("-- draw rect --> rect: {:?}, color: {:?}", &rect, &col);
    }

    fn text(&mut self, text: String, pos: Point2<f32>, size: f32, color: Color) {
        println!(
            "-- draw text --> text: {:?} pos: {:?} size: {:?} color: {:?}",
            &text, &pos, &size, &color
        );
    }
}
