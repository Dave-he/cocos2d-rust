use cocos2d_rust::physics::{PhysicsWorld, PhysicsBody, PhysicsShape, PhysicsShapeType, PhysicsMaterial};
use cocos2d_rust::math::Vec2;

#[test]
fn test_physics_world_creation() {
    let world = PhysicsWorld::new();
    
    assert_eq!(world.get_gravity(), Vec2::new(0.0, -98.0));
}

#[test]
fn test_physics_world_gravity() {
    let mut world = PhysicsWorld::new();
    
    world.set_gravity(Vec2::new(0.0, -10.0));
    assert_eq!(world.get_gravity(), Vec2::new(0.0, -10.0));
}

#[test]
fn test_physics_body_creation() {
    let body = PhysicsBody::create_box(100.0, 100.0);
    
    assert!(!body.is_dynamic());
}

#[test]
fn test_physics_body_dynamic() {
    let mut body = PhysicsBody::create_box(50.0, 50.0);
    
    body.set_dynamic(true);
    assert!(body.is_dynamic());
}

#[test]
fn test_physics_body_mass() {
    let mut body = PhysicsBody::create_circle(25.0);
    
    body.set_mass(10.0);
    assert_eq!(body.get_mass(), 10.0);
}

#[test]
fn test_physics_body_velocity() {
    let mut body = PhysicsBody::create_box(50.0, 50.0);
    
    body.set_velocity(Vec2::new(100.0, 50.0));
    assert_eq!(body.get_velocity(), Vec2::new(100.0, 50.0));
}

#[test]
fn test_physics_body_position() {
    let mut body = PhysicsBody::create_box(50.0, 50.0);
    
    body.set_position(Vec2::new(200.0, 300.0));
    assert_eq!(body.get_position(), Vec2::new(200.0, 300.0));
}

#[test]
fn test_physics_body_rotation() {
    let mut body = PhysicsBody::create_box(50.0, 50.0);
    
    body.set_rotation(45.0);
    assert_eq!(body.get_rotation(), 45.0);
}

#[test]
fn test_physics_shape_box() {
    let shape = PhysicsShape::create_box(Vec2::new(100.0, 100.0), PhysicsMaterial::DEFAULT, Vec2::ZERO);
    
    assert_eq!(shape.get_type(), PhysicsShapeType::Box);
}

#[test]
fn test_physics_shape_circle() {
    let shape = PhysicsShape::create_circle(50.0, PhysicsMaterial::DEFAULT, Vec2::ZERO);
    
    assert_eq!(shape.get_type(), PhysicsShapeType::Circle);
}

#[test]
fn test_physics_collision_detection() {
    let mut world = PhysicsWorld::new();
    
    let mut body1 = PhysicsBody::create_box(50.0, 50.0);
    body1.set_position(Vec2::new(0.0, 0.0));
    
    let mut body2 = PhysicsBody::create_box(50.0, 50.0);
    body2.set_position(Vec2::new(25.0, 0.0));
    
    world.add_body(&body1);
    world.add_body(&body2);
    
    world.step(0.016);
}

#[test]
fn test_physics_body_apply_force() {
    let mut body = PhysicsBody::create_box(50.0, 50.0);
    body.set_dynamic(true);
    
    body.apply_force(Vec2::new(100.0, 0.0), Vec2::ZERO, 0.016);
}

#[test]
fn test_physics_body_apply_impulse() {
    let mut body = PhysicsBody::create_box(50.0, 50.0);
    body.set_dynamic(true);
    
    body.apply_impulse(Vec2::new(50.0, 100.0), Vec2::ZERO);
}
