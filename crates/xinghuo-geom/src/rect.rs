use core::ops::Range;
use core::ops::{Div, DivAssign, Mul, MulAssign};

use glam::Vec2;

use crate::box2::Box2;
use crate::vec2;

#[derive(Debug, Default, Clone, Copy)]
pub struct Rect {
    pub origin: Vec2,
    pub size: Vec2,
}

impl Rect {
    #[inline]
    pub const fn new(origin: Vec2, size: Vec2) -> Self {
        Rect { origin, size }
    }

    #[inline]
    pub fn zero() -> Self {
        Rect::new(Vec2::ZERO, Vec2::ZERO)
    }

    #[inline]
    pub fn from_size(size: Vec2) -> Self {
        Rect {
            origin: Vec2::ZERO,
            size,
        }
    }
}

impl Rect {
    #[inline]
    pub fn min(&self) -> Vec2 {
        self.origin
    }

    #[inline]
    pub fn max(&self) -> Vec2 {
        self.origin + self.size
    }

    #[inline]
    pub fn max_x(&self) -> f32 {
        self.origin.x + self.size.x
    }

    #[inline]
    pub fn min_x(&self) -> f32 {
        self.origin.x
    }

    #[inline]
    pub fn max_y(&self) -> f32 {
        self.origin.y + self.size.y
    }

    #[inline]
    pub fn min_y(&self) -> f32 {
        self.origin.y
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.size.x
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.size.y
    }

    #[inline]
    pub fn x_range(&self) -> Range<f32> {
        self.min_x()..self.max_x()
    }

    #[inline]
    pub fn y_range(&self) -> Range<f32> {
        self.min_y()..self.max_y()
    }

    /// Returns the same rectangle, translated by a vector.
    #[inline]
    #[must_use]
    pub fn translate(&self, by: Vec2) -> Self {
        Self::new(self.origin + by, self.size)
    }

    #[inline]
    pub fn to_box2d(&self) -> Box2 {
        Box2 {
            min: self.min(),
            max: self.max(),
        }
    }
}

impl Rect {
    /// Returns true if this rectangle contains the point. Points are considered
    /// in the rectangle if they are on the left or top edge, but outside if they
    /// are on the right or bottom edge.
    #[inline]
    pub fn contains(&self, p: Vec2) -> bool {
        self.to_box2d().contains(p)
    }

    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        self.to_box2d().intersects(&other.to_box2d())
    }
}

impl Rect {
    #[inline]
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let box2d = self.to_box2d().intersection_unchecked(&other.to_box2d());

        if box2d.is_empty() {
            return None;
        }

        Some(box2d.to_rect())
    }
}

impl Rect {
    #[inline]
    #[must_use]
    pub fn inflate(&self, width: f32, height: f32) -> Self {
        Rect::new(
            vec2(self.origin.x - width, self.origin.y - height),
            vec2(self.size.x + width + width, self.size.y + height + height),
        )
    }
}

impl Rect {
    /// Returns true if this rectangle contains the interior of rect. Always
    /// returns true if rect is empty, and always returns false if rect is
    /// nonempty but this rectangle is empty.
    #[inline]
    pub fn contains_rect(&self, rect: &Self) -> bool {
        rect.is_empty()
            || (self.min_x() <= rect.min_x()
                && rect.max_x() <= self.max_x()
                && self.min_y() <= rect.min_y()
                && rect.max_y() <= self.max_y())
    }
}

impl Rect {
    /// Linearly interpolate between this rectangle and another rectangle.
    #[inline]
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        Self::new(
            self.origin.lerp(other.origin, t),
            self.size.lerp(other.size, t),
        )
    }
}

impl Rect {
    pub fn center(&self) -> Vec2 {
        self.origin + self.size / 2.0
    }
}

impl Rect {
    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        self.to_box2d().union(&other.to_box2d()).to_rect()
    }
}

impl Rect {
    #[inline]
    pub fn scale(&self, x: f32, y: f32) -> Self {
        Rect::new(
            vec2(self.origin.x * x, self.origin.y * y),
            vec2(self.size.x * x, self.size.y * y),
        )
    }
}

impl Rect {
    #[inline]
    pub fn area(&self) -> f32 {
        self.size.x * self.size.y
    }
}

impl Rect {
    #[inline]
    pub fn is_empty(&self) -> bool {
        !(self.size.x > 0.0 && self.size.y > 0.0)
    }
}

impl Rect {
    #[inline]
    pub fn to_non_empty(&self) -> Option<Self> {
        if self.is_empty() {
            return None;
        }

        Some(*self)
    }
}

impl Mul<f32> for Rect {
    type Output = Rect;

    #[inline]
    fn mul(self, scale: f32) -> Self::Output {
        Rect::new(self.origin * scale, self.size * scale)
    }
}

impl MulAssign<f32> for Rect {
    #[inline]
    fn mul_assign(&mut self, scale: f32) {
        self.origin *= scale;
        self.size *= scale;
    }
}

impl Div<f32> for Rect {
    type Output = Rect;

    #[inline]
    fn div(self, scale: f32) -> Self::Output {
        Rect::new(self.origin / scale.clone(), self.size / scale)
    }
}

impl DivAssign<f32> for Rect {
    #[inline]
    fn div_assign(&mut self, scale: f32) {
        self.origin /= scale.clone();
        self.size /= scale;
    }
}

impl Rect {
    /// Returns true if all members are finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.origin.is_finite() && self.size.is_finite()
    }
}

impl From<Vec2> for Rect {
    fn from(size: Vec2) -> Self {
        Self::from_size(size)
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for Rect {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for Rect {}
