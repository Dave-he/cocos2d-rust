use std::ops::{Add, Sub};
use crate::math::Vec2;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color3B {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color3B {
    pub const WHITE: Color3B = Color3B { r: 255, g: 255, b: 255 };
    pub const YELLOW: Color3B = Color3B { r: 255, g: 255, b: 0 };
    pub const GREEN: Color3B = Color3B { r: 0, g: 255, b: 0 };
    pub const BLUE: Color3B = Color3B { r: 0, g: 0, b: 255 };
    pub const RED: Color3B = Color3B { r: 255, g: 0, b: 0 };
    pub const MAGENTA: Color3B = Color3B { r: 255, g: 0, b: 255 };
    pub const BLACK: Color3B = Color3B { r: 0, g: 0, b: 0 };
    pub const ORANGE: Color3B = Color3B { r: 255, g: 128, b: 0 };
    pub const GRAY: Color3B = Color3B { r: 166, g: 166, b: 166 };

    #[inline]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color3B { r, g, b }
    }

    #[inline]
    pub fn from_float3(r: f32, g: f32, b: f32) -> Self {
        Color3B {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
        }
    }

    #[inline]
    pub fn to_color4f(&self, a: u8) -> Color4F {
        Color4F::new(self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, a as f32 / 255.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color4B {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color4B {
    pub const WHITE: Color4B = Color4B { r: 255, g: 255, b: 255, a: 255 };
    pub const YELLOW: Color4B = Color4B { r: 255, g: 255, b: 0, a: 255 };
    pub const GREEN: Color4B = Color4B { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Color4B = Color4B { r: 0, g: 0, b: 255, a: 255 };
    pub const RED: Color4B = Color4B { r: 255, g: 0, b: 0, a: 255 };
    pub const MAGENTA: Color4B = Color4B { r: 255, g: 0, b: 255, a: 255 };
    pub const BLACK: Color4B = Color4B { r: 0, g: 0, b: 0, a: 255 };
    pub const ORANGE: Color4B = Color4B { r: 255, g: 128, b: 0, a: 255 };
    pub const GRAY: Color4B = Color4B { r: 166, g: 166, b: 166, a: 255 };
    pub const TRANSPARENT: Color4B = Color4B { r: 0, g: 0, b: 0, a: 0 };

    #[inline]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color4B { r, g, b, a }
    }

    #[inline]
    pub fn from_color3b(color: Color3B, a: u8) -> Self {
        Color4B { r: color.r, g: color.g, b: color.b, a }
    }

    #[inline]
    pub fn to_color4f(&self) -> Color4F {
        Color4F::new(self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color4F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color4F {
    pub const WHITE: Color4F = Color4F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Color4F = Color4F { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color4F = Color4F { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color4F = Color4F { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const RED: Color4F = Color4F { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const MAGENTA: Color4F = Color4F { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color4F = Color4F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const TRANSPARENT: Color4F = Color4F { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

    #[inline]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color4F { r, g, b, a }
    }

    #[inline]
    pub fn from_color4b(color: Color4B) -> Self {
        Color4F::new(color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0, color.a as f32 / 255.0)
    }

    #[inline]
    pub fn equal(&self, other: &Color4F, variance: f32) -> bool {
        (self.r - other.r).abs() <= variance
            && (self.g - other.g).abs() <= variance
            && (self.b - other.b).abs() <= variance
            && (self.a - other.a).abs() <= variance
    }
}

pub type Point = Vec2;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const ZERO: Size = Size { width: 0.0, height: 0.0 };

    #[inline]
    pub fn new(width: f32, height: f32) -> Self {
        Size { width, height }
    }

    #[inline]
    pub fn set(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}

impl Add for Size {
    type Output = Size;
    fn add(self, other: Size) -> Size {
        Size::new(self.width + other.width, self.height + other.height)
    }
}

impl Sub for Size {
    type Output = Size;
    fn sub(self, other: Size) -> Size {
        Size::new(self.width - other.width, self.height - other.height)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub const ZERO: Rect = Rect {
        origin: Point::ZERO,
        size: Size::ZERO,
    };

    #[inline]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rect {
            origin: Point::new(x, y),
            size: Size::new(width, height),
        }
    }

    #[inline]
    pub fn from_size(size: Size) -> Self {
        Rect {
            origin: Point::ZERO,
            size,
        }
    }

    #[inline]
    pub fn get_min_x(&self) -> f32 {
        self.origin.x
    }

    #[inline]
    pub fn get_mid_x(&self) -> f32 {
        self.origin.x + self.size.width / 2.0
    }

    #[inline]
    pub fn get_max_x(&self) -> f32 {
        self.origin.x + self.size.width
    }

    #[inline]
    pub fn get_min_y(&self) -> f32 {
        self.origin.y
    }

    #[inline]
    pub fn get_mid_y(&self) -> f32 {
        self.origin.y + self.size.height / 2.0
    }

    #[inline]
    pub fn get_max_y(&self) -> f32 {
        self.origin.y + self.size.height
    }

    #[inline]
    pub fn contains_point(&self, point: &Point) -> bool {
        point.x >= self.origin.x
            && point.x <= self.origin.x + self.size.width
            && point.y >= self.origin.y
            && point.y <= self.origin.y + self.size.height
    }

    #[inline]
    pub fn intersects_rect(&self, rect: &Rect) -> bool {
        self.origin.x < rect.origin.x + rect.size.width
            && self.origin.x + self.size.width > rect.origin.x
            && self.origin.y < rect.origin.y + rect.size.height
            && self.origin.y + self.size.height > rect.origin.y
    }

    #[inline]
    pub fn union_rect(&self, rect: &Rect) -> Rect {
        let min_x = self.origin.x.min(rect.origin.x);
        let min_y = self.origin.y.min(rect.origin.y);
        let max_x = (self.origin.x + self.size.width).max(rect.origin.x + rect.size.width);
        let max_y = (self.origin.y + self.size.height).max(rect.origin.y + rect.size.height);
        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    #[inline]
    pub fn intersect_rect(&self, rect: &Rect) -> Rect {
        let min_x = self.origin.x.max(rect.origin.x);
        let min_y = self.origin.y.max(rect.origin.y);
        let max_x = (self.origin.x + self.size.width).min(rect.origin.x + rect.size.width);
        let max_y = (self.origin.y + self.size.height).min(rect.origin.y + rect.size.height);

        if max_x > min_x && max_y > min_y {
            Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
        } else {
            Rect::ZERO
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color3b_new() {
        let color = Color3B::new(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_color3b_predefined_colors() {
        assert_eq!(Color3B::WHITE.r, 255);
        assert_eq!(Color3B::WHITE.g, 255);
        assert_eq!(Color3B::WHITE.b, 255);

        assert_eq!(Color3B::BLACK.r, 0);
        assert_eq!(Color3B::BLACK.g, 0);
        assert_eq!(Color3B::BLACK.b, 0);

        assert_eq!(Color3B::RED.r, 255);
        assert_eq!(Color3B::RED.g, 0);
        assert_eq!(Color3B::RED.b, 0);

        assert_eq!(Color3B::GREEN.r, 0);
        assert_eq!(Color3B::GREEN.g, 255);
        assert_eq!(Color3B::GREEN.b, 0);

        assert_eq!(Color3B::BLUE.r, 0);
        assert_eq!(Color3B::BLUE.g, 0);
        assert_eq!(Color3B::BLUE.b, 255);
    }

    #[test]
    fn test_color3b_from_float3() {
        let color = Color3B::from_float3(1.0, 0.5, 0.0);
        assert_eq!(color.r, 255);
        assert!((color.g as f32 - 127.5).abs() < 1.0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_color3b_to_color4f() {
        let color = Color3B::new(255, 128, 64);
        let color4f = color.to_color4f(128);
        assert!((color4f.r - 1.0).abs() < 0.01);
        assert!((color4f.g - 0.5).abs() < 0.01);
        assert!((color4f.b - 0.25).abs() < 0.01);
        assert!((color4f.a - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_color4b_new() {
        let color = Color4B::new(255, 128, 64, 200);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 200);
    }

    #[test]
    fn test_color4b_predefined_colors() {
        assert_eq!(Color4B::WHITE, Color4B { r: 255, g: 255, b: 255, a: 255 });
        assert_eq!(Color4B::TRANSPARENT, Color4B { r: 0, g: 0, b: 0, a: 0 });
    }

    #[test]
    fn test_color4b_from_color3b() {
        let color3b = Color3B::new(100, 150, 200);
        let color4b = Color4B::from_color3b(color3b, 128);
        assert_eq!(color4b.r, 100);
        assert_eq!(color4b.g, 150);
        assert_eq!(color4b.b, 200);
        assert_eq!(color4b.a, 128);
    }

    #[test]
    fn test_color4b_to_color4f() {
        let color = Color4B::new(255, 255, 255, 128);
        let color4f = color.to_color4f();
        assert!((color4f.r - 1.0).abs() < 0.01);
        assert!((color4f.g - 1.0).abs() < 0.01);
        assert!((color4f.b - 1.0).abs() < 0.01);
        assert!((color4f.a - 0.50196).abs() < 0.001);
    }

    #[test]
    fn test_color4f_new() {
        let color = Color4F::new(0.5, 0.25, 0.75, 1.0);
        assert!((color.r - 0.5).abs() < 0.001);
        assert!((color.g - 0.25).abs() < 0.001);
        assert!((color.b - 0.75).abs() < 0.001);
        assert!((color.a - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_color4f_predefined_colors() {
        assert!((Color4F::WHITE.r - 1.0).abs() < 0.001);
        assert!((Color4F::TRANSPARENT.a - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_color4f_equal() {
        let color1 = Color4F::new(1.0, 0.5, 0.25, 0.75);
        let color2 = Color4F::new(1.0, 0.5, 0.25, 0.75);
        let color3 = Color4F::new(1.0, 0.5, 0.30, 0.75);

        assert!(color1.equal(&color2, 0.01));
        assert!(!color1.equal(&color3, 0.01));
        assert!(color1.equal(&color3, 0.1));
    }

    #[test]
    fn test_color4f_from_color4b() {
        let color4b = Color4B::new(128, 64, 32, 255);
        let color4f = Color4F::from_color4b(color4b);
        assert!((color4f.r - 0.50196).abs() < 0.001);
        assert!((color4f.g - 0.25098).abs() < 0.001);
        assert!((color4f.b - 0.12549).abs() < 0.001);
        assert!((color4f.a - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_size_new() {
        let size = Size::new(100.0, 200.0);
        assert_eq!(size.width, 100.0);
        assert_eq!(size.height, 200.0);
    }

    #[test]
    fn test_size_zero() {
        let size = Size::ZERO;
        assert_eq!(size.width, 0.0);
        assert_eq!(size.height, 0.0);
    }

    #[test]
    fn test_size_set() {
        let mut size = Size::new(100.0, 200.0);
        size.set(300.0, 400.0);
        assert_eq!(size.width, 300.0);
        assert_eq!(size.height, 400.0);
    }

    #[test]
    fn test_size_add() {
        let size1 = Size::new(100.0, 200.0);
        let size2 = Size::new(50.0, 100.0);
        let result = size1 + size2;
        assert_eq!(result.width, 150.0);
        assert_eq!(result.height, 300.0);
    }

    #[test]
    fn test_size_sub() {
        let size1 = Size::new(100.0, 200.0);
        let size2 = Size::new(30.0, 50.0);
        let result = size1 - size2;
        assert_eq!(result.width, 70.0);
        assert_eq!(result.height, 150.0);
    }

    #[test]
    fn test_rect_new() {
        let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
        assert_eq!(rect.origin.x, 10.0);
        assert_eq!(rect.origin.y, 20.0);
        assert_eq!(rect.size.width, 100.0);
        assert_eq!(rect.size.height, 200.0);
    }

    #[test]
    fn test_rect_from_size() {
        let size = Size::new(100.0, 200.0);
        let rect = Rect::from_size(size);
        assert_eq!(rect.origin.x, 0.0);
        assert_eq!(rect.origin.y, 0.0);
        assert_eq!(rect.size.width, 100.0);
        assert_eq!(rect.size.height, 200.0);
    }

    #[test]
    fn test_rect_zero() {
        let rect = Rect::ZERO;
        assert_eq!(rect.origin.x, 0.0);
        assert_eq!(rect.origin.y, 0.0);
        assert_eq!(rect.size.width, 0.0);
        assert_eq!(rect.size.height, 0.0);
    }

    #[test]
    fn test_rect_get_min_x() {
        let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
        assert_eq!(rect.get_min_x(), 10.0);
    }

    #[test]
    fn test_rect_get_mid_x() {
        let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
        assert_eq!(rect.get_mid_x(), 60.0);
    }

    #[test]
    fn test_rect_get_max_x() {
        let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
        assert_eq!(rect.get_max_x(), 110.0);
    }

    #[test]
    fn test_rect_get_min_y() {
        let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
        assert_eq!(rect.get_min_y(), 20.0);
    }

    #[test]
    fn test_rect_get_mid_y() {
        let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
        assert_eq!(rect.get_mid_y(), 120.0);
    }

    #[test]
    fn test_rect_get_max_y() {
        let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
        assert_eq!(rect.get_max_y(), 220.0);
    }

    #[test]
    fn test_rect_contains_point_inside() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        let point = Point::new(50.0, 50.0);
        assert!(rect.contains_point(&point));
    }

    #[test]
    fn test_rect_contains_point_on_edge() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        let point = Point::new(0.0, 0.0);
        assert!(rect.contains_point(&point));
        let point = Point::new(100.0, 100.0);
        assert!(rect.contains_point(&point));
    }

    #[test]
    fn test_rect_contains_point_outside() {
        let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
        let point = Point::new(150.0, 50.0);
        assert!(!rect.contains_point(&point));
        let point = Point::new(50.0, 150.0);
        assert!(!rect.contains_point(&point));
    }

    #[test]
    fn test_rect_intersects_overlapping() {
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = Rect::new(50.0, 50.0, 100.0, 100.0);
        assert!(rect1.intersects_rect(&rect2));
    }

    #[test]
    fn test_rect_intersects_non_overlapping() {
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = Rect::new(200.0, 200.0, 100.0, 100.0);
        assert!(!rect1.intersects_rect(&rect2));
    }

    #[test]
    fn test_rect_intersects_touching() {
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = Rect::new(100.0, 100.0, 100.0, 100.0);
        assert!(!rect1.intersects_rect(&rect2));
    }

    #[test]
    fn test_rect_union() {
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = Rect::new(50.0, 50.0, 100.0, 100.0);
        let union = rect1.union_rect(&rect2);
        assert_eq!(union.origin.x, 0.0);
        assert_eq!(union.origin.y, 0.0);
        assert_eq!(union.size.width, 150.0);
        assert_eq!(union.size.height, 150.0);
    }

    #[test]
    fn test_rect_union_non_overlapping() {
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = Rect::new(200.0, 200.0, 100.0, 100.0);
        let union = rect1.union_rect(&rect2);
        assert_eq!(union.origin.x, 0.0);
        assert_eq!(union.origin.y, 0.0);
        assert_eq!(union.size.width, 300.0);
        assert_eq!(union.size.height, 300.0);
    }

    #[test]
    fn test_rect_intersect() {
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = Rect::new(50.0, 50.0, 100.0, 100.0);
        let intersect = rect1.intersect_rect(&rect2);
        assert_eq!(intersect.origin.x, 50.0);
        assert_eq!(intersect.origin.y, 50.0);
        assert_eq!(intersect.size.width, 50.0);
        assert_eq!(intersect.size.height, 50.0);
    }

    #[test]
    fn test_rect_intersect_no_overlap() {
        let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let rect2 = Rect::new(200.0, 200.0, 100.0, 100.0);
        let intersect = rect1.intersect_rect(&rect2);
        assert_eq!(intersect, Rect::ZERO);
    }

    #[test]
    fn test_point_type_alias() {
        let point: Point = Vec2::new(5.0, 10.0);
        assert_eq!(point.x, 5.0);
        assert_eq!(point.y, 10.0);
    }

    #[test]
    fn test_all_color_traits() {
        let color1 = Color3B::new(100, 150, 200);
        let color2 = color1;
        let color3 = color1.clone();

        assert_eq!(color1, color2);
        assert_eq!(color2, color3);

        let debug_str = format!("{:?}", color1);
        assert!(debug_str.contains("100"));
    }
}
