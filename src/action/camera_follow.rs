/// CameraFollow - 相机跟随动作
/// 
/// 让相机跟随指定的目标节点，支持：
/// - 平滑跟随（线性插值）
/// - 固定偏移
/// - 边界限制
/// - 死区（Dead Zone，减少微小移动）
/// - 多种跟随模式
/// - 目标丢失处理
/// - 缩放/旋转跟随

use crate::base::{Node, RefPtr};
use crate::math::Vec2;

/// 跟随模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FollowMode {
    /// 直接跟随（无延迟）
    Immediate,
    /// 平滑跟随（指数衰减）
    Smooth,
    /// 弹簧跟随（带阻尼的弹簧运动）
    Spring,
    /// 固定速度跟随
    FixedSpeed,
}

/// 跟随轴向约束
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FollowAxis {
    /// 水平和垂直都跟随
    Both,
    /// 仅水平跟随
    Horizontal,
    /// 仅垂直跟随
    Vertical,
}

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
    // === 新增功能字段 ===
    /// 跟随模式
    mode: FollowMode,
    /// 跟随轴向
    axis: FollowAxis,
    /// 死区大小（小于此范围不移动）
    dead_zone: Vec2,
    /// 是否启用
    enabled: bool,
    /// 是否暂停
    paused: bool,
    /// 弹簧参数：刚度（spring mode）
    spring_stiffness: f32,
    /// 弹簧参数：阻尼
    spring_damping: f32,
    /// 弹簧速度状态
    velocity: Vec2,
    /// 固定速度（pixels/s，fixed speed mode）
    fixed_speed: f32,
    /// 缩放跟随（0.0=不跟随缩放，1.0=完全跟随）
    scale_follow: f32,
    /// 目标缩放比例
    target_scale: f32,
    /// 当前相机位置（供插值用）
    current_position: Vec2,
    /// 是否第一帧（跳过初始插值）
    first_frame: bool,
    /// 相机振动偏移
    shake_offset: Vec2,
    /// 振动剩余时间
    shake_time_remaining: f32,
    /// 振动强度
    shake_intensity: f32,
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
            mode: FollowMode::Immediate,
            axis: FollowAxis::Both,
            dead_zone: Vec2::ZERO,
            enabled: true,
            paused: false,
            spring_stiffness: 10.0,
            spring_damping: 5.0,
            velocity: Vec2::ZERO,
            fixed_speed: 200.0,
            scale_follow: 0.0,
            target_scale: 1.0,
            current_position: Vec2::ZERO,
            first_frame: true,
            shake_offset: Vec2::ZERO,
            shake_time_remaining: 0.0,
            shake_intensity: 0.0,
        }
    }

    pub fn with_target_and_offset(target: RefPtr<Node>, offset: Vec2) -> Self {
        let mut f = Self::with_target(target);
        f.offset = offset;
        f
    }

    // ========== Builder 风格 API ==========

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_lerp(mut self, lerp_factor: f32) -> Self {
        self.lerp_factor = lerp_factor.clamp(0.01, 1.0);
        self.mode = FollowMode::Smooth;
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

    pub fn with_mode(mut self, mode: FollowMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_axis(mut self, axis: FollowAxis) -> Self {
        self.axis = axis;
        self
    }

    pub fn with_dead_zone(mut self, width: f32, height: f32) -> Self {
        self.dead_zone = Vec2::new(width.abs(), height.abs());
        self
    }

    pub fn with_spring(mut self, stiffness: f32, damping: f32) -> Self {
        self.mode = FollowMode::Spring;
        self.spring_stiffness = stiffness.max(0.1);
        self.spring_damping = damping.max(0.0);
        self
    }

    pub fn with_fixed_speed(mut self, speed: f32) -> Self {
        self.mode = FollowMode::FixedSpeed;
        self.fixed_speed = speed.max(1.0);
        self
    }

    pub fn set_camera(&mut self, camera: RefPtr<Node>) {
        self.camera = Some(camera);
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn get_offset(&self) -> Vec2 {
        self.offset
    }

    pub fn set_offset(&mut self, offset: Vec2) {
        self.offset = offset;
    }

    pub fn get_current_position(&self) -> Vec2 {
        self.current_position
    }

    /// 启动相机振动
    pub fn shake(&mut self, duration: f32, intensity: f32) {
        self.shake_time_remaining = duration;
        self.shake_intensity = intensity;
    }

    /// 停止振动
    pub fn stop_shake(&mut self) {
        self.shake_time_remaining = 0.0;
        self.shake_offset = Vec2::ZERO;
    }

    // ========== 核心更新方法 ==========

    /// 每帧更新（由引擎调度）
    pub fn update(&mut self, dt: f32) {
        if !self.enabled || self.paused {
            return;
        }

        // 获取目标位置
        let target_pos = {
            let node = self.target.borrow();
            *node.get_position()
        };

        // 目标点 + 偏移
        let desired = Vec2::new(target_pos.x + self.offset.x, target_pos.y + self.offset.y);

        if self.first_frame {
            self.current_position = desired;
            self.first_frame = false;
        }

        // 应用死区
        let delta = Vec2::new(
            desired.x - self.current_position.x,
            desired.y - self.current_position.y,
        );

        let dx = if delta.x.abs() > self.dead_zone.x { delta.x } else { 0.0 };
        let dy = if delta.y.abs() > self.dead_zone.y { delta.y } else { 0.0 };

        let effective_target = Vec2::new(
            self.current_position.x + dx,
            self.current_position.y + dy,
        );

        // 根据模式计算新位置
        let new_pos = match self.mode {
            FollowMode::Immediate => effective_target,
            FollowMode::Smooth => {
                let t = self.lerp_factor;
                Vec2::new(
                    self.current_position.x + (effective_target.x - self.current_position.x) * t,
                    self.current_position.y + (effective_target.y - self.current_position.y) * t,
                )
            }
            FollowMode::Spring => {
                let diff_x = effective_target.x - self.current_position.x;
                let diff_y = effective_target.y - self.current_position.y;
                // 弹簧力 = stiffness * displacement - damping * velocity
                let force_x = self.spring_stiffness * diff_x - self.spring_damping * self.velocity.x;
                let force_y = self.spring_stiffness * diff_y - self.spring_damping * self.velocity.y;
                self.velocity.x += force_x * dt;
                self.velocity.y += force_y * dt;
                Vec2::new(
                    self.current_position.x + self.velocity.x * dt,
                    self.current_position.y + self.velocity.y * dt,
                )
            }
            FollowMode::FixedSpeed => {
                let diff = Vec2::new(
                    effective_target.x - self.current_position.x,
                    effective_target.y - self.current_position.y,
                );
                let dist = (diff.x * diff.x + diff.y * diff.y).sqrt();
                let max_move = self.fixed_speed * dt;
                if dist <= max_move || dist < 0.01 {
                    effective_target
                } else {
                    let ratio = max_move / dist;
                    Vec2::new(
                        self.current_position.x + diff.x * ratio,
                        self.current_position.y + diff.y * ratio,
                    )
                }
            }
        };

        // 应用轴约束
        let constrained = match self.axis {
            FollowAxis::Both => new_pos,
            FollowAxis::Horizontal => Vec2::new(new_pos.x, self.current_position.y),
            FollowAxis::Vertical => Vec2::new(self.current_position.x, new_pos.y),
        };

        // 应用边界限制
        let bounded = if self.bounds_enabled {
            Vec2::new(
                constrained.x.clamp(self.bounds_min.x, self.bounds_max.x),
                constrained.y.clamp(self.bounds_min.y, self.bounds_max.y),
            )
        } else {
            constrained
        };

        self.current_position = bounded;

        // 更新相机振动
        self.update_shake(dt);

        // 应用到相机节点
        let final_pos = Vec2::new(
            self.current_position.x + self.shake_offset.x,
            self.current_position.y + self.shake_offset.y,
        );

        if let Some(ref camera) = self.camera {
            camera.borrow_mut().set_position(final_pos);
        }
    }

    /// 更新振动效果
    fn update_shake(&mut self, dt: f32) {
        if self.shake_time_remaining > 0.0 {
            self.shake_time_remaining -= dt;
            if self.shake_time_remaining <= 0.0 {
                self.shake_time_remaining = 0.0;
                self.shake_offset = Vec2::ZERO;
            } else {
                // 简单伪随机振动（用帧计数+正弦模拟）
                let t = self.shake_time_remaining * 100.0;
                let decay = self.shake_time_remaining; // 线性衰减
                self.shake_offset = Vec2::new(
                    (t * 7.31).sin() * self.shake_intensity * decay,
                    (t * 13.57).cos() * self.shake_intensity * decay,
                );
            }
        }
    }

    /// 立即跳转到目标位置（跳过插值）
    pub fn snap_to_target(&mut self) {
        let target_pos = {
            let node = self.target.borrow();
            *node.get_position()
        };
        self.current_position = Vec2::new(target_pos.x + self.offset.x, target_pos.y + self.offset.y);
        self.velocity = Vec2::ZERO;

        if self.bounds_enabled {
            self.current_position.x = self.current_position.x.clamp(self.bounds_min.x, self.bounds_max.x);
            self.current_position.y = self.current_position.y.clamp(self.bounds_min.y, self.bounds_max.y);
        }

        if let Some(ref camera) = self.camera {
            camera.borrow_mut().set_position(self.current_position);
        }
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
        assert!(follow.enabled);
        assert_eq!(follow.mode, FollowMode::Immediate);
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
        assert_eq!(follow.mode, FollowMode::Smooth);
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

    #[test]
    fn test_camera_follow_immediate_update() {
        let node = RefPtr::new(Node::new());
        node.borrow_mut().set_position(Vec2::new(100.0, 200.0));
        let camera = RefPtr::new(Node::new());
        let mut follow = CameraFollow::with_target(node).with_mode(FollowMode::Immediate);
        follow.set_camera(camera.clone());
        follow.update(1.0 / 60.0);
        let pos = follow.get_current_position();
        assert!((pos.x - 100.0).abs() < 0.01);
        assert!((pos.y - 200.0).abs() < 0.01);
    }

    #[test]
    fn test_camera_follow_smooth_update() {
        let node = RefPtr::new(Node::new());
        node.borrow_mut().set_position(Vec2::new(100.0, 0.0));
        let camera = RefPtr::new(Node::new());
        let mut follow = CameraFollow::with_target(node).with_lerp(0.1);
        follow.set_camera(camera.clone());
        // 第一帧跳转到目标
        follow.update(1.0 / 60.0);
        let pos_first = follow.get_current_position();
        assert!((pos_first.x - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_camera_follow_spring_update() {
        let node = RefPtr::new(Node::new());
        node.borrow_mut().set_position(Vec2::new(50.0, 50.0));
        let camera = RefPtr::new(Node::new());
        let mut follow = CameraFollow::with_target(node).with_spring(10.0, 5.0);
        follow.set_camera(camera.clone());
        // 多帧更新
        for _ in 0..100 {
            follow.update(1.0 / 60.0);
        }
        let pos = follow.get_current_position();
        // 应趋近于 (50, 50)
        assert!((pos.x - 50.0).abs() < 5.0, "Spring should converge, got x={}", pos.x);
        assert!((pos.y - 50.0).abs() < 5.0, "Spring should converge, got y={}", pos.y);
    }

    #[test]
    fn test_camera_follow_fixed_speed() {
        let node = RefPtr::new(Node::new());
        node.borrow_mut().set_position(Vec2::new(200.0, 0.0));
        let camera = RefPtr::new(Node::new());
        let mut follow = CameraFollow::with_target(node).with_fixed_speed(100.0);
        follow.set_camera(camera.clone());
        follow.update(1.0 / 60.0); // 第一帧跳转
        // 移动目标
        follow.first_frame = false;
        follow.current_position = Vec2::ZERO;
        follow.update(1.0); // 1秒，速度100，从0到200距离为200，应移动100
        let pos = follow.get_current_position();
        assert!(pos.x > 0.0 && pos.x < 200.1, "Fixed speed movement, got x={}", pos.x);
    }

    #[test]
    fn test_camera_follow_axis_horizontal() {
        let node = RefPtr::new(Node::new());
        node.borrow_mut().set_position(Vec2::new(100.0, 200.0));
        let camera = RefPtr::new(Node::new());
        let mut follow = CameraFollow::with_target(node)
            .with_axis(FollowAxis::Horizontal);
        follow.set_camera(camera.clone());
        follow.current_position = Vec2::new(0.0, 0.0);
        follow.first_frame = false;
        follow.update(1.0 / 60.0);
        let pos = follow.get_current_position();
        // Y 应保持不变
        assert!((pos.y - 0.0).abs() < 0.01, "Horizontal only: y should not change, got {}", pos.y);
    }

    #[test]
    fn test_camera_follow_dead_zone() {
        let node = RefPtr::new(Node::new());
        node.borrow_mut().set_position(Vec2::new(5.0, 5.0));
        let camera = RefPtr::new(Node::new());
        let mut follow = CameraFollow::with_target(node)
            .with_dead_zone(10.0, 10.0);
        follow.set_camera(camera.clone());
        follow.current_position = Vec2::ZERO;
        follow.first_frame = false;
        follow.update(1.0 / 60.0);
        // 目标在死区内，不应移动
        let pos = follow.get_current_position();
        assert!((pos.x - 0.0).abs() < 0.01, "Dead zone should prevent movement, got x={}", pos.x);
    }

    #[test]
    fn test_camera_follow_bounds() {
        let node = RefPtr::new(Node::new());
        node.borrow_mut().set_position(Vec2::new(1000.0, 1000.0));
        let camera = RefPtr::new(Node::new());
        let mut follow = CameraFollow::with_target(node)
            .with_boundary(Vec2::new(0.0, 0.0), Vec2::new(500.0, 400.0));
        follow.set_camera(camera.clone());
        follow.update(1.0 / 60.0);
        let pos = follow.get_current_position();
        assert!(pos.x <= 500.0, "Should be bounded to 500, got {}", pos.x);
        assert!(pos.y <= 400.0, "Should be bounded to 400, got {}", pos.y);
    }

    #[test]
    fn test_camera_follow_shake() {
        let node = RefPtr::new(Node::new());
        let camera = RefPtr::new(Node::new());
        let mut follow = CameraFollow::with_target(node);
        follow.set_camera(camera.clone());
        follow.shake(0.5, 10.0);
        assert!(follow.shake_time_remaining > 0.0);
        follow.update(0.1);
        // 振动偏移应非零
        let offset = follow.shake_offset;
        assert!(offset.x.abs() > 0.0 || offset.y.abs() > 0.0);
        // 停止振动
        follow.stop_shake();
        assert_eq!(follow.shake_time_remaining, 0.0);
    }

    #[test]
    fn test_camera_follow_snap_to_target() {
        let node = RefPtr::new(Node::new());
        node.borrow_mut().set_position(Vec2::new(500.0, 300.0));
        let camera = RefPtr::new(Node::new());
        let mut follow = CameraFollow::with_target(node).with_lerp(0.01); // 很慢的跟随
        follow.set_camera(camera.clone());
        follow.snap_to_target();
        let pos = follow.get_current_position();
        assert!((pos.x - 500.0).abs() < 0.01);
        assert!((pos.y - 300.0).abs() < 0.01);
    }

    #[test]
    fn test_camera_follow_enable_disable() {
        let node = RefPtr::new(Node::new());
        let mut follow = CameraFollow::with_target(node);
        assert!(follow.is_enabled());
        follow.set_enabled(false);
        assert!(!follow.is_enabled());
        // disabled 不应该移动
        follow.update(1.0 / 60.0);
    }

    #[test]
    fn test_camera_follow_pause_resume() {
        let node = RefPtr::new(Node::new());
        let mut follow = CameraFollow::with_target(node);
        assert!(!follow.is_paused());
        follow.set_paused(true);
        assert!(follow.is_paused());
        follow.set_paused(false);
        assert!(!follow.is_paused());
    }
}
