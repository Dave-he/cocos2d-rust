use crate::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicsShapeType {
    CIRCLE,
    BOX,
    POLYGON,
    EDGE,
    CHAIN,
}

#[derive(Debug)]
pub struct PhysicsShape {
    shape_type: PhysicsShapeType,
    area: f32,
    moment: f32,
    tag: i32,
    body: Option<*const PhysicsBody>,
}

impl PhysicsShape {
    pub fn new(shape_type: PhysicsShapeType) -> PhysicsShape {
        PhysicsShape {
            shape_type,
            area: 0.0,
            moment: 0.0,
            tag: 0,
            body: None,
        }
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

#[derive(Debug)]
pub struct PhysicsWorld {
    gravity: Vec2,
    speed: f32,
    debug_draw_flags: u32,
}

impl PhysicsWorld {
    pub fn new() -> PhysicsWorld {
        PhysicsWorld {
            gravity: Vec2::new(0.0, -98.0),
            speed: 1.0,
            debug_draw_flags: 0,
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

    pub fn add_body(&mut self, body: &PhysicsBody) {
    }

    pub fn remove_body(&mut self, body: &PhysicsBody) {
    }

    pub fn add_shape(&mut self, shape: &PhysicsShape) {
    }

    pub fn remove_shape(&mut self, shape: &PhysicsShape) {
    }

    pub fn add_joint(&mut self, joint: &PhysicsJoint) {
    }

    pub fn remove_joint(&mut self, joint: &PhysicsJoint) {
    }

    pub fn step(&mut self, delta: f32) {
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
        if self.body_a.is_null() { None } else { Some(unsafe { &*self.body_a }) }
    }

    pub fn get_body_b(&self) -> Option<&PhysicsBody> {
        if self.body_b.is_null() { None } else { Some(unsafe { &*self.body_b }) }
    }

    pub fn get_contact_point(&self) -> Vec2 {
        self.contact_point
    }

    pub fn get_contact_normal(&self) -> Vec2 {
        self.contact_normal
    }
}
