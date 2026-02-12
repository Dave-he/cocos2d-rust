use cocos2d_rust::math::{Vec2, Vec3, Vec4, Mat4, Quaternion};

const EPSILON: f32 = 0.0001;

fn assert_float_eq(a: f32, b: f32, message: &str) {
    assert!((a - b).abs() < EPSILON, "{}: {} != {}", message, a, b);
}

fn assert_float_near(a: f32, b: f32, epsilon: f32, message: &str) {
    assert!((a - b).abs() < epsilon, "{}: {} != {} (epsilon: {})", message, a, b, epsilon);
}

#[test]
fn test_vec2_arithmetic_operations() {
    let a = Vec2::new(3.0, 4.0);
    let b = Vec2::new(1.0, 2.0);
    
    let sum = a + b;
    assert_float_eq(sum.x, 4.0, "Vec2 addition x");
    assert_float_eq(sum.y, 6.0, "Vec2 addition y");
    
    let diff = a - b;
    assert_float_eq(diff.x, 2.0, "Vec2 subtraction x");
    assert_float_eq(diff.y, 2.0, "Vec2 subtraction y");
    
    let scaled = a * 2.0;
    assert_float_eq(scaled.x, 6.0, "Vec2 scaling x");
    assert_float_eq(scaled.y, 8.0, "Vec2 scaling y");
}

#[test]
fn test_vec2_length_and_normalization() {
    let v = Vec2::new(3.0, 4.0);
    
    assert_float_eq(v.length(), 5.0, "Vec2 length");
    assert_float_eq(v.length_squared(), 25.0, "Vec2 length squared");
    
    let normalized = v.get_normalized();
    assert_float_eq(normalized.length(), 1.0, "Normalized length");
    assert_float_eq(normalized.x, 0.6, "Normalized x");
    assert_float_eq(normalized.y, 0.8, "Normalized y");
}

#[test]
fn test_vec2_dot_and_cross_product() {
    let a = Vec2::new(1.0, 0.0);
    let b = Vec2::new(0.0, 1.0);
    
    assert_float_eq(a.dot(&b), 0.0, "Dot product perpendicular");
    assert_float_eq(a.cross(&b), 1.0, "Cross product");
    
    let c = Vec2::new(2.0, 3.0);
    let d = Vec2::new(4.0, 5.0);
    assert_float_eq(c.dot(&d), 23.0, "Dot product parallel");
}

#[test]
fn test_vec2_rotation() {
    let mut v = Vec2::new(1.0, 0.0);
    v.rotate(&Vec2::ZERO, std::f32::consts::FRAC_PI_2);
    
    assert_float_near(v.x, 0.0, 0.001, "Rotated x");
    assert_float_near(v.y, 1.0, 0.001, "Rotated y");
}

#[test]
fn test_vec2_distance_calculations() {
    let a = Vec2::new(0.0, 0.0);
    let b = Vec2::new(3.0, 4.0);
    
    assert_float_eq(a.distance(&b), 5.0, "Distance");
    assert_float_eq(a.distance_squared(&b), 25.0, "Distance squared");
}

#[test]
fn test_vec2_lerp() {
    let a = Vec2::new(0.0, 0.0);
    let b = Vec2::new(10.0, 10.0);
    
    let mid = a.lerp(&b, 0.5);
    assert_float_eq(mid.x, 5.0, "Lerp mid x");
    assert_float_eq(mid.y, 5.0, "Lerp mid y");
    
    let quarter = a.lerp(&b, 0.25);
    assert_float_eq(quarter.x, 2.5, "Lerp quarter x");
}

#[test]
fn test_vec3_operations() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    
    let sum = a + b;
    assert_eq!(sum, Vec3::new(5.0, 7.0, 9.0));
    
    let cross = a.cross(&b);
    assert_eq!(cross.x, -3.0);
    assert_eq!(cross.y, 6.0);
    assert_eq!(cross.z, -3.0);
}

#[test]
fn test_vec3_normalization() {
    let v = Vec3::new(2.0, 0.0, 0.0);
    let normalized = v.get_normalized();
    
    assert_float_eq(normalized.length(), 1.0, "Vec3 normalized length");
    assert_eq!(normalized, Vec3::UNIT_X);
}

#[test]
fn test_mat4_identity() {
    let m = Mat4::IDENTITY;
    assert!(m.is_identity());
    
    let v = Vec3::new(1.0, 2.0, 3.0);
    let transformed = m * v;
    assert_eq!(transformed, v);
}

#[test]
fn test_mat4_translation() {
    let translation = Vec3::new(10.0, 20.0, 30.0);
    let m = Mat4::create_translation(&translation);
    
    let point = Vec3::new(1.0, 2.0, 3.0);
    let transformed = m.transform_point(&point);
    
    assert_eq!(transformed, Vec3::new(11.0, 22.0, 33.0));
}

#[test]
fn test_mat4_scaling() {
    let scale = Vec3::new(2.0, 3.0, 4.0);
    let m = Mat4::create_scale(&scale);
    
    let point = Vec3::new(1.0, 1.0, 1.0);
    let transformed = m.transform_point(&point);
    
    assert_eq!(transformed, Vec3::new(2.0, 3.0, 4.0));
}

#[test]
fn test_mat4_rotation() {
    let quat = Quaternion::new(0.0, 0.0, 0.0, 1.0);
    let m = Mat4::create_rotation(&quat);
    
    assert!(m.is_identity());
}

#[test]
fn test_mat4_multiplication() {
    let translate = Mat4::create_translation(&Vec3::new(10.0, 0.0, 0.0));
    let scale = Mat4::create_scale(&Vec3::new(2.0, 2.0, 2.0));
    
    let combined = translate * scale;
    
    let v = Vec3::new(1.0, 1.0, 1.0);
    let result = combined * v;
    
    assert_eq!(result.x, 12.0);
    assert_eq!(result.y, 2.0);
    assert_eq!(result.z, 2.0);
}

#[test]
fn test_mat4_inversion() {
    let m = Mat4::create_translation(&Vec3::new(5.0, 10.0, 15.0));
    
    if let Some(inv) = m.inverted() {
        let result = m * inv;
        
        for i in 0..16 {
            let expected = if i % 5 == 0 { 1.0 } else { 0.0 };
            assert_float_near(result.m[i], expected, 0.001, &format!("Inverted matrix element {}", i));
        }
    } else {
        panic!("Matrix should be invertible");
    }
}

#[test]
fn test_quaternion_identity() {
    let q = Quaternion::new(0.0, 0.0, 0.0, 1.0);
    let m = Mat4::create_rotation(&q);
    assert!(m.is_identity());
}

#[test]
fn test_mat4_perspective() {
    let perspective = Mat4::create_perspective(60.0, 16.0/9.0, 0.1, 1000.0);
    
    assert!(!perspective.is_identity());
    assert!(perspective.m[0] > 0.0);
    assert!(perspective.m[5] > 0.0);
}

#[test]
fn test_mat4_orthographic() {
    let ortho = Mat4::create_orthographic(800.0, 600.0, -1.0, 1.0);
    
    assert!(!ortho.is_identity());
    assert!(ortho.m[0] > 0.0);
    assert!(ortho.m[5] > 0.0);
}

#[test]
fn test_mat4_look_at() {
    let eye = Vec3::new(0.0, 0.0, 10.0);
    let target = Vec3::ZERO;
    let up = Vec3::UNIT_Y;
    
    let view = Mat4::create_look_at(&eye, &target, &up);
    
    assert!(!view.is_identity());
}

#[test]
fn test_vec2_perpendicular() {
    let v = Vec2::new(3.0, 4.0);
    let perp = v.get_perp();
    
    assert_eq!(perp, Vec2::new(-4.0, 3.0));
    assert_float_near(v.dot(&perp), 0.0, 0.001, "Perpendicular dot product");
}

#[test]
fn test_vec2_projection() {
    let a = Vec2::new(3.0, 4.0);
    let b = Vec2::UNIT_X;
    
    let projected = a.project(&b);
    assert_float_eq(projected.x, 3.0, "Projection x");
    assert_float_eq(projected.y, 0.0, "Projection y");
}

#[test]
fn test_vec2_clamp() {
    let mut v = Vec2::new(15.0, -5.0);
    v.clamp(&Vec2::new(0.0, 0.0), &Vec2::new(10.0, 10.0));
    
    assert_eq!(v, Vec2::new(10.0, 0.0));
}

#[test]
fn test_vec2_fuzzy_equals() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(1.01, 2.01);
    
    assert!(a.fuzzy_equals(&b, 0.02));
    assert!(!a.fuzzy_equals(&b, 0.005));
}

#[test]
fn test_vec2_constants() {
    assert_eq!(Vec2::ZERO, Vec2::new(0.0, 0.0));
    assert_eq!(Vec2::ONE, Vec2::new(1.0, 1.0));
    assert_eq!(Vec2::UNIT_X, Vec2::new(1.0, 0.0));
    assert_eq!(Vec2::UNIT_Y, Vec2::new(0.0, 1.0));
}

#[test]
fn test_vec3_constants() {
    assert_eq!(Vec3::ZERO, Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(Vec3::ONE, Vec3::new(1.0, 1.0, 1.0));
    assert_eq!(Vec3::UNIT_X, Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(Vec3::UNIT_Y, Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(Vec3::UNIT_Z, Vec3::new(0.0, 0.0, 1.0));
}
