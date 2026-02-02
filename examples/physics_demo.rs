use cocos2d_rust::{
    PhysicsWorld, PhysicsBody, PhysicsShape, PhysicsBodyType, PhysicsShapeType,
    PhysicsMaterial, Vec2,
    Physics3DWorld, Physics3DBody, Physics3DShape, Physics3DConstraint,
    Physics3DConstraintType, Vec3,
};

fn demo_2d_physics() {
    println!("=== 2D Physics Demo ===\n");

    // Create physics world
    let mut physics_world = PhysicsWorld::new();
    println!("✓ 2D Physics world created");
    println!("  Gravity: {:?}", physics_world.get_gravity());

    // Create custom material
    let ball_material = PhysicsMaterial::new(1.0, 0.8, 0.3);
    println!("✓ Custom material created (density: {}, restitution: {}, friction: {})",
             ball_material.density, ball_material.restitution, ball_material.friction);

    // Create circle shape
    let circle = PhysicsShape::create_circle(20.0, ball_material, Vec2::ZERO);
    println!("✓ Circle shape created (type: {:?}, area: {:.2})", 
             circle.get_type(), circle.get_area());

    // Create box shape
    let box_shape = PhysicsShape::create_box(Vec2::new(50.0, 50.0), 
                                               PhysicsMaterial::DEFAULT, Vec2::ZERO);
    println!("✓ Box shape created (type: {:?}, area: {:.2})", 
             box_shape.get_type(), box_shape.get_area());

    // Create polygon shape
    let triangle = vec![
        Vec2::new(0.0, 30.0),
        Vec2::new(-25.0, -15.0),
        Vec2::new(25.0, -15.0),
    ];
    let polygon = PhysicsShape::create_polygon(&triangle, PhysicsMaterial::DEFAULT, Vec2::ZERO);
    println!("✓ Polygon shape created (type: {:?}, area: {:.2})", 
             polygon.get_type(), polygon.get_area());

    // Create dynamic body
    let mut dynamic_body = PhysicsBody::create_dynamic_body(1.0, 1.0);
    dynamic_body.set_position(Vec2::new(100.0, 200.0));
    dynamic_body.set_velocity(Vec2::new(50.0, 0.0));
    println!("✓ Dynamic body created at ({:.0}, {:.0})", 
             dynamic_body.get_position().x, dynamic_body.get_position().y);

    // Create static body (ground)
    let mut static_body = PhysicsBody::create_static_body();
    static_body.set_position(Vec2::new(200.0, 50.0));
    println!("✓ Static body created (ground)");

    // Apply impulse
    dynamic_body.apply_impulse(Vec2::new(0.0, 100.0), Vec2::ZERO);
    println!("✓ Impulse applied to dynamic body");

    // Ray casting
    println!("\n--- Ray Casting ---");
    let ray_results = physics_world.ray_cast(Vec2::new(0.0, 0.0), Vec2::new(200.0, 200.0));
    println!("  Ray cast from (0,0) to (200,200): {} hits", ray_results.len());

    // Simulate a few steps
    println!("\n--- Simulation Steps ---");
    for i in 0..5 {
        physics_world.step(1.0 / 60.0);
        println!("  Step {}: velocity = {:?}", i+1, dynamic_body.get_velocity());
    }

    println!("\n2D Physics features demonstrated:");
    println!("  ✓ Multiple shape types (circle, box, polygon)");
    println!("  ✓ Physics materials (density, friction, restitution)");
    println!("  ✓ Body types (static, dynamic)");
    println!("  ✓ Force and impulse application");
    println!("  ✓ Ray casting");
    println!("  ✓ Physics simulation stepping\n");
}

fn demo_3d_physics() {
    println!("=== 3D Physics Demo ===\n");

    // Create 3D physics world
    let mut physics_world_3d = Physics3DWorld::new();
    println!("✓ 3D Physics world created");
    println!("  Gravity: {:?}", physics_world_3d.get_gravity());

    // Create 3D shapes
    let box_3d = Physics3DShape::create_box(Vec3::new(2.0, 2.0, 2.0));
    println!("✓ 3D Box shape created (type: {:?})", box_3d.get_type());

    let sphere_3d = Physics3DShape::create_sphere(1.0);
    println!("✓ 3D Sphere shape created (type: {:?})", sphere_3d.get_type());

    let capsule_3d = Physics3DShape::create_capsule(0.5, 2.0);
    println!("✓ 3D Capsule shape created (type: {:?})", capsule_3d.get_type());

    let cylinder_3d = Physics3DShape::create_cylinder(0.5, 2.0);
    println!("✓ 3D Cylinder shape created (type: {:?})", cylinder_3d.get_type());

    let cone_3d = Physics3DShape::create_cone(0.5, 2.0);
    println!("✓ 3D Cone shape created (type: {:?})", cone_3d.get_type());

    // Create 3D bodies
    let mut dynamic_body_3d = Physics3DBody::create_dynamic(10.0);
    dynamic_body_3d.set_position(Vec3::new(0.0, 5.0, 0.0));
    dynamic_body_3d.set_linear_damping(0.1);
    dynamic_body_3d.set_angular_damping(0.05);
    println!("✓ 3D Dynamic body created (mass: {}, pos: {:?})", 
             dynamic_body_3d.get_mass(), dynamic_body_3d.get_position());

    let static_body_3d = Physics3DBody::create_static();
    println!("✓ 3D Static body created (ground plane)");

    // Apply forces
    dynamic_body_3d.apply_central_impulse(Vec3::new(5.0, 10.0, 0.0));
    println!("✓ Central impulse applied");

    dynamic_body_3d.apply_torque(Vec3::new(0.0, 1.0, 0.0), 0.016);
    println!("✓ Torque applied");

    // Create constraints
    let body_a = Physics3DBody::new();
    let body_b = Physics3DBody::new();

    let _point_to_point = Physics3DConstraint::create_point_to_point(
        &body_a, &body_b, Vec3::ZERO, Vec3::ZERO
    );
    println!("✓ Point-to-point constraint created");

    let _hinge = Physics3DConstraint::create_hinge(
        &body_a, &body_b, 
        Vec3::ZERO, Vec3::ZERO,
        Vec3::UP, Vec3::UP
    );
    println!("✓ Hinge constraint created");

    let _slider = Physics3DConstraint::create_slider(&body_a, &body_b);
    println!("✓ Slider constraint created");

    // Simulate
    println!("\n--- 3D Simulation Steps ---");
    physics_world_3d.set_substeps(4);
    for i in 0..5 {
        physics_world_3d.step(1.0 / 60.0);
        println!("  Step {}: velocity = {:?}", i+1, dynamic_body_3d.get_linear_velocity());
    }

    println!("\n3D Physics features demonstrated:");
    println!("  ✓ Multiple 3D shapes (box, sphere, capsule, cylinder, cone)");
    println!("  ✓ 3D rigid bodies with mass and inertia");
    println!("  ✓ Force, impulse, and torque application");
    println!("  ✓ Constraints (point-to-point, hinge, slider)");
    println!("  ✓ Damping (linear and angular)");
    println!("  ✓ Substep simulation for better accuracy\n");
}

fn main() {
    println!("╔═══════════════════════════════════════════════╗");
    println!("║   Cocos2d-Rust Physics System Demo           ║");
    println!("╚═══════════════════════════════════════════════╝\n");

    demo_2d_physics();
    println!("\n{}\n", "=".repeat(50));
    demo_3d_physics();

    println!("\n╔═══════════════════════════════════════════════╗");
    println!("║   Physics System Overview                     ║");
    println!("╠═══════════════════════════════════════════════╣");
    println!("║ 2D Physics Features:                          ║");
    println!("║   • Rigid bodies (static, dynamic, kinematic) ║");
    println!("║   • Collision shapes (circle, box, polygon)   ║");
    println!("║   • Physics materials (friction, restitution) ║");
    println!("║   • Joints and constraints                    ║");
    println!("║   • Ray casting and queries                   ║");
    println!("║   • Contact listeners and callbacks           ║");
    println!("║                                               ║");
    println!("║ 3D Physics Features:                          ║");
    println!("║   • 3D rigid body dynamics                    ║");
    println!("║   • Multiple shape primitives                 ║");
    println!("║   • Constraints (hinge, slider, 6DOF, etc.)   ║");
    println!("║   • Force and torque application              ║");
    println!("║   • Damping and friction                      ║");
    println!("║   • Ray casting and collision detection       ║");
    println!("╚═══════════════════════════════════════════════╝");
    
    println!("\nPhysics demo completed! ⚛️🎮");
}
