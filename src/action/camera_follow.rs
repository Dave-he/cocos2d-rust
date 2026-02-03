/// CameraFollow - 相机跟随动作
/// 
/// 让相机跟随指定的目标节点，支持：
/// - 平滑跟随（线性插值）
/// - 固定偏移
/// - 边界限制
/// - 目标丢失处理

use crate::base::{Node, RefPtr};
use crate::math::Vec2;

#[derive(Debug, Clone)]
pub struct CameraFollow {
    target: RefPtr<Node>,
    offset: Vec2,
    lerp_factor: f32,
    bounds_enabled: bool,
    bounds_min: Vec2,
    bounds_max: Vec2,
    world_rect: (f32, f32, f32, f32),
    boundary_set: bool,
    camera: Option<RefPtr<Node>>,
}

impl CameraFollow {
    pub fn with_target(target: RefPtr<Node>) -> Self {
        Self {
            target,
            offset: Vec2::ZERO,
            lerp_factor: 1.0,
            bounds_enabled: false,
            bounds_min: Vec2::ZERO,
            bounds_max: Vec2::ZERO,
            world_rect: (0.0, 0.0, 0.0, 0.0),
            boundary_set: false,
            camera: None,
        }
    }

    pub fn with_target_and_offset(target: RefPtr<Node>, offset: Vec2) -> Self {
        Self {
            target,
            offset,
            lerp_factor: 1.0,
            bounds_enabled: false,
            bounds_min: Vec2::ZERO,
            bounds_max: Vec2::ZERO,
            world_rect: (0.0, 0.0, 0.0, 0.0),
            boundary_set: false,
            camera: None,
        }
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_lerp(mut self, lerp_factor: f32) -> Self {
        self.lerp_factor = lerp_factor.clamp(0.0, 1.0);
        self
    }

    pub fn with_boundary(mut self, min: Vec2, max: Vec2) -> Self {
        self.bounds_enabled = true;
        self.bounds_min = min;
        self.bounds_max = max;
        self
    }

    pub fn with_world_rect(mut self, left: f32, bottom: f32, right: f32, top: f32) -> Self {
        self.bounds_enabled = true;
        self.boundary_set = true;
        self.world_rect = (left, bottom, right, top);
        self
    }

    pub fn set_camera(&mut self, camera: RefPtr<Node>) {
        self.camera = Some(camera);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_follow_creation() {
        let node = RefPtr::new(Node::new());
        let follow = CameraFollow::with_target(node);
        assert_eq!(follow.lerp_factor, 1.0);
        assert!(!follow.bounds_enabled);
    }

    #[test]
    fn test_camera_follow_with_offset() {
        let node = RefPtr::new(Node::new());
        let offset = Vec2::new(100.0, 50.0);
        let follow = CameraFollow::with_target_and_offset(node, offset);
        assert_eq!(follow.offset.x, 100.0);
        assert_eq!(follow.offset.y, 50.0);
    }

    #[test]
    fn test_camera_follow_with_lerp() {
        let node = RefPtr::new(Node::new());
        let follow = CameraFollow::with_target(node).with_lerp(0.5);
        assert_eq!(follow.lerp_factor, 0.5);
    }

    #[test]
    fn test_camera_follow_with_boundary() {
        let node = RefPtr::new(Node::new());
        let follow = CameraFollow::with_target(node)
            .with_boundary(Vec2::new(0.0, 0.0), Vec2::new(800.0, 600.0));
        assert!(follow.bounds_enabled);
        assert_eq!(follow.bounds_min.x, 0.0);
        assert_eq!(follow.bounds_max.x, 800.0);
    }

    #[test]
    fn test_camera_follow_clone() {
        let node = RefPtr::new(Node::new());
        let follow = CameraFollow::with_target(node)
            .with_offset(Vec2::new(10.0, 20.0));
        let cloned = follow.clone();
        assert_eq!(cloned.offset.x, 10.0);
        assert_eq!(cloned.offset.y, 20.0);
    }
}
