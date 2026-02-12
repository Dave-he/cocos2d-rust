use crate::math::Vec2;

/// Physics material properties
#[derive(Debug, Clone, Copy)]
pub struct PhysicsMaterial {
    /// Density of the material (kg/m²)
    pub density: f32,
    /// Restitution (bounciness) - 0.0 to 1.0
    pub restitution: f32,
    /// Friction coefficient - 0.0 to 1.0
    pub friction: f32,
}

impl PhysicsMaterial {
    pub const DEFAULT: PhysicsMaterial = PhysicsMaterial {
        density: 0.1,
        restitution: 0.5,
        friction: 0.5,
    };

    pub fn new(density: f32, restitution: f32, friction: f32) -> Self {
        PhysicsMaterial {
            density,
            restitution: restitution.clamp(0.0, 1.0),
            friction: friction.clamp(0.0, 1.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicsShapeType {
    UNKNOWN,
    CIRCLE,
    BOX,
    POLYGON,
    EDGE_SEGMENT,
    EDGE_BOX,
    EDGE_POLYGON,
    EDGE_CHAIN,
}

#[derive(Debug)]
pub struct PhysicsShape {
    shape_type: PhysicsShapeType,
    area: f32,
    moment: f32,
    tag: i32,
    material: PhysicsMaterial,
    sensor: bool,
    category_bitmask: u32,
    collision_bitmask: u32,
    contact_test_bitmask: u32,
    body: Option<*const PhysicsBody>,
}

impl PhysicsShape {
    pub fn new(shape_type: PhysicsShapeType) -> PhysicsShape {
        PhysicsShape {
            shape_type,
            area: 0.0,
            moment: 0.0,
            tag: 0,
            material: PhysicsMaterial::DEFAULT,
            sensor: false,
            category_bitmask: 0xFFFFFFFF,
            collision_bitmask: 0xFFFFFFFF,
            contact_test_bitmask: 0x00000000,
            body: None,
        }
    }

    /// Create a circle shape
    pub fn create_circle(radius: f32, material: PhysicsMaterial, offset: Vec2) -> Self {
        let area = std::f32::consts::PI * radius * radius;
        let moment = material.density * area * radius * radius / 2.0;
        
        PhysicsShape {
            shape_type: PhysicsShapeType::CIRCLE,
            area,
            moment,
            tag: 0,
            material,
            sensor: false,
            category_bitmask: 0xFFFFFFFF,
            collision_bitmask: 0xFFFFFFFF,
            contact_test_bitmask: 0x00000000,
            body: None,
        }
    }

    /// Create a box shape
    pub fn create_box(size: Vec2, material: PhysicsMaterial, offset: Vec2) -> Self {
        let area = size.x * size.y;
        let moment = material.density * area * (size.x * size.x + size.y * size.y) / 12.0;
        
        PhysicsShape {
            shape_type: PhysicsShapeType::BOX,
            area,
            moment,
            tag: 0,
            material,
            sensor: false,
            category_bitmask: 0xFFFFFFFF,
            collision_bitmask: 0xFFFFFFFF,
            contact_test_bitmask: 0x00000000,
            body: None,
        }
    }

    /// Create a polygon shape
    pub fn create_polygon(points: &[Vec2], material: PhysicsMaterial, offset: Vec2) -> Self {
        // Simple area calculation for convex polygon
        let area = Self::calculate_polygon_area(points);
        let moment = material.density * area * 100.0; // Simplified moment calculation
        
        PhysicsShape {
            shape_type: PhysicsShapeType::POLYGON,
            area,
            moment,
            tag: 0,
            material,
            sensor: false,
            category_bitmask: 0xFFFFFFFF,
            collision_bitmask: 0xFFFFFFFF,
            contact_test_bitmask: 0x00000000,
            body: None,
        }
    }

    /// Create an edge segment shape
    pub fn create_edge_segment(a: Vec2, b: Vec2, material: PhysicsMaterial, border: f32) -> Self {
        PhysicsShape {
            shape_type: PhysicsShapeType::EDGE_SEGMENT,
            area: 0.0,
            moment: f32::INFINITY,
            tag: 0,
            material,
            sensor: false,
            category_bitmask: 0xFFFFFFFF,
            collision_bitmask: 0xFFFFFFFF,
            contact_test_bitmask: 0x00000000,
            body: None,
        }
    }

    fn calculate_polygon_area(points: &[Vec2]) -> f32 {
        if points.len() < 3 {
            return 0.0;
        }
        
        let mut area = 0.0;
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            area += points[i].x * points[j].y;
            area -= points[j].x * points[i].y;
        }
        (area / 2.0).abs()
    }

    pub fn get_type(&self) -> PhysicsShapeType {
        self.shape_type
    }

    pub fn get_area(&self) -> f32 {
        self.area
    }

    pub fn get_moment(&self) -> f32 {
        self.moment
    }

    pub fn get_tag(&self) -> i32 {
        self.tag
    }

    pub fn set_tag(&mut self, tag: i32) {
      self.tag = tag;
    }

    pub fn get_material(&self) -> PhysicsMaterial {
        self.material
    }

    pub fn set_material(&mut self, material: PhysicsMaterial) {
        self.material = material;
    }

    pub fn is_sensor(&self) -> bool {
        self.sensor
    }

    pub fn set_sensor(&mut self, sensor: bool) {
        self.sensor = sensor;
    }

    pub fn get_category_bitmask(&self) -> u32 {
        self.category_bitmask
    }

    pub fn set_category_bitmask(&mut self, bitmask: u32) {
        self.category_bitmask = bitmask;
    }

    pub fn get_collision_bitmask(&self) -> u32 {
        self.collision_bitmask
    }

    pub fn set_collision_bitmask(&mut self, bitmask: u32) {
        self.collision_bitmask = bitmask;
    }

    pub fn get_contact_test_bitmask(&self) -> u32 {
        self.contact_test_bitmask
    }

    pub fn set_contact_test_bitmask(&mut self, bitmask: u32) {
        self.contact_test_bitmask = bitmask;
    }

    pub fn get_body(&self) -> Option<&PhysicsBody> {
        self.body.map(|b| unsafe { &*b })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicsBodyType {
    STATIC,
    DYNAMIC,
    KINEMATIC,
}

#[derive(Debug)]
pub struct PhysicsBody {
    body_type: PhysicsBodyType,
    mass: f32,
    moment: f32,
    linear_velocity: Vec2,
    angular_velocity: f32,
    velocity_limit: f32,
    angular_velocity_limit: f32,
    position: Vec2,
    rotation: f32,
    tag: i32,
    enabled: bool,
    gravity_enabled: bool,
    collision_enabled: bool,
}

impl Default for PhysicsBody {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsBody {
    pub fn new() -> PhysicsBody {
        PhysicsBody {
            body_type: PhysicsBodyType::DYNAMIC,
            mass: 0.0,
            moment: 0.0,
            linear_velocity: Vec2::ZERO,
            angular_velocity: 0.0,
            velocity_limit: 0.0,
            angular_velocity_limit: 0.0,
            position: Vec2::ZERO,
            rotation: 0.0,
            tag: 0,
            enabled: true,
            gravity_enabled: true,
            collision_enabled: true,
        }
    }

    pub fn create_static_body() -> PhysicsBody {
        PhysicsBody {
            body_type: PhysicsBodyType::STATIC,
            mass: 0.0,
            moment: 0.0,
            linear_velocity: Vec2::ZERO,
            angular_velocity: 0.0,
            velocity_limit: 0.0,
            angular_velocity_limit: 0.0,
            position: Vec2::ZERO,
            rotation: 0.0,
            tag: 0,
            enabled: true,
            gravity_enabled: false,
            collision_enabled: true,
        }
    }

    pub fn create_dynamic_body(mass: f32, moment: f32) -> PhysicsBody {
        PhysicsBody {
            body_type: PhysicsBodyType::DYNAMIC,
            mass,
            moment,
            linear_velocity: Vec2::ZERO,
            angular_velocity: 0.0,
            velocity_limit: 0.0,
            angular_velocity_limit: 0.0,
            position: Vec2::ZERO,
            rotation: 0.0,
            tag: 0,
            enabled: true,
            gravity_enabled: true,
            collision_enabled: true,
        }
    }

    /// Create a box body (convenience method)
    /// By default creates a static body, call set_dynamic(true) to make it dynamic
    pub fn create_box(width: f32, height: f32) -> PhysicsBody {
        let material = PhysicsMaterial::DEFAULT;
        let size = Vec2::new(width, height);
        let area = width * height;
        let moment = material.density * area * (width * width + height * height) / 12.0;
        
        let mut body = PhysicsBody::create_static_body();
        body.mass = material.density * area;
        body.moment = moment;
        body
    }

    /// Create a circle body (convenience method)
    /// By default creates a static body, call set_dynamic(true) to make it dynamic
    pub fn create_circle(radius: f32) -> PhysicsBody {
        let material = PhysicsMaterial::DEFAULT;
        let area = std::f32::consts::PI * radius * radius;
        let moment = material.density * area * radius * radius / 2.0;
        
        let mut body = PhysicsBody::create_static_body();
        body.mass = material.density * area;
        body.moment = moment;
        body
    }

    pub fn is_dynamic(&self) -> bool {
        self.body_type == PhysicsBodyType::DYNAMIC
    }

    pub fn set_dynamic(&mut self, dynamic: bool) {
        self.body_type = if dynamic {
            PhysicsBodyType::DYNAMIC
        } else {
            PhysicsBodyType::STATIC
        };
    }

    pub fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }

    pub fn get_type(&self) -> PhysicsBodyType {
        self.body_type
    }

    pub fn get_mass(&self) -> f32 {
        self.mass
    }

    pub fn get_moment(&self) -> f32 {
        self.moment
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.linear_velocity
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.linear_velocity = velocity;
    }

    pub fn get_angular_velocity(&self) -> f32 {
        self.angular_velocity
    }

    pub fn set_angular_velocity(&mut self, velocity: f32) {
        self.angular_velocity = velocity;
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn get_tag(&self) -> i32 {
        self.tag
    }

    pub fn set_tag(&mut self, tag: i32) {
        self.tag = tag;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_gravity_enabled(&self) -> bool {
        self.gravity_enabled
    }

    pub fn set_gravity_enabled(&mut self, enabled: bool) {
        self.gravity_enabled = enabled;
    }

    pub fn is_collision_enabled(&self) -> bool {
        self.collision_enabled
    }

    pub fn set_collision_enabled(&mut self, enabled: bool) {
        self.collision_enabled = enabled;
    }

    /// Apply an impulse to the body
    pub fn apply_impulse(&mut self, impulse: Vec2, offset: Vec2) {
        if self.body_type == PhysicsBodyType::DYNAMIC {
            self.linear_velocity += impulse / self.mass;
            // Angular impulse calculation
            let r = offset;
            let angular_impulse = r.x * impulse.y - r.y * impulse.x;
            self.angular_velocity += angular_impulse / self.moment;
        }
    }

    /// Apply a force to the body
    pub fn apply_force(&mut self, force: Vec2, offset: Vec2, delta_time: f32) {
        if self.body_type == PhysicsBodyType::DYNAMIC {
            let impulse = force * delta_time;
            self.apply_impulse(impulse, offset);
        }
    }

    /// Get the velocity at a point
    pub fn get_velocity_at_local_point(&self, point: Vec2) -> Vec2 {
        let r = point - self.position;
        Vec2::new(
            self.linear_velocity.x - r.y * self.angular_velocity,
            self.linear_velocity.y + r.x * self.angular_velocity,
        )
    }

    /// Reset forces
    pub fn reset_forces(&mut self) {
        // In a real implementation, this would reset accumulated forces
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointType {
    DISTANCE,
    SPRING,
    GEAR,
    PULLEY,
    WHEEL,
    MOUSE,
    FIXED,
    WELD,
}

#[derive(Debug)]
pub struct PhysicsJoint {
    joint_type: JointType,
    body_a: *const PhysicsBody,
    body_b: *const PhysicsBody,
    collide_connected: bool,
    enabled: bool,
}

impl PhysicsJoint {
    pub fn new(joint_type: JointType) -> PhysicsJoint {
        PhysicsJoint {
            joint_type,
            body_a: std::ptr::null(),
            body_b: std::ptr::null(),
            collide_connected: false,
            enabled: true,
        }
    }

    pub fn get_type(&self) -> JointType {
        self.joint_type
    }

    pub fn set_bodies(&mut self, body_a: &PhysicsBody, body_b: &PhysicsBody) {
        self.body_a = body_a as *const PhysicsBody;
        self.body_b = body_b as *const PhysicsBody;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn get_collide_connected(&self) -> bool {
        self.collide_connected
    }

    pub fn set_collide_connected(&mut self, collide: bool) {
        self.collide_connected = collide;
    }
}

/// Ray cast result information
#[derive(Debug, Clone)]
pub struct RayCastInfo {
    pub shape: usize,  // Shape ID
    pub start: Vec2,
    pub end: Vec2,
    pub contact: Vec2,
    pub normal: Vec2,
    pub fraction: f32,
}

/// Query result for rect/point queries
#[derive(Debug, Clone)]
pub struct QueryInfo {
    pub shape: usize,  // Shape ID
}

#[derive(Debug)]
pub struct PhysicsWorld {
    gravity: Vec2,
    speed: f32,
    debug_draw_flags: u32,
    substeps: i32,
    update_rate: i32,
    bodies: Vec<PhysicsBody>,
    shapes: Vec<PhysicsShape>,
    joints: Vec<PhysicsJoint>,
    auto_step: bool,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsWorld {
    pub fn new() -> PhysicsWorld {
        PhysicsWorld {
            gravity: Vec2::new(0.0, -98.0),
            speed: 1.0,
            debug_draw_flags: 0,
            substeps: 1,
            update_rate: 1,
            bodies: Vec::new(),
            shapes: Vec::new(),
            joints: Vec::new(),
            auto_step: true,
        }
    }

    pub fn get_gravity(&self) -> Vec2 {
        self.gravity
    }

    pub fn set_gravity(&mut self, gravity: Vec2) {
        self.gravity = gravity;
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }


    pub fn get_substeps(&self) -> i32 {
        self.substeps
    }

    pub fn set_substeps(&mut self, substeps: i32) {
        self.substeps = substeps.max(1);
    }

    pub fn get_update_rate(&self) -> i32 {
        self.update_rate
    }

    pub fn set_update_rate(&mut self, rate: i32) {
        self.update_rate = rate.max(1);
    }

    pub fn add_body(&mut self, body: &PhysicsBody) {
        // In real implementation, this would add the body to the physics simulation
    }

    pub fn remove_body(&mut self, body: &PhysicsBody) {
        // In real implementation, this would remove the body from the physics simulation
    }

    pub fn add_shape(&mut self, shape: &PhysicsShape) {
        // In real implementation, this would add the shape to the physics simulation
    }

    pub fn remove_shape(&mut self, shape: &PhysicsShape) {
        // In real implementation, this would remove the shape from the physics simulation
    }

    pub fn add_joint(&mut self, joint: &PhysicsJoint) {
        // In real implementation, this would add the joint to the physics simulation
    }

    pub fn remove_joint(&mut self, joint: &PhysicsJoint) {
        // In real implementation, this would remove the joint from the physics simulation
    }

    /// Perform a ray cast in the physics world
    pub fn ray_cast(&self, start: Vec2, end: Vec2) -> Vec<RayCastInfo> {
        let mut results = Vec::new();
        
        // Simple ray-circle intersection for demonstration
        // In real implementation, this would use Chipmunk or Box2D
        for (i, shape) in self.shapes.iter().enumerate() {
            if shape.get_type() == PhysicsShapeType::CIRCLE {
                // Simplified ray-circle intersection
                if let Some(hit) = self.ray_circle_intersect(start, end, Vec2::ZERO, 10.0) {
                    results.push(RayCastInfo {
                        shape: i,
                        start,
                        end,
                        contact: hit.0,
                        normal: hit.1,
                        fraction: hit.2,
                    });
                }
            }
        }
        
        results
    }

    fn ray_circle_intersect(&self, start: Vec2, end: Vec2, center: Vec2, radius: f32) 
        -> Option<(Vec2, Vec2, f32)> {
        let d = end - start;
        let f = start - center;
        
        let a = d.dot(&d);
        let b = 2.0 * f.dot(&d);
        let c = f.dot(&f) - radius * radius;
        
        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            return None;
        }
        
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        
        if (0.0..=1.0).contains(&t) {
            let contact = start + d * t;
            let mut normal_vec = contact - center;
            normal_vec.normalize();
            Some((contact, normal_vec, t))
        } else {
            None
        }
    }

    /// Query shapes in a rectangle
    pub fn query_rect(&self, rect_start: Vec2, rect_end: Vec2) -> Vec<QueryInfo> {
        let mut results = Vec::new();
        
        // Simple AABB intersection for demonstration
        for (i, _shape) in self.shapes.iter().enumerate() {
            // In real implementation, check if shape intersects with rect
            results.push(QueryInfo { shape: i });
        }
        
        results
    }

    /// Query shapes at a point
    pub fn query_point(&self, point: Vec2) -> Vec<QueryInfo> {
        let mut results = Vec::new();
        
        // Check if point is inside any shape
        for (i, _shape) in self.shapes.iter().enumerate() {
            // In real implementation, check if point is inside shape
            results.push(QueryInfo { shape: i });
        }
        
        results
    }

    /// Get all bodies in the world
    pub fn get_all_bodies(&self) -> &[PhysicsBody] {
        &self.bodies
    }

    /// Step the physics simulation
    pub fn step(&mut self, delta: f32) {
        if !self.auto_step {
            return;
        }

        let dt = delta * self.speed / self.substeps as f32;
        
        for _ in 0..self.substeps {
            // Apply gravity
            for body in &mut self.bodies {
                if body.is_gravity_enabled() && body.get_type() == PhysicsBodyType::DYNAMIC {
                    let force = self.gravity * body.get_mass();
                    body.apply_force(force, Vec2::ZERO, dt);
                }
            }
            
            // Update positions (simplified Euler integration)
            for body in &mut self.bodies {
                if body.get_type() == PhysicsBodyType::DYNAMIC && body.is_enabled() {
                    let pos = body.get_position();
                    let vel = body.get_velocity();
                    body.set_position(pos + vel * dt);
                    
                    let rot = body.get_rotation();
                    let ang_vel = body.get_angular_velocity();
                    body.set_rotation(rot + ang_vel * dt);
                }
            }
            
            // Collision detection and response would go here
            // In real implementation, this would use Chipmunk or Box2D
        }
    }

    pub fn set_auto_step(&mut self, auto_step: bool) {
        self.auto_step = auto_step;
    }

    pub fn is_auto_step(&self) -> bool {
        self.auto_step
    }

    pub fn set_debug_draw_enabled(&mut self, enabled: bool) {
        self.debug_draw_flags = if enabled { 0xFFFFFFFF } else { 0 };
    }

    pub fn is_debug_draw_enabled(&self) -> bool {
        self.debug_draw_flags != 0
    }
}

#[derive(Debug)]
pub struct PhysicsContact {
    contact_id: i32,
    body_a: *const PhysicsBody,
    body_b: *const PhysicsBody,
    contact_point: Vec2,
    contact_normal: Vec2,
}

impl Default for PhysicsContact {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsContact {
    pub fn new() -> PhysicsContact {
        PhysicsContact {
            contact_id: 0,
            body_a: std::ptr::null(),
            body_b: std::ptr::null(),
            contact_point: Vec2::ZERO,
            contact_normal: Vec2::ZERO,
        }
    }

    pub fn get_contact_id(&self) -> i32 {
        self.contact_id
    }

    pub fn get_body_a(&self) -> Option<&PhysicsBody> {
        if self.body_a.is_null() {
            None
        } else {
            Some(unsafe { &*self.body_a })
        }
    }

    pub fn get_body_b(&self) -> Option<&PhysicsBody> {
        if self.body_b.is_null() {
            None
        } else {
            Some(unsafe { &*self.body_b })
        }
    }

    pub fn get_contact_point(&self) -> Vec2 {
        self.contact_point
    }

    pub fn get_contact_normal(&self) -> Vec2 {
        self.contact_normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_material() {
        let material = PhysicsMaterial::new(1.0, 0.8, 0.5);
        assert_eq!(material.density, 1.0);
        assert_eq!(material.restitution, 0.8);
        assert_eq!(material.friction, 0.5);

        // Test clamping
        let material2 = PhysicsMaterial::new(1.0, 1.5, -0.5);
        assert_eq!(material2.restitution, 1.0);
        assert_eq!(material2.friction, 0.0);
    }

    #[test]
    fn test_physics_shape_creation() {
        // Test circle shape
        let circle = PhysicsShape::create_circle(10.0, PhysicsMaterial::DEFAULT, Vec2::ZERO);
        assert_eq!(circle.get_type(), PhysicsShapeType::CIRCLE);
        assert!(circle.get_area() > 0.0);

        // Test box shape
        let box_shape = PhysicsShape::create_box(Vec2::new(20.0, 30.0), PhysicsMaterial::DEFAULT, Vec2::ZERO);
        assert_eq!(box_shape.get_type(), PhysicsShapeType::BOX);
        assert_eq!(box_shape.get_area(), 600.0);

        // Test polygon shape
        let points = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(0.0, 10.0),
        ];
        let polygon = PhysicsShape::create_polygon(&points, PhysicsMaterial::DEFAULT, Vec2::ZERO);
        assert_eq!(polygon.get_type(), PhysicsShapeType::POLYGON);
        assert_eq!(polygon.get_area(), 100.0);
    }

    #[test]
    fn test_physics_body_types() {
        let static_body = PhysicsBody::create_static_body();
        assert_eq!(static_body.get_type(), PhysicsBodyType::STATIC);
        assert!(!static_body.is_gravity_enabled());

        let dynamic_body = PhysicsBody::create_dynamic_body(10.0, 5.0);
        assert_eq!(dynamic_body.get_type(), PhysicsBodyType::DYNAMIC);
        assert_eq!(dynamic_body.get_mass(), 10.0);
        assert_eq!(dynamic_body.get_moment(), 5.0);
        assert!(dynamic_body.is_gravity_enabled());
    }

    #[test]
    fn test_physics_body_velocity() {
        let mut body = PhysicsBody::new();
        
        body.set_velocity(Vec2::new(10.0, 20.0));
        assert_eq!(body.get_velocity(), Vec2::new(10.0, 20.0));

        body.set_angular_velocity(1.5);
        assert_eq!(body.get_angular_velocity(), 1.5);
    }

    #[test]
    fn test_physics_body_impulse() {
        let mut body = PhysicsBody::create_dynamic_body(10.0, 10.0);
        body.set_velocity(Vec2::ZERO);

        let impulse = Vec2::new(100.0, 0.0);
        body.apply_impulse(impulse, Vec2::ZERO);

        // After impulse, velocity should be impulse / mass
        assert_eq!(body.get_velocity(), Vec2::new(10.0, 0.0));
    }

    #[test]
    fn test_physics_world() {
        let mut world = PhysicsWorld::new();
        
        assert_eq!(world.get_gravity(), Vec2::new(0.0, -98.0));
        
        world.set_gravity(Vec2::new(0.0, -9.8));
        assert_eq!(world.get_gravity(), Vec2::new(0.0, -9.8));

        assert_eq!(world.get_speed(), 1.0);
        world.set_speed(2.0);
        assert_eq!(world.get_speed(), 2.0);

        assert_eq!(world.get_substeps(), 1);
        world.set_substeps(4);
        assert_eq!(world.get_substeps(), 4);
    }

    #[test]
    fn test_physics_joint() {
        let joint = PhysicsJoint::new(JointType::DISTANCE);
        assert_eq!(joint.get_type(), JointType::DISTANCE);
        assert!(joint.is_enabled());
        assert!(!joint.get_collide_connected());
    }

    #[test]
    fn test_physics_contact() {
        let contact = PhysicsContact::new();
        assert_eq!(contact.get_contact_id(), 0);
        assert_eq!(contact.get_contact_point(), Vec2::ZERO);
        assert_eq!(contact.get_contact_normal(), Vec2::ZERO);
    }

    #[test]
    fn test_shape_bitmasks() {
        let mut shape = PhysicsShape::new(PhysicsShapeType::CIRCLE);
        
        assert_eq!(shape.get_category_bitmask(), 0xFFFFFFFF);
        assert_eq!(shape.get_collision_bitmask(), 0xFFFFFFFF);
        assert_eq!(shape.get_contact_test_bitmask(), 0x00000000);

        shape.set_category_bitmask(0x0001);
        shape.set_collision_bitmask(0x0002);
        shape.set_contact_test_bitmask(0x0004);

        assert_eq!(shape.get_category_bitmask(), 0x0001);
        assert_eq!(shape.get_collision_bitmask(), 0x0002);
        assert_eq!(shape.get_contact_test_bitmask(), 0x0004);
    }

    #[test]
    fn test_shape_sensor() {
        let mut shape = PhysicsShape::new(PhysicsShapeType::CIRCLE);
        
        assert!(!shape.is_sensor());
        
        shape.set_sensor(true);
        assert!(shape.is_sensor());
    }
}
