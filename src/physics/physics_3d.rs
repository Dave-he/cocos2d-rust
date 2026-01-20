use crate::math::Vec3;

#[derive(Debug)]
pub struct Physics3DBody {
    mass: f32,
    position: Vec3,
    rotation: Vec3,
    velocity: Vec3,
    angular_velocity: Vec3,
    enabled: bool,
}

impl Physics3DBody {
    pub fn new() -> Physics3DBody {
        Physics3DBody {
            mass: 1.0,
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            enabled: true,
        }
    }

    pub fn get_mass(&self) -> f32 {
        self.mass
    }

    pub fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn get_rotation(&self) -> Vec3 {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
    }

    pub fn get_velocity(&self) -> Vec3 {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }

    pub fn get_angular_velocity(&self) -> Vec3 {
        self.angular_velocity
    }

    pub fn set_angular_velocity(&mut self, velocity: Vec3) {
        self.angular_velocity = velocity;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Physics3DShapeType {
    BOX,
    SPHERE,
    CAPSULE,
    CONVEX_HULL,
    MESH,
}

#[derive(Debug)]
pub struct Physics3DShape {
    shape_type: Physics3DShapeType,
    size: Vec3,
    radius: f32,
    height: f32,
}

impl Physics3DShape {
    pub fn new(shape_type: Physics3DShapeType) -> Physics3DShape {
        Physics3DShape {
            shape_type,
            size: Vec3::new(1.0, 1.0, 1.0),
            radius: 0.5,
            height: 1.0,
        }
    }

    pub fn create_box(size: Vec3) -> Physics3DShape {
        Physics3DShape {
            shape_type: Physics3DShapeType::BOX,
            size,
            radius: 0.0,
            height: 0.0,
        }
    }

    pub fn create_sphere(radius: f32) -> Physics3DShape {
        Physics3DShape {
            shape_type: Physics3DShapeType::SPHERE,
            size: Vec3::ZERO,
            radius,
            height: 0.0,
        }
    }

    pub fn create_capsule(radius: f32, height: f32) -> Physics3DShape {
        Physics3DShape {
            shape_type: Physics3DShapeType::CAPSULE,
            size: Vec3::ZERO,
            radius,
            height,
        }
    }

    pub fn get_type(&self) -> Physics3DShapeType {
        self.shape_type
    }
}

#[derive(Debug)]
pub struct Physics3DWorld {
    gravity: Vec3,
    simulation_time: f32,
    debug_draw: bool,
}

impl Physics3DWorld {
    pub fn new() -> Physics3DWorld {
        Physics3DWorld {
            gravity: Vec3::new(0.0, -9.8, 0.0),
            simulation_time: 0.0,
            debug_draw: false,
        }
    }

    pub fn get_gravity(&self) -> Vec3 {
        self.gravity
    }

    pub fn set_gravity(&mut self, gravity: Vec3) {
        self.gravity = gravity;
    }

    pub fn add_body(&mut self, body: &Physics3DBody) {
    }

    pub fn remove_body(&mut self, body: &Physics3DBody) {
    }

    pub fn step(&mut self, delta: f32) {
        self.simulation_time += delta;
    }

    pub fn set_debug_draw_enabled(&mut self, enabled: bool) {
        self.debug_draw = enabled;
    }

    pub fn is_debug_draw_enabled(&self) -> bool {
        self.debug_draw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavMeshQueryResult {
    FAILURE,
    SUCCESS,
    PARTIAL,
}

#[derive(Debug)]
pub struct NavMeshPath {
    corners: Vec<Vec3>,
    length: f32,
}

impl NavMeshPath {
    pub fn new() -> NavMeshPath {
        NavMeshPath {
            corners: Vec::new(),
            length: 0.0,
        }
    }

    pub fn get_corners(&self) -> &Vec<Vec3> {
        &self.corners
    }

    pub fn get_length(&self) -> f32 {
        self.length
    }
}
