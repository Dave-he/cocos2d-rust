pub const EPSILON: f32 = 0.0001;

pub fn assert_float_eq(a: f32, b: f32, message: &str) {
    assert!((a - b).abs() < EPSILON, "{}: {} != {}", message, a, b);
}

pub fn assert_float_near(a: f32, b: f32, epsilon: f32, message: &str) {
    assert!((a - b).abs() < epsilon, "{}: {} != {} (epsilon: {})", message, a, b, epsilon);
}
