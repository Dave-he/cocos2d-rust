use std::ops::{Mul, MulAssign, Add, AddAssign, Sub, SubAssign, Neg};
use std::f32;
use crate::math::{Vec3, Vec4, Quaternion};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4 {
    pub m: [f32; 16],
}

impl Mat4 {
    pub const IDENTITY: Mat4 = Mat4 {
        m: [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ],
    };
    
    pub const ZERO: Mat4 = Mat4 {
        m: [0.0; 16],
    };

    pub fn new(m11: f32, m12: f32, m13: f32, m14: f32,
               m21: f32, m22: f32, m23: f32, m24: f32,
               m31: f32, m32: f32, m33: f32, m34: f32,
               m41: f32, m42: f32, m43: f32, m44: f32) -> Self {
        Mat4 {
            m: [
                m11, m21, m31, m41,
                m12, m22, m32, m42,
                m13, m23, m33, m43,
                m14, m24, m34, m44, // stored in column-major order to match OpenGL/Cocos2d
            ],
        }
    }
    
    pub fn from_array(mat: &[f32; 16]) -> Self {
        Mat4 { m: *mat }
    }

    pub fn is_identity(&self) -> bool {
        *self == Mat4::IDENTITY
    }

    pub fn set_identity(&mut self) {
        *self = Mat4::IDENTITY;
    }

    pub fn set_zero(&mut self) {
        *self = Mat4::ZERO;
    }

    pub fn multiply(&mut self, mat: &Mat4) {
        let product = *self * *mat;
        *self = product;
    }
    
    pub fn create_look_at(eye: &Vec3, target: &Vec3, up: &Vec3) -> Mat4 {
        let mut z_axis = *eye - *target;
        z_axis.normalize();
        
        let mut x_axis = up.cross(&z_axis);
        x_axis.normalize();
        
        let mut y_axis = z_axis.cross(&x_axis);
        y_axis.normalize();
        
        let mut m = Mat4::IDENTITY;
        m.m[0] = x_axis.x;
        m.m[1] = y_axis.x;
        m.m[2] = z_axis.x;
        
        m.m[4] = x_axis.y;
        m.m[5] = y_axis.y;
        m.m[6] = z_axis.y;
        
        m.m[8] = x_axis.z;
        m.m[9] = y_axis.z;
        m.m[10] = z_axis.z;
        
        m.m[12] = -x_axis.dot(eye);
        m.m[13] = -y_axis.dot(eye);
        m.m[14] = -z_axis.dot(eye);
        
        m
    }

    pub fn create_perspective(field_of_view: f32, aspect_ratio: f32, z_near_plane: f32, z_far_plane: f32) -> Mat4 {
        let mut m = Mat4::ZERO;
        let f = 1.0 / (field_of_view.to_radians() / 2.0).tan();
        
        m.m[0] = f / aspect_ratio;
        m.m[5] = f;
        m.m[10] = (z_far_plane + z_near_plane) / (z_near_plane - z_far_plane);
        m.m[11] = -1.0;
        m.m[14] = (2.0 * z_far_plane * z_near_plane) / (z_near_plane - z_far_plane);
        
        m
    }
    
    pub fn create_orthographic(width: f32, height: f32, z_near_plane: f32, z_far_plane: f32) -> Mat4 {
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        Mat4::create_orthographic_off_center(-half_width, half_width, -half_height, half_height, z_near_plane, z_far_plane)
    }

    pub fn create_orthographic_off_center(left: f32, right: f32, bottom: f32, top: f32,
                                            z_near_plane: f32, z_far_plane: f32) -> Mat4 {
        let mut m = Mat4::ZERO;
        
        m.m[0] = 2.0 / (right - left);
        m.m[5] = 2.0 / (top - bottom);
        m.m[10] = -2.0 / (z_far_plane - z_near_plane);
        m.m[12] = -(right + left) / (right - left);
        m.m[13] = -(top + bottom) / (top - bottom);
        m.m[14] = -(z_far_plane + z_near_plane) / (z_far_plane - z_near_plane);
        m.m[15] = 1.0;
        
        m
    }
    
    pub fn create_translation(translation: &Vec3) -> Mat4 {
        let mut m = Mat4::IDENTITY;
        m.m[12] = translation.x;
        m.m[13] = translation.y;
        m.m[14] = translation.z;
        m
    }
    
    pub fn create_scale(scale: &Vec3) -> Mat4 {
        let mut m = Mat4::IDENTITY;
        m.m[0] = scale.x;
        m.m[5] = scale.y;
        m.m[10] = scale.z;
        m
    }
    
    pub fn create_rotation(quat: &Quaternion) -> Mat4 {
        let x = quat.x;
        let y = quat.y;
        let z = quat.z;
        let w = quat.w;
        
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let xz = x * z;
        let xw = x * w;
        let yz = y * z;
        let yw = y * w;
        let zw = z * w;
        
        let mut m = Mat4::IDENTITY;
        
        m.m[0] = 1.0 - 2.0 * (yy + zz);
        m.m[1] = 2.0 * (xy + zw);
        m.m[2] = 2.0 * (xz - yw);
        
        m.m[4] = 2.0 * (xy - zw);
        m.m[5] = 1.0 - 2.0 * (xx + zz);
        m.m[6] = 2.0 * (yz + xw);
        
        m.m[8] = 2.0 * (xz + yw);
        m.m[9] = 2.0 * (yz - xw);
        m.m[10] = 1.0 - 2.0 * (xx + yy);
        
        m
    }
    
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
         let t = Mat4::create_translation(&Vec3::new(x, y, z));
         self.multiply(&t);
    }
    
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
         let s = Mat4::create_scale(&Vec3::new(x, y, z));
         self.multiply(&s);
    }
    
    pub fn rotate(&mut self, quat: &Quaternion) {
        let r = Mat4::create_rotation(quat);
        self.multiply(&r);
    }
    
    pub fn transform_point(&self, point: &Vec3) -> Vec3 {
        let x = point.x;
        let y = point.y;
        let z = point.z;
        
        Vec3 {
            x: x * self.m[0] + y * self.m[4] + z * self.m[8] + self.m[12],
            y: x * self.m[1] + y * self.m[5] + z * self.m[9] + self.m[13],
            z: x * self.m[2] + y * self.m[6] + z * self.m[10] + self.m[14],
        }
    }
    
    pub fn transform_vector(&self, vector: &Vec3) -> Vec3 {
         let x = vector.x;
        let y = vector.y;
        let z = vector.z;
        
        Vec3 {
            x: x * self.m[0] + y * self.m[4] + z * self.m[8],
            y: x * self.m[1] + y * self.m[5] + z * self.m[9],
            z: x * self.m[2] + y * self.m[6] + z * self.m[10],
        }
    }
}

// Operator Overloads

impl Add for Mat4 {
    type Output = Mat4;
    fn add(self, other: Mat4) -> Mat4 {
        let mut m = Mat4::ZERO;
        for i in 0..16 {
            m.m[i] = self.m[i] + other.m[i];
        }
        m
    }
}

impl Sub for Mat4 {
    type Output = Mat4;
    fn sub(self, other: Mat4) -> Mat4 {
        let mut m = Mat4::ZERO;
        for i in 0..16 {
            m.m[i] = self.m[i] - other.m[i];
        }
        m
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, other: Mat4) -> Mat4 {
        let mut dst = Mat4::ZERO;
        let m1 = self.m;
        let m2 = other.m;
        
        // Col 0
        dst.m[0]  = m1[0] * m2[0]  + m1[4] * m2[1]  + m1[8]  * m2[2]  + m1[12] * m2[3];
        dst.m[1]  = m1[1] * m2[0]  + m1[5] * m2[1]  + m1[9]  * m2[2]  + m1[13] * m2[3];
        dst.m[2]  = m1[2] * m2[0]  + m1[6] * m2[1]  + m1[10] * m2[2]  + m1[14] * m2[3];
        dst.m[3]  = m1[3] * m2[0]  + m1[7] * m2[1]  + m1[11] * m2[2]  + m1[15] * m2[3];

        // Col 1
        dst.m[4]  = m1[0] * m2[4]  + m1[4] * m2[5]  + m1[8]  * m2[6]  + m1[12] * m2[7];
        dst.m[5]  = m1[1] * m2[4]  + m1[5] * m2[5]  + m1[9]  * m2[6]  + m1[13] * m2[7];
        dst.m[6]  = m1[2] * m2[4]  + m1[6] * m2[5]  + m1[10] * m2[6]  + m1[14] * m2[7];
        dst.m[7]  = m1[3] * m2[4]  + m1[7] * m2[5]  + m1[11] * m2[6]  + m1[15] * m2[7];

        // Col 2
        dst.m[8]  = m1[0] * m2[8]  + m1[4] * m2[9]  + m1[8]  * m2[10] + m1[12] * m2[11];
        dst.m[9]  = m1[1] * m2[8]  + m1[5] * m2[9]  + m1[9]  * m2[10] + m1[13] * m2[11];
        dst.m[10] = m1[2] * m2[8]  + m1[6] * m2[9]  + m1[10] * m2[10] + m1[14] * m2[11];
        dst.m[11] = m1[3] * m2[8]  + m1[7] * m2[9]  + m1[11] * m2[10] + m1[15] * m2[11];

        // Col 3
        dst.m[12] = m1[0] * m2[12] + m1[4] * m2[13] + m1[8]  * m2[14] + m1[12] * m2[15];
        dst.m[13] = m1[1] * m2[12] + m1[5] * m2[13] + m1[9]  * m2[14] + m1[13] * m2[15];
        dst.m[14] = m1[2] * m2[12] + m1[6] * m2[13] + m1[10] * m2[14] + m1[14] * m2[15];
        dst.m[15] = m1[3] * m2[12] + m1[7] * m2[13] + m1[11] * m2[14] + m1[15] * m2[15];
        
        dst
    }
}

impl MulAssign<Mat4> for Mat4 {
    fn mul_assign(&mut self, other: Mat4) {
        *self = *self * other;
    }
}

// Transform Vec3 (treating as point w=1) - NOTE: This matches transformPoint behavior usually
impl Mul<Vec3> for Mat4 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        self.transform_point(&v)
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, v: Vec4) -> Vec4 {
         Vec4 {
            x: v.x * self.m[0] + v.y * self.m[4] + v.z * self.m[8]  + v.w * self.m[12],
            y: v.x * self.m[1] + v.y * self.m[5] + v.z * self.m[9]  + v.w * self.m[13],
            z: v.x * self.m[2] + v.y * self.m[6] + v.z * self.m[10] + v.w * self.m[14],
            w: v.x * self.m[3] + v.y * self.m[7] + v.z * self.m[11] + v.w * self.m[15],
        }
    }
}
