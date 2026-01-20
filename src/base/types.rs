use std::ops::{Add, Sub, Mul, Div};
use crate::math::Vec2;

/// Color type for 3 components (RGB)
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

/// Color type for 4 components (RGBA)
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

/// Color type with float components (RGBA)
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

/// Point/Vector2D type
pub type Point = Vec2;

/// Size type
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

/// Rectangle type
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
