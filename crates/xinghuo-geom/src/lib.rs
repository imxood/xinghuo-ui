pub use euclid::default::Box2D;
pub type Transform<S> = euclid::default::Transform2D<S>;
pub type Rotation<S> = euclid::default::Rotation2D<S>;
pub type Translation<S> = euclid::Translation2D<S, euclid::UnknownUnit, euclid::UnknownUnit>;
pub use euclid::default::Scale;
pub use euclid::Angle;

use euclid::UnknownUnit;

// pub type Length<T> = euclid::Length<T, UnknownUnit>;
pub type Point2<T> = euclid::Point2D<T, UnknownUnit>;
pub type Vector2<T> = euclid::Vector2D<T, UnknownUnit>;
// pub type Vector3D<T> = euclid::Vector3D<T, UnknownUnit>;
// pub type HomogeneousVector<T> = euclid::HomogeneousVector<T, UnknownUnit>;
pub type Size2<T> = euclid::Size2D<T, UnknownUnit>;
// pub type Size3D<T> = euclid::Size3D<T, UnknownUnit>;
pub type Rect<T> = euclid::Rect<T, UnknownUnit>;
pub type Box2<T> = euclid::Box2D<T, UnknownUnit>;

pub type Point3<T> = euclid::Point3D<T, UnknownUnit>;
// pub type Box3D<T> = euclid::Box3D<T, UnknownUnit>;
// pub type SideOffsets2D<T> = euclid::SideOffsets2D<T, UnknownUnit>;
// pub type Transform2D<T> = euclid::Transform2D<T, UnknownUnit, UnknownUnit>;
// pub type Transform3D<T> = euclid::Transform3D<T, UnknownUnit, UnknownUnit>;
// pub type Rotation2D<T> = euclid::Rotation2D<T, UnknownUnit, UnknownUnit>;
// pub type Rotation3D<T> = euclid::Rotation3D<T, UnknownUnit, UnknownUnit>;
// pub type Translation2D<T> = euclid::Translation2D<T, UnknownUnit, UnknownUnit>;
// pub type Translation3D<T> = euclid::Translation3D<T, UnknownUnit, UnknownUnit>;
// pub type Scale<T> = euclid::Scale<T, UnknownUnit, UnknownUnit>;
// pub type RigidTransform3D<T> = euclid::RigidTransform3D<T, UnknownUnit, UnknownUnit>;

#[inline]
pub fn size2<T>(w: T, h: T) -> Size2<T> {
    Size2::new(w, h)
}

#[inline]
pub fn vector2<T>(w: T, h: T) -> Vector2<T> {
    Vector2::new(w, h)
}

#[inline]
pub fn point2<T>(x: T, y: T) -> Point2<T> {
    Point2::new(x, y)
}

#[inline]
pub fn box2<T>(min: Point2<T>, max: Point2<T>) -> Box2<T> {
    Box2::new(min, max)
}

pub fn rect2<T>(p: Point2<T>, s: Size2<T>) -> Rect<T> {
    Rect::new(p, s)
}

#[inline]
pub fn point3<T>(x: T, y: T, z: T) -> Point3<T> {
    Point3::new(x, y, z)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color([u8; 4]);

impl Default for Color {
    fn default() -> Self {
        Self([255, 255, 255, 255])
    }
}

impl Color {
    pub const RED: Self = Self([0xff, 0x00, 0x00, 0xff]);
    pub const GLEEN: Self = Self([0x00, 0xff, 0x00, 0xff]);
    pub const BLUE: Self = Self([0x00, 0x00, 0xff, 0xff]);
    pub const YELLOW: Self = Self([0xff, 0xff, 0x00, 0xff]);

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

/// 从字符串中获取Rgba
/// 1. 以'#'开头, 后面是4个8位的十六进制数字, 如: "#12345678"
/// 2. 4个u8的数组或元组, 如: "[255, 0, 0, 255]" "(255, 0, 0, 255)"
///
impl From<&str> for Color {
    fn from(s: &str) -> Self {
        if s.len() == 9 && s.starts_with('#') {
            let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(0xff);
            let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(0xff);
            let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(0xff);
            let a = u8::from_str_radix(&s[7..9], 16).unwrap_or(0xff);
            return Self([r, g, b, a]);
        }
        let s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        if s.starts_with('(') && s.starts_with(')') || s.starts_with('[') && s.starts_with(']') {
            let s = &s[1..s.len() - 1];
            let s = s.split(',').collect::<Vec<_>>();
            if s.len() != 4 {
                return Self::default();
            }
            let r = u8::from_str_radix(s[0], 16).unwrap_or(0xff);
            let g = u8::from_str_radix(s[1], 16).unwrap_or(0xff);
            let b = u8::from_str_radix(s[2], 16).unwrap_or(0xff);
            let a = u8::from_str_radix(s[3], 16).unwrap_or(0xff);
            return Self([r, g, b, a]);
        }
        Self::default()
    }
}

/// 从u32数据中获取Rgba
/// 如: 0xff0000ff
impl From<u32> for Color {
    fn from(n: u32) -> Self {
        Self([
            (n >> 24 & 0xff) as u8,
            (n >> 16 & 0xff) as u8,
            (n >> 8 & 0xff) as u8,
            (n & 0xff) as u8,
        ])
    }
}
