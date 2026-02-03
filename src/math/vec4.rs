use std::f32;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const ZERO: Vec4 = Vec4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    pub const ONE: Vec4 = Vec4 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        w: 1.0,
    };
    pub const UNIT_X: Vec4 = Vec4 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    pub const UNIT_Y: Vec4 = Vec4 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        w: 0.0,
    };
    pub const UNIT_Z: Vec4 = Vec4 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
        w: 0.0,
    };
    pub const UNIT_W: Vec4 = Vec4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }

    pub fn from_array(array: &[f32; 4]) -> Self {
        Vec4 {
            x: array[0],
            y: array[1],
            z: array[2],
            w: array[3],
        }
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
        (self.x - v.x).powi(2)
            + (self.y - v.y).powi(2)
            + (self.z - v.z).powi(2)
            + (self.w - v.w).powi(2)
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
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
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
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
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
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;
    fn mul(self, scalar: f32) -> Vec4 {
        Vec4 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
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
        Vec4 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.0001;

    fn assert_vec4_eq(a: Vec4, b: Vec4) {
        assert!((a.x - b.x).abs() < EPSILON, "x: {} != {}", a.x, b.x);
        assert!((a.y - b.y).abs() < EPSILON, "y: {} != {}", a.y, b.y);
        assert!((a.z - b.z).abs() < EPSILON, "z: {} != {}", a.z, b.z);
        assert!((a.w - b.w).abs() < EPSILON, "w: {} != {}", a.w, b.w);
    }

    #[test]
    fn test_vec4_new() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
        assert_eq!(v.w, 4.0);
    }

    #[test]
    fn test_vec4_constants() {
        assert_eq!(Vec4::ZERO, Vec4::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(Vec4::ONE, Vec4::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(Vec4::UNIT_X, Vec4::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(Vec4::UNIT_Y, Vec4::new(0.0, 1.0, 0.0, 0.0));
        assert_eq!(Vec4::UNIT_Z, Vec4::new(0.0, 0.0, 1.0, 0.0));
        assert_eq!(Vec4::UNIT_W, Vec4::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vec4_from_array() {
        let arr = [1.0, 2.0, 3.0, 4.0];
        let v = Vec4::from_array(&arr);
        assert_eq!(v, Vec4::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn test_vec4_from_color() {
        let color = 0xFF00_80FF; // RGBA: R=255, G=0, B=128, A=255
        let v = Vec4::from_color(color);
        assert!((v.x - 1.0).abs() < EPSILON);
        assert!((v.y - 0.0).abs() < EPSILON);
        assert!((v.z - 0.50196).abs() < 0.01);
        assert!((v.w - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_vec4_is_zero() {
        assert!(Vec4::ZERO.is_zero());
        assert!(!Vec4::new(0.1, 0.0, 0.0, 0.0).is_zero());
    }

    #[test]
    fn test_vec4_is_one() {
        assert!(Vec4::ONE.is_one());
        assert!(!Vec4::new(1.0, 1.0, 1.0, 0.9).is_one());
    }

    #[test]
    fn test_vec4_add() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);
        assert_eq!(a + b, Vec4::new(6.0, 8.0, 10.0, 12.0));
    }

    #[test]
    fn test_vec4_add_assign() {
        let mut a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        a += Vec4::new(5.0, 6.0, 7.0, 8.0);
        assert_eq!(a, Vec4::new(6.0, 8.0, 10.0, 12.0));
    }

    #[test]
    fn test_vec4_sub() {
        let a = Vec4::new(10.0, 8.0, 6.0, 4.0);
        let b = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(a - b, Vec4::new(9.0, 6.0, 3.0, 0.0));
    }

    #[test]
    fn test_vec4_sub_assign() {
        let mut a = Vec4::new(10.0, 8.0, 6.0, 4.0);
        a -= Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(a, Vec4::new(9.0, 6.0, 3.0, 0.0));
    }

    #[test]
    fn test_vec4_neg() {
        let v = Vec4::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-v, Vec4::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn test_vec4_mul_scalar() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v * 2.0, Vec4::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn test_vec4_mul_assign() {
        let mut v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        v *= 2.0;
        assert_eq!(v, Vec4::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn test_vec4_div_scalar() {
        let v = Vec4::new(10.0, 8.0, 6.0, 4.0);
        assert_eq!(v / 2.0, Vec4::new(5.0, 4.0, 3.0, 2.0));
    }

    #[test]
    fn test_vec4_div_assign() {
        let mut v = Vec4::new(10.0, 8.0, 6.0, 4.0);
        v /= 2.0;
        assert_eq!(v, Vec4::new(5.0, 4.0, 3.0, 2.0));
    }

    #[test]
    fn test_vec4_length() {
        let v = Vec4::new(1.0, 2.0, 2.0, 2.0); // sqrt(1+4+4+4) = sqrt(13)
        assert!((v.length() - 3.6055).abs() < 0.01);
    }

    #[test]
    fn test_vec4_length_squared() {
        let v = Vec4::new(1.0, 2.0, 2.0, 2.0);
        assert_eq!(v.length_squared(), 13.0);
    }

    #[test]
    fn test_vec4_normalize() {
        let mut v = Vec4::new(1.0, 2.0, 2.0, 2.0);
        v.normalize();
        assert!((v.length() - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_vec4_normalize_zero() {
        let mut v = Vec4::ZERO;
        v.normalize();
        assert_eq!(v, Vec4::ZERO);
    }

    #[test]
    fn test_vec4_dot() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);
        assert_eq!(a.dot(&b), 70.0); // 1*5 + 2*6 + 3*7 + 4*8 = 70
    }

    #[test]
    fn test_vec4_distance() {
        let a = Vec4::ZERO;
        let b = Vec4::new(1.0, 2.0, 2.0, 2.0);
        assert!((a.distance(&b) - 3.6055).abs() < 0.01);
    }

    #[test]
    fn test_vec4_distance_squared() {
        let a = Vec4::ZERO;
        let b = Vec4::new(1.0, 2.0, 2.0, 2.0);
        assert_eq!(a.distance_squared(&b), 13.0);
    }

    #[test]
    fn test_vec4_angle() {
        let a = Vec4::UNIT_X;
        let b = Vec4::UNIT_X;
        let angle = Vec4::angle(&a, &b);
        assert!((angle - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_vec4_scale() {
        let mut v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        v.scale(2.0);
        assert_eq!(v, Vec4::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn test_vec4_clamp() {
        let mut v = Vec4::new(10.0, -5.0, 15.0, 20.0);
        v.clamp(&Vec4::ZERO, &Vec4::new(5.0, 5.0, 5.0, 5.0));
        assert_eq!(v, Vec4::new(5.0, 0.0, 5.0, 5.0));
    }

    #[test]
    fn test_vec4_set() {
        let mut v = Vec4::ZERO;
        v.set(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v, Vec4::new(1.0, 2.0, 3.0, 4.0));
    }
}
