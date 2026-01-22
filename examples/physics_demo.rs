use cocos2d_rust::{PhysicsWorld, PhysicsBody, Vec2};

fn main() {
    println!("=== Cocos2d-Rust Physics Demo ===\n");

    // Create physics world
    let mut physics_world = PhysicsWorld::new();
    println!("✓ Physics world created");

    // Set gravity
    physics_world.set_gravity(Vec2::new(0.0, -98.0));
    println!("✓ Gravity set to -98.0");

    // Create dynamic body
    let mut body = PhysicsBody::create_dynamic_body(1.0, 1.0);
    println!("✓ Dynamic body created");

    // Add body to world
    physics_world.add_body(&body);
    println!("✓ Body added to world\n");

    println!("Physics features:");
    println!("  - Rigid bodies: Static, dynamic, kinematic");
    println!("  - Collision detection: AABB, circle, polygon");
    println!("  - Joints: Distance, spring, pulley, wheel");
    println!("  - Ray casting: Query physics world");
    println!("  - Contact listener: Collision callbacks\n");

    println!("Physics demo completed! ⚛️");
}
