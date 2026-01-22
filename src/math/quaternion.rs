use std::ops::{Mul, MulAssign};
use std::f32;
use crate::math::Vec3;
// use crate::math::Mat4; // Cyclic dependency handling will be needed

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub const ZERO: Quaternion = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const IDENTITY: Quaternion = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub fn identity() -> Self {
        Self::IDENTITY
    }

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Quaternion { x, y, z, w }
    }
    
    pub fn from_array(array: &[f32; 4]) -> Self {
        Quaternion { x: array[0], y: array[1], z: array[2], w: array[3] }
    }
    
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let half_angle = angle * 0.5;
        let sin_half_angle = half_angle.sin();
        
        let mut normal_axis = axis;
        normal_axis.normalize();
        
        Quaternion {
            x: normal_axis.x * sin_half_angle,
            y: normal_axis.y * sin_half_angle,
            z: normal_axis.z * sin_half_angle,
            w: half_angle.cos(),
        }
    }

    pub fn is_identity(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0 && self.w == 1.0
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0 && self.w == 0.0
    }
    
    pub fn conjugate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn get_conjugated(&self) -> Self {
        Quaternion { x: -self.x, y: -self.y, z: -self.z, w: self.w }
    }

    pub fn inverse(&mut self) -> bool {
        let n = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        if n == 1.0 {
            self.conjugate();
            return true;
        }
        if n < f32::EPSILON {
            return false;
        }
        let n_inv = 1.0 / n;
        self.x = -self.x * n_inv;
        self.y = -self.y * n_inv;
        self.z = -self.z * n_inv;
        self.w = self.w * n_inv;
        true
    }

    pub fn get_inversed(&self) -> Self {
        let mut q = *self;
        q.inverse();
        q
    }

    pub fn multiply(&mut self, q: &Quaternion) {
        let x = self.w * q.x + self.x * q.w + self.y * q.z - self.z * q.y;
        let y = self.w * q.y - self.x * q.z + self.y * q.w + self.z * q.x;
        let z = self.w * q.z + self.x * q.y - self.y * q.x + self.z * q.w;
        let w = self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z;
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }

    pub fn normalize(&mut self) {
        let n = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
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
        let mut q = *self;
        q.normalize();
        q
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }
    
    pub fn to_axis_angle(&self) -> (Vec3, f32) {
        let mut q = *self;
        if q.w > 1.0 {
            q.normalize();
        }
        
        let angle = 2.0 * q.w.acos();
        let s = (1.0 - q.w * q.w).sqrt();
        
        if s < 0.001 { // Divide by zero check
             (Vec3::new(q.x, q.y, q.z), angle) // x,y,z should be 0 if s is 0? C++ impl usually defaults to x=1 or similar if angle is 0
        } else {
             (Vec3::new(q.x / s, q.y / s, q.z / s), angle)
        }
    }

    pub fn lerp(q1: &Quaternion, q2: &Quaternion, t: f32) -> Quaternion {
        let mut q = Quaternion {
            x: q1.x * (1.0 - t) + q2.x * t,
            y: q1.y * (1.0 - t) + q2.y * t,
            z: q1.z * (1.0 - t) + q2.z * t,
            w: q1.w * (1.0 - t) + q2.w * t,
        };
        q.normalize();
        q
    }

    pub fn slerp(q1: &Quaternion, q2: &Quaternion, t: f32) -> Quaternion {
        let mut q2 = *q2;
        let mut dot = q1.x * q2.x + q1.y * q2.y + q1.z * q2.z + q1.w * q2.w;
        
        if dot < 0.0 {
            dot = -dot;
            q2.x = -q2.x;
            q2.y = -q2.y;
            q2.z = -q2.z;
            q2.w = -q2.w;
        }
        
        if dot > 0.9995 {
             return Quaternion::lerp(q1, &q2, t);
        }
        
        let theta_0 = dot.acos();
        let theta = theta_0 * t;
        let sin_theta = theta.sin();
        let sin_theta_0 = theta_0.sin();
        
        let s1 = (theta_0 - theta).sin() / sin_theta_0; // wait, standard formula is different?
        // Standard: sin((1-t)*theta)/sin(theta) * q1 + sin(t*theta)/sin(theta) * q2
        
        let s0 = ((1.0 - t) * theta_0).sin() / sin_theta_0;
        let s1 = (t * theta_0).sin() / sin_theta_0;
        
        Quaternion {
            x: q1.x * s0 + q2.x * s1,
            y: q1.y * s0 + q2.y * s1,
            z: q1.z * s0 + q2.z * s1,
            w: q1.w * s0 + q2.w * s1,
        }
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: Quaternion) -> Quaternion {
        let mut q = self;
        q.multiply(&other);
        q
    }
}

impl MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, other: Quaternion) {
        self.multiply(&other);
    }
}

// Mul with Vec3 (Rotation)
impl Mul<Vec3> for Quaternion {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        // q * v * q^-1
        let vec_quat = Quaternion::new(v.x, v.y, v.z, 0.0);
        let mut res = self * vec_quat;
        res.multiply(&self.get_conjugated());
        Vec3::new(res.x, res.y, res.z)
    }
}
