use crate::math::{Vec3, Quaternion};

/// Physics material for 3D objects
#[derive(Debug, Clone, Copy)]
pub struct Physics3DMaterial {
    pub friction: f32,
    pub restitution: f32,
    pub rolling_friction: f32,
    pub spinning_friction: f32,
}

impl Physics3DMaterial {
    pub const DEFAULT: Physics3DMaterial = Physics3DMaterial {
        friction: 0.5,
        restitution: 0.0,
        rolling_friction: 0.0,
        spinning_friction: 0.0,
    };

    pub fn new(friction: f32, restitution: f32) -> Self {
        Physics3DMaterial {
            friction,
            restitution,
            rolling_friction: 0.0,
            spinning_friction: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Physics3DBodyType {
    STATIC,
    DYNAMIC,
    KINEMATIC,
}

#[derive(Debug)]
pub struct Physics3DBody {
    body_type: Physics3DBodyType,
    mass: f32,
    position: Vec3,
    rotation: Quaternion,
    linear_velocity: Vec3,
    angular_velocity: Vec3,
    linear_damping: f32,
    angular_damping: f32,
    enabled: bool,
    gravity_enabled: bool,
    collision_enabled: bool,
}

impl Default for Physics3DBody {
    fn default() -> Self {
        Self::new()
    }
}

impl Physics3DBody {
    pub fn new() -> Physics3DBody {
        Physics3DBody {
            body_type: Physics3DBodyType::DYNAMIC,
            mass: 1.0,
            position: Vec3::ZERO,
            rotation: Quaternion::IDENTITY,
            linear_velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            linear_damping: 0.0,
            angular_damping: 0.0,
            enabled: true,
            gravity_enabled: true,
            collision_enabled: true,
        }
    }

    pub fn create_static() -> Self {
        Physics3DBody {
            body_type: Physics3DBodyType::STATIC,
            mass: 0.0,
            position: Vec3::ZERO,
            rotation: Quaternion::IDENTITY,
            linear_velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            linear_damping: 0.0,
            angular_damping: 0.0,
            enabled: true,
            gravity_enabled: false,
            collision_enabled: true,
        }
    }

    pub fn create_dynamic(mass: f32) -> Self {
        Physics3DBody {
            body_type: Physics3DBodyType::DYNAMIC,
            mass,
            position: Vec3::ZERO,
            rotation: Quaternion::IDENTITY,
            linear_velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            linear_damping: 0.0,
            angular_damping: 0.0,
            enabled: true,
            gravity_enabled: true,
            collision_enabled: true,
        }
    }

    pub fn get_type(&self) -> Physics3DBodyType {
        self.body_type
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

    pub fn get_rotation(&self) -> Quaternion {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Quaternion) {
        self.rotation = rotation;
    }

    pub fn get_linear_velocity(&self) -> Vec3 {
        self.linear_velocity
    }

    pub fn set_linear_velocity(&mut self, velocity: Vec3) {
        self.linear_velocity = velocity;
    }

    pub fn get_angular_velocity(&self) -> Vec3 {
        self.angular_velocity
    }

    pub fn set_angular_velocity(&mut self, velocity: Vec3) {
        self.angular_velocity = velocity;
    }

    pub fn get_linear_damping(&self) -> f32 {
        self.linear_damping
    }

    pub fn set_linear_damping(&mut self, damping: f32) {
        self.linear_damping = damping;
    }

    pub fn get_angular_damping(&self) -> f32 {
        self.angular_damping
    }

    pub fn set_angular_damping(&mut self, damping: f32) {
        self.angular_damping = damping;
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

    /// Apply a central impulse
    pub fn apply_central_impulse(&mut self, impulse: Vec3) {
        if self.body_type == Physics3DBodyType::DYNAMIC {
            self.linear_velocity += impulse / self.mass;
        }
    }

    /// Apply an impulse at a point
    pub fn apply_impulse(&mut self, impulse: Vec3, rel_pos: Vec3) {
        if self.body_type == Physics3DBodyType::DYNAMIC {
            self.apply_central_impulse(impulse);
            let torque_impulse = rel_pos.cross(&impulse);
            self.angular_velocity += torque_impulse;
        }
    }

    /// Apply a central force
    pub fn apply_central_force(&mut self, force: Vec3, delta_time: f32) {
        self.apply_central_impulse(force * delta_time);
    }

    /// Apply torque
    pub fn apply_torque(&mut self, torque: Vec3, delta_time: f32) {
        if self.body_type == Physics3DBodyType::DYNAMIC {
            self.angular_velocity += torque * delta_time;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Physics3DShapeType {
    UNKNOWN,
    BOX,
    SPHERE,
    CYLINDER,
    CAPSULE,
    CONE,
    ConvexHull,
    MESH,
    HeightField,
    COMPOUND,
}

#[derive(Debug)]
pub struct Physics3DShape {
    shape_type: Physics3DShapeType,
    size: Vec3,
    radius: f32,
    height: f32,
    material: Physics3DMaterial,
}

impl Physics3DShape {
    pub fn new(shape_type: Physics3DShapeType) -> Physics3DShape {
        Physics3DShape {
            shape_type,
            size: Vec3::new(1.0, 1.0, 1.0),
            radius: 0.5,
            height: 1.0,
            material: Physics3DMaterial::DEFAULT,
        }
    }

    pub fn create_box(size: Vec3) -> Physics3DShape {
        Physics3DShape {
            shape_type: Physics3DShapeType::BOX,
            size,
            radius: 0.0,
            height: 0.0,
            material: Physics3DMaterial::DEFAULT,
        }
    }

    pub fn create_sphere(radius: f32) -> Physics3DShape {
        Physics3DShape {
            shape_type: Physics3DShapeType::SPHERE,
            size: Vec3::ZERO,
            radius,
            height: 0.0,
            material: Physics3DMaterial::DEFAULT,
        }
    }

    pub fn create_cylinder(radius: f32, height: f32) -> Physics3DShape {
        Physics3DShape {
            shape_type: Physics3DShapeType::CYLINDER,
            size: Vec3::ZERO,
            radius,
            height,
            material: Physics3DMaterial::DEFAULT,
        }
    }

    pub fn create_capsule(radius: f32, height: f32) -> Physics3DShape {
        Physics3DShape {
            shape_type: Physics3DShapeType::CAPSULE,
            size: Vec3::ZERO,
            radius,
            height,
            material: Physics3DMaterial::DEFAULT,
        }
    }

    pub fn create_cone(radius: f32, height: f32) -> Physics3DShape {
        Physics3DShape {
            shape_type: Physics3DShapeType::CONE,
            size: Vec3::ZERO,
            radius,
            height,
            material: Physics3DMaterial::DEFAULT,
        }
    }

    pub fn get_type(&self) -> Physics3DShapeType {
        self.shape_type
    }

    pub fn get_material(&self) -> Physics3DMaterial {
        self.material
    }

    pub fn set_material(&mut self, material: Physics3DMaterial) {
        self.material = material;
    }
}

/// 3D Physics constraint types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Physics3DConstraintType {
    PointToPoint,
    HINGE,
    SLIDER,
    ConeTwist,
    GENERIC_6DOF,
    FIXED,
}

/// 3D Physics constraint
#[derive(Debug)]
pub struct Physics3DConstraint {
    constraint_type: Physics3DConstraintType,
    body_a: Option<*const Physics3DBody>,
    body_b: Option<*const Physics3DBody>,
    enabled: bool,
    breaking_impulse: f32,
}

impl Physics3DConstraint {
    pub fn new(constraint_type: Physics3DConstraintType) -> Self {
        Physics3DConstraint {
            constraint_type,
            body_a: None,
            body_b: None,
            enabled: true,
            breaking_impulse: f32::INFINITY,
        }
    }

    pub fn create_point_to_point(body_a: &Physics3DBody, body_b: &Physics3DBody, 
                                  _pivot_in_a: Vec3, _pivot_in_b: Vec3) -> Self {
        Physics3DConstraint {
            constraint_type: Physics3DConstraintType::PointToPoint,
            body_a: Some(body_a as *const Physics3DBody),
            body_b: Some(body_b as *const Physics3DBody),
            enabled: true,
            breaking_impulse: f32::INFINITY,
        }
    }

    pub fn create_hinge(body_a: &Physics3DBody, body_b: &Physics3DBody,
                       _pivot_in_a: Vec3, _pivot_in_b: Vec3,
                       _axis_in_a: Vec3, _axis_in_b: Vec3) -> Self {
        Physics3DConstraint {
            constraint_type: Physics3DConstraintType::HINGE,
            body_a: Some(body_a as *const Physics3DBody),
            body_b: Some(body_b as *const Physics3DBody),
            enabled: true,
            breaking_impulse: f32::INFINITY,
        }
    }

    pub fn create_slider(body_a: &Physics3DBody, body_b: &Physics3DBody) -> Self {
        Physics3DConstraint {
            constraint_type: Physics3DConstraintType::SLIDER,
            body_a: Some(body_a as *const Physics3DBody),
            body_b: Some(body_b as *const Physics3DBody),
            enabled: true,
            breaking_impulse: f32::INFINITY,
        }
    }

    pub fn create_cone_twist(body_a: &Physics3DBody, body_b: &Physics3DBody) -> Self {
        Physics3DConstraint {
            constraint_type: Physics3DConstraintType::ConeTwist,
            body_a: Some(body_a as *const Physics3DBody),
            body_b: Some(body_b as *const Physics3DBody),
            enabled: true,
            breaking_impulse: f32::INFINITY,
        }
    }

    pub fn create_generic_6dof(body_a: &Physics3DBody, body_b: &Physics3DBody) -> Self {
        Physics3DConstraint {
            constraint_type: Physics3DConstraintType::GENERIC_6DOF,
            body_a: Some(body_a as *const Physics3DBody),
            body_b: Some(body_b as *const Physics3DBody),
            enabled: true,
            breaking_impulse: f32::INFINITY,
        }
    }

    pub fn get_type(&self) -> Physics3DConstraintType {
        self.constraint_type
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn get_breaking_impulse(&self) -> f32 {
        self.breaking_impulse
    }

    pub fn set_breaking_impulse(&mut self, impulse: f32) {
        self.breaking_impulse = impulse;
    }

    pub fn get_body_a(&self) -> Option<&Physics3DBody> {
        self.body_a.and_then(|ptr| if ptr.is_null() { None } else { Some(unsafe { &*ptr }) })
    }

    pub fn get_body_b(&self) -> Option<&Physics3DBody> {
        self.body_b.and_then(|ptr| if ptr.is_null() { None } else { Some(unsafe { &*ptr }) })
    }
}

/// Ray cast result for 3D physics
#[derive(Debug, Clone)]
pub struct RayCastResult {
    pub body: usize,  // Body ID
    pub hit_point: Vec3,
    pub hit_normal: Vec3,
    pub hit_fraction: f32,
}

#[derive(Debug)]
pub struct Physics3DWorld {
    gravity: Vec3,
    simulation_time: f32,
    debug_draw: bool,
    substeps: i32,
    bodies: Vec<Physics3DBody>,
    shapes: Vec<Physics3DShape>,
    constraints: Vec<Physics3DConstraint>,
}

impl Default for Physics3DWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl Physics3DWorld {
    pub fn new() -> Physics3DWorld {
        Physics3DWorld {
            gravity: Vec3::new(0.0, -9.8, 0.0),
            simulation_time: 0.0,
            debug_draw: false,
            substeps: 1,
            bodies: Vec::new(),
            shapes: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn get_gravity(&self) -> Vec3 {
        self.gravity
    }

    pub fn set_gravity(&mut self, gravity: Vec3) {
        self.gravity = gravity;
    }

    pub fn get_simulation_time(&self) -> f32 {
        self.simulation_time
    }

    pub fn get_substeps(&self) -> i32 {
        self.substeps
    }

    pub fn add_body(&mut self, body: &Physics3DBody) {
        // In real implementation, add to physics engine
    }

    pub fn remove_body(&mut self, body: &Physics3DBody) {
        // In real implementation, remove from physics engine
    }

    pub fn set_substeps(&mut self, substeps: i32) {
        self.substeps = substeps.max(1);
    }

    pub fn add_constraint(&mut self, _constraint: &Physics3DConstraint) {
        // In real implementation, add to physics engine
    }

    pub fn remove_constraint(&mut self, _constraint: &Physics3DConstraint) {
        // In real implementation, remove from physics engine
    }

    /// Perform ray cast in 3D world
    pub fn ray_cast(&self, _from: Vec3, _to: Vec3) -> Option<RayCastResult> {
        // Simplified ray casting implementation
        // In real implementation, use Bullet physics
        None
    }

    /// Step the physics simulation
    pub fn step(&mut self, delta: f32) {
        let dt = delta / self.substeps as f32;
        
        for _ in 0..self.substeps {
            // Apply gravity
            for body in &mut self.bodies {
                if body.is_gravity_enabled() && body.get_type() == Physics3DBodyType::DYNAMIC {
                    let force = self.gravity * body.get_mass();
                    body.apply_central_force(force, dt);
                }
            }
            
            // Apply damping
            for body in &mut self.bodies {
                if body.get_type() == Physics3DBodyType::DYNAMIC {
                    let linear_damping = 1.0 - body.get_linear_damping() * dt;
                    let angular_damping = 1.0 - body.get_angular_damping() * dt;
                    
                    let vel = body.get_linear_velocity();
                    body.set_linear_velocity(vel * linear_damping.max(0.0));
                    
                    let ang_vel = body.get_angular_velocity();
                    body.set_angular_velocity(ang_vel * angular_damping.max(0.0));
                }
            }
            
            // Update positions (simplified Euler integration)
            for body in &mut self.bodies {
                if body.get_type() == Physics3DBodyType::DYNAMIC && body.is_enabled() {
                    let pos = body.get_position();
                    let vel = body.get_linear_velocity();
                    body.set_position(pos + vel * dt);
                    
                    // Integrate rotation using quaternion
                    let ang_vel = body.get_angular_velocity();
                    if ang_vel.length() > 0.0001 {
                        let angle = ang_vel.length() * dt;
                        let mut axis = ang_vel;
                        axis.normalize();
                        let delta_rot = Quaternion::from_axis_angle(axis, angle);
                        let rot = body.get_rotation();
                        body.set_rotation(delta_rot * rot);
                    }
                }
            }
            
            self.simulation_time += dt;
        }
    }

    pub fn set_debug_draw_enabled(&mut self, enabled: bool) {
        self.debug_draw = enabled;
    }

    pub fn is_debug_draw_enabled(&self) -> bool {
        self.debug_draw
    }

    pub fn get_all_bodies(&self) -> &[Physics3DBody] {
        &self.bodies
    }
}

// Navigation Mesh support (NavMesh)
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

impl Default for NavMeshPath {
    fn default() -> Self {
        Self::new()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_3d_material() {
        let material = Physics3DMaterial::new(0.5, 0.3);
        assert_eq!(material.friction, 0.5);
        assert_eq!(material.restitution, 0.3);
        assert_eq!(material.rolling_friction, 0.0);
        assert_eq!(material.spinning_friction, 0.0);
    }

    #[test]
    fn test_physics_3d_shape_creation() {
        let box_shape = Physics3DShape::create_box(Vec3::new(2.0, 3.0, 4.0));
        assert_eq!(box_shape.get_type(), Physics3DShapeType::BOX);

        let sphere = Physics3DShape::create_sphere(1.5);
        assert_eq!(sphere.get_type(), Physics3DShapeType::SPHERE);

        let capsule = Physics3DShape::create_capsule(0.5, 2.0);
        assert_eq!(capsule.get_type(), Physics3DShapeType::CAPSULE);

        let cylinder = Physics3DShape::create_cylinder(1.0, 3.0);
        assert_eq!(cylinder.get_type(), Physics3DShapeType::CYLINDER);

        let cone = Physics3DShape::create_cone(1.0, 2.0);
        assert_eq!(cone.get_type(), Physics3DShapeType::CONE);
    }

    #[test]
    fn test_physics_3d_body_types() {
        let static_body = Physics3DBody::create_static();
        assert_eq!(static_body.get_type(), Physics3DBodyType::STATIC);
        assert_eq!(static_body.get_mass(), 0.0);
        assert!(!static_body.is_gravity_enabled());

        let dynamic_body = Physics3DBody::create_dynamic(50.0);
        assert_eq!(dynamic_body.get_type(), Physics3DBodyType::DYNAMIC);
        assert_eq!(dynamic_body.get_mass(), 50.0);
        assert!(dynamic_body.is_gravity_enabled());
    }

    #[test]
    fn test_physics_3d_body_velocity() {
        let mut body = Physics3DBody::new();
        
        body.set_linear_velocity(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(body.get_linear_velocity(), Vec3::new(1.0, 2.0, 3.0));

        body.set_angular_velocity(Vec3::new(0.1, 0.2, 0.3));
        assert_eq!(body.get_angular_velocity(), Vec3::new(0.1, 0.2, 0.3));
    }

    #[test]
    fn test_physics_3d_body_damping() {
        let mut body = Physics3DBody::new();
        
        assert_eq!(body.get_linear_damping(), 0.0);
        assert_eq!(body.get_angular_damping(), 0.0);

        body.set_linear_damping(0.1);
        body.set_angular_damping(0.05);

        assert_eq!(body.get_linear_damping(), 0.1);
        assert_eq!(body.get_angular_damping(), 0.05);
    }

    #[test]
    fn test_physics_3d_body_impulse() {
        let mut body = Physics3DBody::create_dynamic(10.0);
        body.set_linear_velocity(Vec3::ZERO);

        let impulse = Vec3::new(50.0, 0.0, 0.0);
        body.apply_central_impulse(impulse);

        // After impulse, velocity should be impulse / mass
        assert_eq!(body.get_linear_velocity(), Vec3::new(5.0, 0.0, 0.0));
    }

    #[test]
    fn test_physics_3d_world() {
        let mut world = Physics3DWorld::new();
        
        assert_eq!(world.get_gravity(), Vec3::new(0.0, -9.8, 0.0));
        
        world.set_gravity(Vec3::new(0.0, -10.0, 0.0));
        assert_eq!(world.get_gravity(), Vec3::new(0.0, -10.0, 0.0));

        assert_eq!(world.get_substeps(), 1);
        world.set_substeps(4);
        assert_eq!(world.get_substeps(), 4);

        assert!(!world.is_debug_draw_enabled());
        world.set_debug_draw_enabled(true);
        assert!(world.is_debug_draw_enabled());
    }

    #[test]
    fn test_physics_3d_constraint_types() {
        let body_a = Physics3DBody::new();
        let body_b = Physics3DBody::new();

        let p2p = Physics3DConstraint::create_point_to_point(&body_a, &body_b, Vec3::ZERO, Vec3::ZERO);
        assert_eq!(p2p.get_type(), Physics3DConstraintType::PointToPoint);
        assert!(p2p.is_enabled());

        let hinge = Physics3DConstraint::create_hinge(&body_a, &body_b, 
            Vec3::ZERO, Vec3::ZERO, Vec3::UNIT_Y, Vec3::UNIT_Y);
        assert_eq!(hinge.get_type(), Physics3DConstraintType::HINGE);

        let slider = Physics3DConstraint::create_slider(&body_a, &body_b);
        assert_eq!(slider.get_type(), Physics3DConstraintType::SLIDER);
    }

    #[test]
    fn test_physics_3d_constraint_breaking_impulse() {
        let mut constraint = Physics3DConstraint::new(Physics3DConstraintType::FIXED);
        
        assert_eq!(constraint.get_breaking_impulse(), f32::INFINITY);
        
        constraint.set_breaking_impulse(100.0);
        assert_eq!(constraint.get_breaking_impulse(), 100.0);
    }

    #[test]
    fn test_nav_mesh_path() {
        let path = NavMeshPath::new();
        assert_eq!(path.get_corners().len(), 0);
        assert_eq!(path.get_length(), 0.0);
    }
}
