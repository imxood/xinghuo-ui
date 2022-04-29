use cgmath::{Point2, Vector2, Vector4};
use xinghuo_ui::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    /// Paint nothing. This can be useful as a placeholder.
    Noop,
    /// Recursively nest more shapes - sometimes a convenience to be able to do.
    /// For performance reasons it is better to avoid it.
    Vec(Vec<Shape>),
    Circle(CircleShape),
    /// A line between two points.
    LineSegment {
        points: [Point2<f32>; 2],
        stroke: Stroke,
    },
    /// A series of lines between points.
    /// The path can have a stroke and/or fill (if closed).
    Path(PathShape),
    Rect(RectShape),
    // Text(TextShape),
    // Mesh(Mesh),
    // QuadraticBezier(QuadraticBezierShape),
    // CubicBezier(CubicBezierShape),

    // Callback(PaintCallback),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color32(Vector4<u8>);

impl Default for Color32 {
    fn default() -> Self {
        Self(Vector4 {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        })
    }
}

impl Color32 {
    #[inline(always)]
    pub fn r(&self) -> u8 {
        self.0[0]
    }

    #[inline(always)]
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    #[inline(always)]
    pub fn b(&self) -> u8 {
        self.0[2]
    }

    #[inline(always)]
    pub fn a(&self) -> u8 {
        self.0[3]
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CircleShape {
    pub center: Point2<f32>,
    pub radius: f32,
    pub fill: Color32,
    pub stroke: Stroke,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Stroke {
    pub width: f32,
    pub color: Color32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PathShape {
    /// Filled paths should prefer clockwise order.
    pub points: Vec<Point2<f32>>,
    /// If true, connect the first and last of the points together.
    /// This is required if `fill != TRANSPARENT`.
    pub closed: bool,
    /// Fill is only supported for convex polygons.
    pub fill: Color32,
    pub stroke: Stroke,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct RectShape {
    pub rect: Rect,
    pub fill: Color32,
    pub border: Stroke,
    pub boader_radiu: Value,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    /// One of the corners of the rectangle, usually the left top one.
    pub pos: Point2<f32>,

    /// The other corner, opposing [`Self::min`]. Usually the right bottom one.
    pub size: Vector2<f32>,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            pos: Point2 { x: 0.0, y: 0.0 },
            size: Vector2 { x: 0.0, y: 0.0 },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Rounding {
    pub left_top: f32,
    pub right_top: f32,
    pub right_bottom: f32,
    pub left_bottm: f32,
}
