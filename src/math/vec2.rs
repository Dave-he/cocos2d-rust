use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::f32;

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
        Vec2 { x: array[0], y: array[1] }
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
        Vec2 { x: self.x + other.x, y: self.y + other.y }
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
        Vec2 { x: self.x - other.x, y: self.y - other.y }
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
        Vec2 { x: -self.x, y: -self.y }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2 { x: self.x * scalar, y: self.y * scalar }
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
        Vec2 { x: self.x / scalar, y: self.y / scalar }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}
