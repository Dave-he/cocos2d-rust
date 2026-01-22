use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::f32;
use crate::math::Vec2; // Assuming we might interact with Vec2, though not strictly required by header logic yet.

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
    pub const UNIT_X: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const UNIT_Y: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const UNIT_Z: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn from_array(array: &[f32; 3]) -> Self {
        Vec3 { x: array[0], y: array[1], z: array[2] }
    }
    
    // Simplification: Color conversion is usually dependent on format (RGBA vs ARGB), 
    // but the header implies 0xRRGGBB.
    pub fn from_color(color: u32) -> Self {
        let r = ((color >> 16) & 0xFF) as f32 / 255.0;
        let g = ((color >> 8) & 0xFF) as f32 / 255.0;
        let b = (color & 0xFF) as f32 / 255.0;
        Vec3::new(r, g, b)
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn is_one(&self) -> bool {
        self.x == 1.0 && self.y == 1.0 && self.z == 1.0
    }

    pub fn angle(v1: &Vec3, v2: &Vec3) -> f32 {
        let dot = v1.dot(v2);
        let len_sq = v1.length_squared() * v2.length_squared();
        (dot / len_sq.sqrt()).acos()
    }

    pub fn add(&mut self, v: &Vec3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
    
    pub fn add_components(&mut self, x: f32, y: f32, z: f32) {
        self.x += x;
        self.y += y;
        self.z += z;
    }

    pub fn clamp(&mut self, min: &Vec3, max: &Vec3) {
        self.x = self.x.max(min.x).min(max.x);
        self.y = self.y.max(min.y).min(max.y);
        self.z = self.z.max(min.z).min(max.z);
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn distance(&self, v: &Vec3) -> f32 {
        self.distance_squared(v).sqrt()
    }

    pub fn distance_squared(&self, v: &Vec3) -> f32 {
        (self.x - v.x).powi(2) + (self.y - v.y).powi(2) + (self.z - v.z).powi(2)
    }

    pub fn dot(&self, v: &Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn normalize(&mut self) {
        let n = self.length_squared();
        if n == 1.0 || n == 0.0 {
            return;
        }
        let n = n.sqrt();
        self.x /= n;
        self.y /= n;
        self.z /= n;
    }

    pub fn get_normalized(&self) -> Self {
        let mut v = *self;
        v.normalize();
        v
    }

    pub fn scale(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn set_zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }

    pub fn subtract(&mut self, v: &Vec3) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }

    pub fn smooth(&mut self, target: &Vec3, elapsed_time: f32, response_time: f32) {
        if elapsed_time > 0.0 {
            *self += (*target - *self) * (elapsed_time / (elapsed_time + response_time));
        }
    }
    
    pub fn lerp(&self, target: &Vec3, alpha: f32) -> Vec3 {
        *self * (1.0 - alpha) + *target * alpha
    }
}

// Operator Overloads

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f32) -> Vec3 {
        Vec3 { x: self.x / scalar, y: self.y / scalar, z: self.z / scalar }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}
