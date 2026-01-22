use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::f32;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const ZERO: Vec4 = Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const ONE: Vec4 = Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    pub const UNIT_X: Vec4 = Vec4 { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const UNIT_Y: Vec4 = Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 };
    pub const UNIT_Z: Vec4 = Vec4 { x: 0.0, y: 0.0, z: 1.0, w: 0.0 };
    pub const UNIT_W: Vec4 = Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }
    
    pub fn from_array(array: &[f32; 4]) -> Self {
        Vec4 { x: array[0], y: array[1], z: array[2], w: array[3] }
    }

    pub fn from_color(color: u32) -> Self {
        let r = ((color >> 24) & 0xFF) as f32 / 255.0;
        let g = ((color >> 16) & 0xFF) as f32 / 255.0;
        let b = ((color >> 8) & 0xFF) as f32 / 255.0;
        let a = (color & 0xFF) as f32 / 255.0;
        Vec4::new(r, g, b, a)
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0 && self.w == 0.0
    }

    pub fn is_one(&self) -> bool {
        self.x == 1.0 && self.y == 1.0 && self.z == 1.0 && self.w == 1.0
    }

    pub fn angle(v1: &Vec4, v2: &Vec4) -> f32 {
        let dot = v1.dot(v2);
        let len_sq = v1.length_squared() * v2.length_squared();
        (dot / len_sq.sqrt()).acos()
    }

    pub fn add(&mut self, v: &Vec4) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self.w += v.w;
    }

    pub fn clamp(&mut self, min: &Vec4, max: &Vec4) {
        self.x = self.x.max(min.x).min(max.x);
        self.y = self.y.max(min.y).min(max.y);
        self.z = self.z.max(min.z).min(max.z);
        self.w = self.w.max(min.w).min(max.w);
    }

    pub fn distance(&self, v: &Vec4) -> f32 {
        self.distance_squared(v).sqrt()
    }

    pub fn distance_squared(&self, v: &Vec4) -> f32 {
        (self.x - v.x).powi(2) + (self.y - v.y).powi(2) + (self.z - v.z).powi(2) + (self.w - v.w).powi(2)
    }

    pub fn dot(&self, v: &Vec4) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)
    }

    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self.w = -self.w;
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
        self.w /= n;
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
        self.w *= scalar;
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }

    pub fn subtract(&mut self, v: &Vec4) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
        self.w -= v.w;
    }
}

// Operator Overloads

impl Add for Vec4 {
    type Output = Vec4;
    fn add(self, other: Vec4) -> Vec4 {
        Vec4 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, w: self.w + other.w }
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Vec4) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl Sub for Vec4 {
    type Output = Vec4;
    fn sub(self, other: Vec4) -> Vec4 {
        Vec4 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, w: self.w - other.w }
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, other: Vec4) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl Neg for Vec4 {
    type Output = Vec4;
    fn neg(self) -> Vec4 {
        Vec4 { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;
    fn mul(self, scalar: f32) -> Vec4 {
        Vec4 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar, w: self.w * scalar }
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;
    fn div(self, scalar: f32) -> Vec4 {
        Vec4 { x: self.x / scalar, y: self.y / scalar, z: self.z / scalar, w: self.w / scalar }
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
        self.w /= scalar;
    }
}
