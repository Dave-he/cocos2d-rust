pub mod physics_2d;
pub mod physics_3d;

pub use physics_2d::{PhysicsWorld, PhysicsBody, PhysicsShape, PhysicsJoint};
pub use physics_3d::{Physics3DWorld, Physics3DShape, Physics3DBody};
