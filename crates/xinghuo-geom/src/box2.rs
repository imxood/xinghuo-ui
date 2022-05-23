use core::ops::{Div, DivAssign, Mul, MulAssign};
use glam::Vec2;

use crate::{rect::Rect, vec2};

#[derive(Debug, Default, Clone, Copy)]
pub struct Box2 {
    pub min: Vec2,
    pub max: Vec2,
}

impl Box2 {
    #[inline]
    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn from_origin_and_size(origin: Vec2, size: Vec2) -> Self {
        Self {
            min: origin,
            max: vec2(origin.x + size.x, origin.y + size.y),
        }
    }

    #[inline]
    pub fn from_size(size: Vec2) -> Self {
        Self {
            min: Vec2::ZERO,
            max: vec2(size.x, size.y),
        }
    }
}

impl Box2 {
    #[inline]
    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    /// Change the size of the box by adjusting the max endpoint
    /// without modifying the min endpoint.
    #[inline]
    pub fn set_size(&mut self, size: Vec2) {
        let diff = self.size() - size;
        self.max -= diff;
    }

    #[inline]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    #[inline]
    pub fn to_rect(&self) -> Rect {
        Rect {
            origin: self.min,
            size: self.size(),
        }
    }
}

impl Box2 {
    #[inline]
    pub fn translate(&self, by: Vec2) -> Self {
        Box2 {
            min: self.min + by,
            max: self.max + by,
        }
    }
}

impl Box2 {
    /// Returns true if the box has a negative area.
    ///
    /// The common interpretation for a negative box is to consider it empty. It can be obtained
    /// by calculating the intersection of two boxes that do not intersect.
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.max.x < self.min.x || self.max.y < self.min.y
    }

    /// Returns true if the size is zero, negative or NaN.
    #[inline]
    pub fn is_empty(&self) -> bool {
        !(self.max.x > self.min.x && self.max.y > self.min.y)
    }

    /// Returns `true` if the two boxes intersect.
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
    }

    /// Returns `true` if this box contains the point. Points are considered
    /// in the box if they are on the front, left or top faces, but outside if they
    /// are on the back, right or bottom faces.
    #[inline]
    pub fn contains(&self, p: Vec2) -> bool {
        self.min.x <= p.x && p.x < self.max.x && self.min.y <= p.y && p.y < self.max.y
    }

    /// Returns `true` if this box contains the interior of the other box. Always
    /// returns `true` if other is empty, and always returns `false` if other is
    /// nonempty but this box is empty.
    #[inline]
    pub fn contains_box(&self, other: &Self) -> bool {
        other.is_empty()
            || (self.min.x <= other.min.x
                && other.max.x <= self.max.x
                && self.min.y <= other.min.y
                && other.max.y <= self.max.y)
    }
}

impl Box2 {
    #[inline]
    pub fn to_non_empty(&self) -> Option<Self> {
        if self.is_empty() {
            return None;
        }

        Some(*self)
    }

    /// Computes the intersection of two boxes, returning `None` if the boxes do not intersect.
    #[inline]
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let b = self.intersection_unchecked(other);

        if b.is_empty() {
            return None;
        }

        Some(b)
    }

    /// Computes the intersection of two boxes without check whether they do intersect.
    ///
    /// The result is a negative box if the boxes do not intersect.
    /// This can be useful for computing the intersection of more than two boxes, as
    /// it is possible to chain multiple intersection_unchecked calls and check for
    /// empty/negative result at the end.
    #[inline]
    pub fn intersection_unchecked(&self, other: &Self) -> Self {
        Box2 {
            min: vec2(self.min.x.max(other.min.x), self.min.y.max(other.min.y)),
            max: vec2(self.max.x.min(other.max.x), self.max.y.min(other.max.y)),
        }
    }

    /// Computes the union of two boxes.
    ///
    /// If either of the boxes is empty, the other one is returned.
    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        if other.is_empty() {
            return *self;
        }
        if self.is_empty() {
            return *other;
        }

        Box2 {
            min: vec2(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
            max: vec2(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
        }
    }
}

impl Box2 {
    /// Inflates the box by the specified sizes on each side respectively.
    #[inline]
    #[must_use]
    pub fn inflate(&self, width: f32, height: f32) -> Self {
        Box2 {
            min: vec2(self.min.x - width, self.min.y - height),
            max: vec2(self.max.x + width, self.max.y + height),
        }
    }
}

impl Box2 {
    /// Linearly interpolate between this box and another box.
    #[inline]
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        Self::new(self.min.lerp(other.min, t), self.max.lerp(other.max, t))
    }
}

impl Box2 {
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) / 2.0
    }
}

impl Box2 {
    #[inline]
    pub fn area(&self) -> f32 {
        let size = self.size();
        size.x * size.y
    }
}

impl Box2 {
    /// Constructor, setting all sides to zero.
    pub fn zero() -> Self {
        Box2::new(Vec2::ZERO, Vec2::ZERO)
    }
}

impl Mul<f32> for Box2 {
    type Output = Box2;

    #[inline]
    fn mul(self, scale: f32) -> Self::Output {
        Box2::new(self.min * scale, self.max * scale)
    }
}

impl MulAssign<f32> for Box2 {
    #[inline]
    fn mul_assign(&mut self, scale: f32) {
        *self = Box2::new(self.min * scale, self.max * scale);
    }
}

impl Div<f32> for Box2 {
    type Output = Box2;

    #[inline]
    fn div(self, scale: f32) -> Self::Output {
        Box2::new(self.min / scale, self.max / scale)
    }
}

impl DivAssign<f32> for Box2 {
    #[inline]
    fn div_assign(&mut self, scale: f32) {
        *self = Box2::new(self.min / scale, self.max / scale);
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for Box2 {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for Box2 {}
