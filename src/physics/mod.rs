pub mod physics_2d;
pub mod physics_3d;

pub use physics_2d::{
    PhysicsWorld, PhysicsBody, PhysicsShape, PhysicsJoint, PhysicsContact,
    PhysicsBodyType, PhysicsShapeType, JointType, PhysicsMaterial,
    RayCastInfo, QueryInfo,
};
pub use physics_3d::{Physics3DWorld, Physics3DShape, Physics3DBody, Physics3DShapeType};
