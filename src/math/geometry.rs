use crate::math::Vec2;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub const ZERO: Size = Size { width: 0.0, height: 0.0 };

    pub fn new(width: f32, height: f32) -> Self {
        Size { width, height }
    }
    
    pub fn from_vec2(point: Vec2) -> Self {
        Size { width: point.x, height: point.y }
    }
    
    pub fn to_vec2(&self) -> Vec2 {
        Vec2 { x: self.width, y: self.height }
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
    
    pub fn equals(&self, target: &Size) -> bool {
        self.width == target.width && self.height == target.height
    }
}

// Size operators

impl Add for Size {
    type Output = Size;
    fn add(self, other: Size) -> Size {
        Size { width: self.width + other.width, height: self.height + other.height }
    }
}

impl Sub for Size {
    type Output = Size;
    fn sub(self, other: Size) -> Size {
        Size { width: self.width - other.width, height: self.height - other.height }
    }
}

impl Mul<f32> for Size {
    type Output = Size;
    fn mul(self, scalar: f32) -> Size {
        Size { width: self.width * scalar, height: self.height * scalar }
    }
}

impl Div<f32> for Size {
    type Output = Size;
    fn div(self, scalar: f32) -> Size {
        Size { width: self.width / scalar, height: self.height / scalar }
    }
}

// Allow conversion into Vec2
impl From<Size> for Vec2 {
    fn from(s: Size) -> Vec2 {
        Vec2 { x: s.width, y: s.height }
    }
}

impl From<Vec2> for Size {
    fn from(v: Vec2) -> Size {
        Size { width: v.x, height: v.y }
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect {
    pub origin: Vec2,
    pub size: Size,
}

impl Rect {
    pub const ZERO: Rect = Rect { origin: Vec2::ZERO, size: Size::ZERO };

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rect {
            origin: Vec2::new(x, y),
            size: Size::new(width, height),
        }
    }
    
    pub fn from_pos_size(pos: Vec2, size: Size) -> Self {
        Rect { origin: pos, size }
    }

    pub fn set_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.origin.x = x;
        self.origin.y = y;
        self.size.width = width;
        self.size.height = height;
    }

    pub fn width(&self) -> f32 {
        self.size.width
    }

    pub fn height(&self) -> f32 {
        self.size.height
    }

    pub fn get_min_x(&self) -> f32 {
        self.origin.x
    }

    pub fn get_mid_x(&self) -> f32 {
        self.origin.x + self.size.width / 2.0
    }

    pub fn get_max_x(&self) -> f32 {
        self.origin.x + self.size.width
    }

    pub fn get_min_y(&self) -> f32 {
        self.origin.y
    }

    pub fn get_mid_y(&self) -> f32 {
        self.origin.y + self.size.height / 2.0
    }

    pub fn get_max_y(&self) -> f32 {
        self.origin.y + self.size.height
    }

    pub fn equals(&self, rect: &Rect) -> bool {
        self.origin == rect.origin && self.size == rect.size
    }

    pub fn contains_point(&self, point: &Vec2) -> bool {
        point.x >= self.get_min_x() && point.x <= self.get_max_x() &&
        point.y >= self.get_min_y() && point.y <= self.get_max_y()
    }

    pub fn intersects_rect(&self, rect: &Rect) -> bool {
        !(self.get_max_x() < rect.get_min_x() ||
          rect.get_max_x() < self.get_min_x() ||
          self.get_max_y() < rect.get_min_y() ||
          rect.get_max_y() < self.get_min_y())
    }
    
    pub fn intersects_circle(&self, center: &Vec2, radius: f32) -> bool {
        let rectangle_center = Vec2::new(self.get_mid_x(), self.get_mid_y());
        
        // This is a simplified check. A proper circle-rect check requires finding the closest point on the rect.
        // The C++ implementation likely does something specific. 
        // For now, I'll implement the "Clamp method" to find the closest point.
        
        let mut closest_point = *center;
        
        let min_x = self.get_min_x();
        let max_x = self.get_max_x();
        let min_y = self.get_min_y();
        let max_y = self.get_max_y();

        if closest_point.x < min_x { closest_point.x = min_x; }
        else if closest_point.x > max_x { closest_point.x = max_x; }
        
        if closest_point.y < min_y { closest_point.y = min_y; }
        else if closest_point.y > max_y { closest_point.y = max_y; }
        
        closest_point.distance_squared(center) <= radius * radius
    }

    pub fn union_with_rect(&self, rect: &Rect) -> Rect {
        let min_x = self.get_min_x().min(rect.get_min_x());
        let min_y = self.get_min_y().min(rect.get_min_y());
        let max_x = self.get_max_x().max(rect.get_max_x());
        let max_y = self.get_max_y().max(rect.get_max_y());
        
        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    pub fn merge(&mut self, rect: &Rect) {
        *self = self.union_with_rect(rect);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.0001;

    // Size tests
    #[test]
    fn test_size_new() {
        let s = Size::new(100.0, 200.0);
        assert_eq!(s.width, 100.0);
        assert_eq!(s.height, 200.0);
    }

    #[test]
    fn test_size_constants() {
        assert_eq!(Size::ZERO.width, 0.0);
        assert_eq!(Size::ZERO.height, 0.0);
    }

    #[test]
    fn test_size_from_vec2() {
        let v = Vec2::new(50.0, 100.0);
        let s = Size::from_vec2(v);
        assert_eq!(s.width, 50.0);
        assert_eq!(s.height, 100.0);
    }

    #[test]
    fn test_size_to_vec2() {
        let s = Size::new(100.0, 200.0);
        let v = s.to_vec2();
        assert_eq!(v.x, 100.0);
        assert_eq!(v.y, 200.0);
    }

    #[test]
    fn test_size_equals() {
        let s1 = Size::new(100.0, 200.0);
        let s2 = Size::new(100.0, 200.0);
        let s3 = Size::new(50.0, 200.0);
        assert!(s1.equals(&s2));
        assert!(!s1.equals(&s3));
    }

    #[test]
    fn test_size_add() {
        let s1 = Size::new(100.0, 200.0);
        let s2 = Size::new(50.0, 50.0);
        let result = s1 + s2;
        assert_eq!(result.width, 150.0);
        assert_eq!(result.height, 250.0);
    }

    #[test]
    fn test_size_sub() {
        let s1 = Size::new(100.0, 200.0);
        let s2 = Size::new(30.0, 50.0);
        let result = s1 - s2;
        assert_eq!(result.width, 70.0);
        assert_eq!(result.height, 150.0);
    }

    // Rect tests
    #[test]
    fn test_rect_new() {
        let r = Rect::new(10.0, 20.0, 100.0, 200.0);
        assert_eq!(r.x, 10.0);
        assert_eq!(r.y, 20.0);
        assert_eq!(r.width, 100.0);
        assert_eq!(r.height, 200.0);
    }

    #[test]
    fn test_rect_constants() {
        assert_eq!(Rect::ZERO.x, 0.0);
        assert_eq!(Rect::ZERO.y, 0.0);
        assert_eq!(Rect::ZERO.width, 0.0);
        assert_eq!(Rect::ZERO.height, 0.0);
    }

    #[test]
    fn test_rect_bounds() {
        let r = Rect::new(10.0, 20.0, 100.0, 200.0);
        assert_eq!(r.get_min_x(), 10.0);
        assert_eq!(r.get_max_x(), 110.0);
        assert_eq!(r.get_min_y(), 20.0);
        assert_eq!(r.get_max_y(), 220.0);
        assert_eq!(r.get_mid_x(), 60.0);
        assert_eq!(r.get_mid_y(), 120.0);
    }

    #[test]
    fn test_rect_contains_point() {
        let r = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert!(r.contains(&Vec2::new(50.0, 50.0)));
        assert!(r.contains(&Vec2::new(0.0, 0.0)));
        assert!(!r.contains(&Vec2::new(100.0, 100.0))); // on edge, depends on implementation
        assert!(!r.contains(&Vec2::new(150.0, 50.0)));
    }

    #[test]
    fn test_rect_intersects() {
        let r1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let r2 = Rect::new(50.0, 50.0, 100.0, 100.0);
        let r3 = Rect::new(200.0, 200.0, 50.0, 50.0);
        
        assert!(r1.intersects(&r2));
        assert!(!r1.intersects(&r3));
    }

    #[test]
    fn test_rect_union() {
        let r1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let r2 = Rect::new(50.0, 50.0, 100.0, 100.0);
        let union = r1.union_with_rect(&r2);
        
        assert_eq!(union.x, 0.0);
        assert_eq!(union.y, 0.0);
        assert_eq!(union.width, 150.0);
        assert_eq!(union.height, 150.0);
    }

    #[test]
    fn test_rect_merge() {
        let mut r1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let r2 = Rect::new(50.0, 50.0, 100.0, 100.0);
        r1.merge(&r2);
        
        assert_eq!(r1.width, 150.0);
        assert_eq!(r1.height, 150.0);
    }

    #[test]
    fn test_rect_intersects_circle() {
        let r = Rect::new(0.0, 0.0, 100.0, 100.0);
        let center = Vec2::new(50.0, 50.0);
        let center_outside = Vec2::new(150.0, 150.0);
        
        assert!(r.intersects_circle(&center, 10.0));
        assert!(!r.intersects_circle(&center_outside, 5.0));
    }
}
