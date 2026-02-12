// 简化的数学库测试 - 只测试 Vec2 和 Mat4 的内置测试
// 这些测试在源文件中已经存在并且应该可以工作

#[cfg(test)]
mod vec2_integration_tests {
    use cocos2d_rust::math::Vec2;
    
    const EPSILON: f32 = 0.0001;
    
    fn assert_float_near(a: f32, b: f32, epsilon: f32) {
        assert!((a - b).abs() < epsilon, "Expected {}, got {}", b, a);
    }
    
    #[test]
    fn test_vec2_basic_operations() {
        let a = Vec2::new(3.0, 4.0);
        let b = Vec2::new(1.0, 2.0);
        
        // 加法
        let sum = a + b;
        assert_eq!(sum.x, 4.0);
        assert_eq!(sum.y, 6.0);
        
        // 减法
        let diff = a - b;
        assert_eq!(diff.x, 2.0);
        assert_eq!(diff.y, 2.0);
        
        // 标量乘法
        let scaled = a * 2.0;
        assert_eq!(scaled.x, 6.0);
        assert_eq!(scaled.y, 8.0);
    }
    
    #[test]
    fn test_vec2_length() {
        let v = Vec2::new(3.0, 4.0);
        assert_float_near(v.length(), 5.0, EPSILON);
        assert_eq!(v.length_squared(), 25.0);
    }
    
    #[test]
    fn test_vec2_normalize() {
        let v = Vec2::new(3.0, 4.0);
        let normalized = v.get_normalized();
        assert_float_near(normalized.length(), 1.0, EPSILON);
    }
    
    #[test]
    fn test_vec2_dot_product() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);
        assert_float_near(a.dot(&b), 0.0, EPSILON);
        
        let c = Vec2::new(2.0, 3.0);
        let d = Vec2::new(4.0, 5.0);
        assert_eq!(c.dot(&d), 23.0);
    }
    
    #[test]
    fn test_vec2_constants() {
        assert_eq!(Vec2::ZERO, Vec2::new(0.0, 0.0));
        assert_eq!(Vec2::ONE, Vec2::new(1.0, 1.0));
        assert_eq!(Vec2::UNIT_X, Vec2::new(1.0, 0.0));
        assert_eq!(Vec2::UNIT_Y, Vec2::new(0.0, 1.0));
    }
}

#[cfg(test)]
mod vec3_integration_tests {
    use cocos2d_rust::math::Vec3;
    
    #[test]
    fn test_vec3_basic() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        
        let sum = a + b;
        assert_eq!(sum, Vec3::new(5.0, 7.0, 9.0));
    }
    
    #[test]
    fn test_vec3_cross_product() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let cross = a.cross(&b);
        
        assert_eq!(cross.x, -3.0);
        assert_eq!(cross.y, 6.0);
        assert_eq!(cross.z, -3.0);
    }
}

#[cfg(test)]
mod mat4_integration_tests {
    use cocos2d_rust::math::{Mat4, Vec3, Quaternion};
    
    const EPSILON: f32 = 0.001;
    
    #[test]
    fn test_mat4_identity() {
        let m = Mat4::IDENTITY;
        assert!(m.is_identity());
    }
    
    #[test]
    fn test_mat4_translation() {
        let m = Mat4::create_translation(&Vec3::new(10.0, 20.0, 30.0));
        let point = Vec3::new(1.0, 2.0, 3.0);
        let result = m.transform_point(&point);
        
        assert_eq!(result, Vec3::new(11.0, 22.0, 33.0));
    }
    
    #[test]
    fn test_mat4_scaling() {
        let m = Mat4::create_scale(&Vec3::new(2.0, 3.0, 4.0));
        let point = Vec3::new(1.0, 1.0, 1.0);
        let result = m.transform_point(&point);
        
        assert_eq!(result, Vec3::new(2.0, 3.0, 4.0));
    }
    
    #[test]
    fn test_mat4_multiplication() {
        let t = Mat4::create_translation(&Vec3::new(10.0, 0.0, 0.0));
        let s = Mat4::create_scale(&Vec3::new(2.0, 2.0, 2.0));
        let combined = t * s;
        
        let v = Vec3::new(1.0, 1.0, 1.0);
        let result = combined * v;
        
        assert_eq!(result.x, 12.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 2.0);
    }
}
