use std::f32;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub const ONE: Vec2 = Vec2 { x: 1.0, y: 1.0 };
    pub const UNIT_X: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    pub const UNIT_Y: Vec2 = Vec2 { x: 0.0, y: 1.0 };
    pub const ANCHOR_MIDDLE: Vec2 = Vec2 { x: 0.5, y: 0.5 };
    pub const ANCHOR_BOTTOM_LEFT: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub const ANCHOR_TOP_LEFT: Vec2 = Vec2 { x: 0.0, y: 1.0 };
    pub const ANCHOR_BOTTOM_RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    pub const ANCHOR_TOP_RIGHT: Vec2 = Vec2 { x: 1.0, y: 1.0 };
    pub const ANCHOR_MIDDLE_RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.5 };
    pub const ANCHOR_MIDDLE_LEFT: Vec2 = Vec2 { x: 0.0, y: 0.5 };
    pub const ANCHOR_MIDDLE_TOP: Vec2 = Vec2 { x: 0.5, y: 1.0 };
    pub const ANCHOR_MIDDLE_BOTTOM: Vec2 = Vec2 { x: 0.5, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn from_array(array: &[f32; 2]) -> Self {
        Vec2 {
            x: array[0],
            y: array[1],
        }
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn is_one(&self) -> bool {
        self.x == 1.0 && self.y == 1.0
    }

    pub fn angle(v1: &Vec2, v2: &Vec2) -> f32 {
        let dz = v1.x * v2.y - v1.y * v2.x;
        dz.atan2(v1.dot(v2) + f32::EPSILON)
    }

    pub fn add(&mut self, v: &Vec2) {
        self.x += v.x;
        self.y += v.y;
    }

    pub fn clamp(&mut self, min: &Vec2, max: &Vec2) {
        self.x = self.x.max(min.x).min(max.x);
        self.y = self.y.max(min.y).min(max.y);
    }

    pub fn distance(&self, v: &Vec2) -> f32 {
        ((self.x - v.x).powi(2) + (self.y - v.y).powi(2)).sqrt()
    }

    pub fn distance_squared(&self, v: &Vec2) -> f32 {
        (self.x - v.x).powi(2) + (self.y - v.y).powi(2)
    }

    pub fn dot(&self, v: &Vec2) -> f32 {
        self.x * v.x + self.y * v.y
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    pub fn normalize(&mut self) {
        let n = self.length_squared();
        if n == 1.0 || n == 0.0 {
            return;
        }
        let n = n.sqrt();
        self.x /= n;
        self.y /= n;
    }

    pub fn get_normalized(&self) -> Self {
        let mut v = *self;
        v.normalize();
        v
    }

    pub fn scale(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }

    pub fn scale_vec(&mut self, scale: &Vec2) {
        self.x *= scale.x;
        self.y *= scale.y;
    }

    pub fn rotate(&mut self, point: &Vec2, angle: f32) {
        let sin_angle = angle.sin();
        let cos_angle = angle.cos();

        if point.is_zero() {
            let temp_x = self.x * cos_angle - self.y * sin_angle;
            self.y = self.y * cos_angle + self.x * sin_angle;
            self.x = temp_x;
        } else {
            let temp_x = self.x - point.x;
            let temp_y = self.y - point.y;

            self.x = temp_x * cos_angle - temp_y * sin_angle + point.x;
            self.y = temp_y * cos_angle + temp_x * sin_angle + point.y;
        }
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_zero(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    pub fn subtract(&mut self, v: &Vec2) {
        self.x -= v.x;
        self.y -= v.y;
    }

    pub fn smooth(&mut self, target: &Vec2, elapsed_time: f32, response_time: f32) {
        if elapsed_time > 0.0 {
            *self += (*target - *self) * (elapsed_time / (elapsed_time + response_time));
        }
    }

    pub fn fuzzy_equals(&self, target: &Vec2, variance: f32) -> bool {
        if self.x - variance <= target.x && target.x <= self.x + variance {
            if self.y - variance <= target.y && target.y <= self.y + variance {
                return true;
            }
        }
        false
    }

    pub fn get_angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn cross(&self, other: &Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn get_perp(&self) -> Vec2 {
        Vec2::new(-self.y, self.x)
    }

    pub fn get_r_perp(&self) -> Vec2 {
        Vec2::new(self.y, -self.x)
    }

    pub fn project(&self, other: &Vec2) -> Vec2 {
        *other * (self.dot(other) / other.dot(other))
    }

    pub fn rotate_by_angle(&self, pivot: &Vec2, angle: f32) -> Vec2 {
        let mut res = *self;
        res.rotate(pivot, angle);
        res
    }

    pub fn for_angle(a: f32) -> Vec2 {
        Vec2::new(a.cos(), a.sin())
    }

    pub fn lerp(&self, other: &Vec2, alpha: f32) -> Vec2 {
        *self * (1.0 - alpha) + *other * alpha
    }
}

// Operator Overloads

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

// Element-wise multiplication (if needed, or dot product?)
// C++ didn't seem to have element-wise * for Vec2 * Vec2 in the operator overload section shown,
// but it had `scale(const Vec2& scale)`.
// We'll leave Vec2 * Vec2 undefined for now unless we see it in C++ implementation as element-wise.
// Actually, C++ `inline void scale(const Vec2& scale);` exists.

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, scalar: f32) -> Vec2 {
        Vec2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.0001;

    fn assert_vec2_eq(a: Vec2, b: Vec2) {
        assert!((a.x - b.x).abs() < EPSILON, "x: {} != {}", a.x, b.x);
        assert!((a.y - b.y).abs() < EPSILON, "y: {} != {}", a.y, b.y);
    }

    #[test]
    fn test_vec2_new() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn test_vec2_constants() {
        assert_eq!(Vec2::ZERO, Vec2::new(0.0, 0.0));
        assert_eq!(Vec2::ONE, Vec2::new(1.0, 1.0));
        assert_eq!(Vec2::UNIT_X, Vec2::new(1.0, 0.0));
        assert_eq!(Vec2::UNIT_Y, Vec2::new(0.0, 1.0));
        assert_eq!(Vec2::ANCHOR_MIDDLE, Vec2::new(0.5, 0.5));
    }

    #[test]
    fn test_vec2_from_array() {
        let arr = [5.0, 7.0];
        let v = Vec2::from_array(&arr);
        assert_eq!(v.x, 5.0);
        assert_eq!(v.y, 7.0);
    }

    #[test]
    fn test_vec2_is_zero() {
        assert!(Vec2::ZERO.is_zero());
        assert!(!Vec2::new(0.1, 0.0).is_zero());
        assert!(!Vec2::new(0.0, 0.1).is_zero());
    }

    #[test]
    fn test_vec2_is_one() {
        assert!(Vec2::ONE.is_one());
        assert!(!Vec2::new(1.0, 0.9).is_one());
    }

    #[test]
    fn test_vec2_add() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        let result = a + b;
        assert_eq!(result, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_vec2_add_assign() {
        let mut a = Vec2::new(1.0, 2.0);
        a += Vec2::new(3.0, 4.0);
        assert_eq!(a, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_vec2_sub() {
        let a = Vec2::new(5.0, 7.0);
        let b = Vec2::new(2.0, 3.0);
        let result = a - b;
        assert_eq!(result, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_vec2_sub_assign() {
        let mut a = Vec2::new(5.0, 7.0);
        a -= Vec2::new(2.0, 3.0);
        assert_eq!(a, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_vec2_neg() {
        let v = Vec2::new(3.0, -4.0);
        let result = -v;
        assert_eq!(result, Vec2::new(-3.0, 4.0));
    }

    #[test]
    fn test_vec2_mul_scalar() {
        let v = Vec2::new(2.0, 3.0);
        let result = v * 2.0;
        assert_eq!(result, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_vec2_mul_assign() {
        let mut v = Vec2::new(2.0, 3.0);
        v *= 2.0;
        assert_eq!(v, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_vec2_div_scalar() {
        let v = Vec2::new(6.0, 8.0);
        let result = v / 2.0;
        assert_eq!(result, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_vec2_div_assign() {
        let mut v = Vec2::new(6.0, 8.0);
        v /= 2.0;
        assert_eq!(v, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_vec2_length() {
        let v = Vec2::new(3.0, 4.0);
        assert!((v.length() - 5.0).abs() < EPSILON);
    }

    #[test]
    fn test_vec2_length_squared() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.length_squared(), 25.0);
    }

    #[test]
    fn test_vec2_normalize() {
        let mut v = Vec2::new(3.0, 4.0);
        v.normalize();
        assert!((v.length() - 1.0).abs() < EPSILON);
        assert_vec2_eq(v, Vec2::new(0.6, 0.8));
    }

    #[test]
    fn test_vec2_normalize_zero() {
        let mut v = Vec2::ZERO;
        v.normalize(); // Should not panic
        assert_eq!(v, Vec2::ZERO); // Zero vector stays zero
    }

    #[test]
    fn test_vec2_get_normalized() {
        let v = Vec2::new(3.0, 4.0);
        let normalized = v.get_normalized();
        assert!((normalized.length() - 1.0).abs() < EPSILON);
        assert_eq!(v, Vec2::new(3.0, 4.0)); // Original unchanged
    }

    #[test]
    fn test_vec2_dot() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a.dot(&b), 11.0); // 1*3 + 2*4 = 11
    }

    #[test]
    fn test_vec2_cross() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a.cross(&b), -2.0); // 1*4 - 2*3 = -2
    }

    #[test]
    fn test_vec2_distance() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(3.0, 4.0);
        assert!((a.distance(&b) - 5.0).abs() < EPSILON);
    }

    #[test]
    fn test_vec2_distance_squared() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a.distance_squared(&b), 25.0);
    }

    #[test]
    fn test_vec2_angle() {
        let a = Vec2::UNIT_X;
        let b = Vec2::UNIT_Y;
        let angle = Vec2::angle(&a, &b);
        assert!((angle - std::f32::consts::FRAC_PI_2).abs() < EPSILON);
    }

    #[test]
    fn test_vec2_clamp() {
        let mut v = Vec2::new(5.0, -2.0);
        v.clamp(&Vec2::new(0.0, 0.0), &Vec2::new(3.0, 3.0));
        assert_eq!(v, Vec2::new(3.0, 0.0));
    }

    #[test]
    fn test_vec2_scale() {
        let mut v = Vec2::new(2.0, 3.0);
        v.scale(2.5);
        assert_vec2_eq(v, Vec2::new(5.0, 7.5));
    }

    #[test]
    fn test_vec2_scale_vec() {
        let mut v = Vec2::new(2.0, 3.0);
        v.scale_vec(&Vec2::new(2.0, 3.0));
        assert_vec2_eq(v, Vec2::new(4.0, 9.0));
    }

    #[test]
    fn test_vec2_rotate() {
        let mut v = Vec2::new(1.0, 0.0);
        v.rotate(&Vec2::ZERO, std::f32::consts::FRAC_PI_2);
        assert_vec2_eq(v, Vec2::new(0.0, 1.0));
    }

    #[test]
    fn test_vec2_get_perp() {
        let v = Vec2::new(3.0, 4.0);
        let perp = v.get_perp();
        assert_eq!(perp, Vec2::new(-4.0, 3.0));
        // Verify it's perpendicular
        assert!((v.dot(&perp)).abs() < EPSILON);
    }

    #[test]
    fn test_vec2_get_r_perp() {
        let v = Vec2::new(3.0, 4.0);
        let r_perp = v.get_r_perp();
        assert_eq!(r_perp, Vec2::new(4.0, -3.0));
        assert!((v.dot(&r_perp)).abs() < EPSILON);
    }

    #[test]
    fn test_vec2_project() {
        let a = Vec2::new(3.0, 4.0);
        let b = Vec2::UNIT_X;
        let projected = a.project(&b);
        assert_vec2_eq(projected, Vec2::new(3.0, 0.0));
    }

    #[test]
    fn test_vec2_lerp() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(10.0, 10.0);
        let mid = a.lerp(&b, 0.5);
        assert_vec2_eq(mid, Vec2::new(5.0, 5.0));
    }

    #[test]
    fn test_vec2_for_angle() {
        let v = Vec2::for_angle(0.0);
        assert_vec2_eq(v, Vec2::UNIT_X);
        
        let v = Vec2::for_angle(std::f32::consts::FRAC_PI_2);
        assert_vec2_eq(v, Vec2::UNIT_Y);
    }

    #[test]
    fn test_vec2_get_angle() {
        let v = Vec2::UNIT_X;
        assert!((v.get_angle() - 0.0).abs() < EPSILON);
        
        let v = Vec2::UNIT_Y;
        assert!((v.get_angle() - std::f32::consts::FRAC_PI_2).abs() < EPSILON);
    }

    #[test]
    fn test_vec2_fuzzy_equals() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(1.01, 2.01);
        
        assert!(a.fuzzy_equals(&b, 0.02));
        assert!(!a.fuzzy_equals(&b, 0.005));
    }

    #[test]
    fn test_vec2_smooth() {
        let mut current = Vec2::new(0.0, 0.0);
        let target = Vec2::new(10.0, 10.0);
        
        current.smooth(&target, 0.5, 1.0);
        
        // Should move towards target but not reach it
        assert!(current.x > 0.0 && current.x < 10.0);
        assert!(current.y > 0.0 && current.y < 10.0);
    }

    #[test]
    fn test_vec2_set() {
        let mut v = Vec2::ZERO;
        v.set(5.0, 7.0);
        assert_eq!(v, Vec2::new(5.0, 7.0));
    }

    #[test]
    fn test_vec2_set_zero() {
        let mut v = Vec2::new(5.0, 7.0);
        v.set_zero();
        assert_eq!(v, Vec2::ZERO);
    }
}
