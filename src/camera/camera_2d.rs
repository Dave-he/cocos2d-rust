use crate::base::Node;
use crate::math::{Vec2, Vec3, Mat4};
use std::sync::{Arc, Mutex};

/// 2D 相机
/// 
/// 提供 2D 场景的视图控制，包括：
/// - 位置控制（平移）
/// - 缩放（Zoom）
/// - 旋转
/// - 跟随目标
/// - 边界限制
#[derive(Debug, Clone)]
pub struct Camera2D {
    node: Node,
    
    // 相机属性
    position: Vec2,
    zoom: f32,
    rotation: f32, // 角度
    
    // 跟随目标
    follow_target: Option<Arc<Mutex<Node>>>,
    follow_offset: Vec2,
    follow_lerp: f32, // 0.0 ~ 1.0, 跟随平滑度
    
    // 边界限制
    bounds_enabled: bool,
    bounds_min: Vec2,
    bounds_max: Vec2,
    
    // 视口大小
    viewport_size: Vec2,
}

impl Camera2D {
    /// 创建2D相机
    pub fn new() -> Self {
        Camera2D {
            node: Node::new(),
            position: Vec2::ZERO,
            zoom: 1.0,
            rotation: 0.0,
            follow_target: None,
            follow_offset: Vec2::ZERO,
            follow_lerp: 1.0,
            bounds_enabled: false,
            bounds_min: Vec2::ZERO,
            bounds_max: Vec2::ZERO,
            viewport_size: Vec2::new(800.0, 600.0), // 默认视口
        }
    }
    
    /// 创建2D相机
    pub fn create() -> Self {
        Self::new()
    }
    
    // ========== 位置控制 ==========
    
    /// 设置相机位置
    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.apply_bounds();
    }
    
    /// 获取相机位置
    pub fn get_position(&self) -> Vec2 {
        self.position
    }
    
    /// 移动相机
    pub fn move_by(&mut self, delta: Vec2) {
        self.position = self.position + delta;
        self.apply_bounds();
    }
    
    // ========== 缩放控制 ==========
    
    /// 设置缩放级别
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.1).min(10.0); // 限制在合理范围内
    }
    
    /// 获取缩放级别
    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }
    
    /// 缩放by倍数
    pub fn zoom_by(&mut self, factor: f32) {
        self.set_zoom(self.zoom * factor);
    }
    
    // ========== 旋转控制 ==========
    
    /// 设置旋转角度（度）
    pub fn set_rotation(&mut self, degrees: f32) {
        self.rotation = degrees % 360.0;
    }
    
    /// 获取旋转角度（度）
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    
    /// 旋转by角度
    pub fn rotate_by(&mut self, degrees: f32) {
        self.set_rotation(self.rotation + degrees);
    }
    
    // ========== 跟随目标 ==========
    
    /// 设置跟随目标
    pub fn set_follow_target(&mut self, target: Option<Arc<Mutex<Node>>>) {
        self.follow_target = target;
    }
    
    /// 获取跟随目标
    pub fn get_follow_target(&self) -> Option<Arc<Mutex<Node>>> {
        self.follow_target.clone()
    }
    
    /// 设置跟随偏移
    pub fn set_follow_offset(&mut self, offset: Vec2) {
        self.follow_offset = offset;
    }
    
    /// 获取跟随偏移
    pub fn get_follow_offset(&self) -> Vec2 {
        self.follow_offset
    }
    
    /// 设置跟随平滑度 (0.0 ~ 1.0)
    /// 1.0 = 立即跟随, 0.1 = 平滑跟随
    pub fn set_follow_lerp(&mut self, lerp: f32) {
        self.follow_lerp = lerp.clamp(0.0, 1.0);
    }
    
    /// 获取跟随平滑度
    pub fn get_follow_lerp(&self) -> f32 {
        self.follow_lerp
    }
    
    // ========== 边界限制 ==========
    
    /// 启用边界限制
    pub fn enable_bounds(&mut self, min: Vec2, max: Vec2) {
        self.bounds_enabled = true;
        self.bounds_min = min;
        self.bounds_max = max;
        self.apply_bounds();
    }
    
    /// 禁用边界限制
    pub fn disable_bounds(&mut self) {
        self.bounds_enabled = false;
    }
    
    /// 是否启用边界
    pub fn is_bounds_enabled(&self) -> bool {
        self.bounds_enabled
    }
    
    /// 获取边界范围
    pub fn get_bounds(&self) -> (Vec2, Vec2) {
        (self.bounds_min, self.bounds_max)
    }
    
    /// 应用边界限制
    fn apply_bounds(&mut self) {
        if !self.bounds_enabled {
            return;
        }
        
        self.position.x = self.position.x.clamp(self.bounds_min.x, self.bounds_max.x);
        self.position.y = self.position.y.clamp(self.bounds_min.y, self.bounds_max.y);
    }
    
    // ========== 视口控制 ==========
    
    /// 设置视口大小
    pub fn set_viewport_size(&mut self, size: Vec2) {
        self.viewport_size = size;
    }
    
    /// 获取视口大小
    pub fn get_viewport_size(&self) -> Vec2 {
        self.viewport_size
    }
    
    // ========== 更新 ==========
    
    /// 更新相机（每帧调用）
    /// delta_time: 时间增量（秒）
    pub fn update(&mut self, delta_time: f32) {
        // 计算目标位置（在借用之前）
        let target_position = if let Some(target) = &self.follow_target {
            if let Ok(target_node) = target.lock() {
                // 复制 Vec2 值（Vec2 实现了 Copy）
                let pos = target_node.get_position();
                Some(Vec2::new(pos.x, pos.y))
            } else {
                None
            }
        } else {
            None
        };
        
        // 更新位置
        if let Some(target_pos) = target_position {
            let desired_pos = Vec2::new(
                target_pos.x + self.follow_offset.x,
                target_pos.y + self.follow_offset.y,
            );
            
            // 平滑跟随
            if self.follow_lerp >= 0.99 {
                self.position = desired_pos;
            } else {
                let lerp_factor = 1.0 - (1.0 - self.follow_lerp).powf(delta_time * 60.0);
                self.position = self.position.lerp(&desired_pos, lerp_factor);
            }
            
            self.apply_bounds();
        }
    }
    
    // ========== 坐标转换 ==========
    
    /// 世界坐标转屏幕坐标
    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let relative = world_pos - self.position;
        let scaled = Vec2::new(
            relative.x * self.zoom,
            relative.y * self.zoom,
        );
        
        // 应用旋转（如果需要）
        if self.rotation != 0.0 {
            let angle_rad = self.rotation.to_radians();
            let cos = angle_rad.cos();
            let sin = angle_rad.sin();
            
            Vec2::new(
                scaled.x * cos - scaled.y * sin,
                scaled.x * sin + scaled.y * cos,
            ) + self.viewport_size / 2.0
        } else {
            scaled + self.viewport_size / 2.0
        }
    }
    
    /// 屏幕坐标转世界坐标
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let centered = screen_pos - self.viewport_size / 2.0;
        
        // 反向旋转
        let unrotated = if self.rotation != 0.0 {
            let angle_rad = -self.rotation.to_radians();
            let cos = angle_rad.cos();
            let sin = angle_rad.sin();
            
            Vec2::new(
                centered.x * cos - centered.y * sin,
                centered.x * sin + centered.y * cos,
            )
        } else {
            centered
        };
        
        // 反向缩放
        let unscaled = Vec2::new(
            unrotated.x / self.zoom,
            unrotated.y / self.zoom,
        );
        
        unscaled + self.position
    }
    
    /// 获取视图矩阵
    pub fn get_view_matrix(&self) -> Mat4 {
        // 这里返回一个简化的视图矩阵
        // 实际实现需要考虑位置、缩放、旋转
        let mut matrix = Mat4::IDENTITY;
        
        // 平移
        matrix.translate(-self.position.x, -self.position.y, 0.0);
        
        // 缩放
        matrix.scale(self.zoom, self.zoom, 1.0);
        
        matrix
    }
    
    /// 获取节点
    pub fn get_node(&self) -> &Node {
        &self.node
    }
    
    /// 获取可变节点
    pub fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Default for Camera2D {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_camera_creation() {
        let camera = Camera2D::new();
        assert_eq!(camera.get_position(), Vec2::ZERO);
        assert_eq!(camera.get_zoom(), 1.0);
        assert_eq!(camera.get_rotation(), 0.0);
    }
    
    #[test]
    fn test_position_control() {
        let mut camera = Camera2D::new();
        
        camera.set_position(Vec2::new(100.0, 200.0));
        assert_eq!(camera.get_position(), Vec2::new(100.0, 200.0));
        
        camera.move_by(Vec2::new(50.0, -50.0));
        assert_eq!(camera.get_position(), Vec2::new(150.0, 150.0));
    }
    
    #[test]
    fn test_zoom_control() {
        let mut camera = Camera2D::new();
        
        camera.set_zoom(2.0);
        assert_eq!(camera.get_zoom(), 2.0);
        
        camera.zoom_by(0.5);
        assert_eq!(camera.get_zoom(), 1.0);
        
        // 测试边界
        camera.set_zoom(0.05); // 太小
        assert_eq!(camera.get_zoom(), 0.1);
        
        camera.set_zoom(15.0); // 太大
        assert_eq!(camera.get_zoom(), 10.0);
    }
    
    #[test]
    fn test_rotation_control() {
        let mut camera = Camera2D::new();
        
        camera.set_rotation(45.0);
        assert_eq!(camera.get_rotation(), 45.0);
        
        camera.rotate_by(30.0);
        assert_eq!(camera.get_rotation(), 75.0);
        
        // 测试循环
        camera.set_rotation(370.0);
        assert_eq!(camera.get_rotation(), 10.0);
    }
    
    #[test]
    fn test_follow_offset() {
        let mut camera = Camera2D::new();
        
        let offset = Vec2::new(10.0, -20.0);
        camera.set_follow_offset(offset);
        
        assert_eq!(camera.get_follow_offset(), offset);
    }
    
    #[test]
    fn test_follow_lerp() {
        let mut camera = Camera2D::new();
        
        camera.set_follow_lerp(0.5);
        assert_eq!(camera.get_follow_lerp(), 0.5);
        
        // 测试边界
        camera.set_follow_lerp(-0.1);
        assert_eq!(camera.get_follow_lerp(), 0.0);
        
        camera.set_follow_lerp(1.5);
        assert_eq!(camera.get_follow_lerp(), 1.0);
    }
    
    #[test]
    fn test_bounds() {
        let mut camera = Camera2D::new();
        
        let min = Vec2::new(-500.0, -500.0);
        let max = Vec2::new(500.0, 500.0);
        
        camera.enable_bounds(min, max);
        assert!(camera.is_bounds_enabled());
        
        let (got_min, got_max) = camera.get_bounds();
        assert_eq!(got_min, min);
        assert_eq!(got_max, max);
        
        // 测试边界限制
        camera.set_position(Vec2::new(1000.0, 1000.0));
        assert_eq!(camera.get_position(), max);
        
        camera.set_position(Vec2::new(-1000.0, -1000.0));
        assert_eq!(camera.get_position(), min);
        
        camera.disable_bounds();
        assert!(!camera.is_bounds_enabled());
    }
    
    #[test]
    fn test_viewport_size() {
        let mut camera = Camera2D::new();
        
        let size = Vec2::new(1920.0, 1080.0);
        camera.set_viewport_size(size);
        
        assert_eq!(camera.get_viewport_size(), size);
    }
    
    #[test]
    fn test_world_to_screen() {
        let mut camera = Camera2D::new();
        camera.set_viewport_size(Vec2::new(800.0, 600.0));
        camera.set_position(Vec2::ZERO);
        camera.set_zoom(1.0);
        
        // 原点应该在屏幕中心
        let screen_pos = camera.world_to_screen(Vec2::ZERO);
        assert_eq!(screen_pos, Vec2::new(400.0, 300.0));
    }
    
    #[test]
    fn test_screen_to_world() {
        let mut camera = Camera2D::new();
        camera.set_viewport_size(Vec2::new(800.0, 600.0));
        camera.set_position(Vec2::ZERO);
        camera.set_zoom(1.0);
        
        // 屏幕中心应该对应世界原点
        let world_pos = camera.screen_to_world(Vec2::new(400.0, 300.0));
        assert!((world_pos.x - 0.0).abs() < 0.01);
        assert!((world_pos.y - 0.0).abs() < 0.01);
    }
    
    #[test]
    fn test_coordinate_conversion_roundtrip() {
        let mut camera = Camera2D::new();
        camera.set_viewport_size(Vec2::new(800.0, 600.0));
        camera.set_position(Vec2::new(100.0, 50.0));
        camera.set_zoom(1.5);
        
        let world_pos = Vec2::new(200.0, 150.0);
        let screen_pos = camera.world_to_screen(world_pos);
        let back_to_world = camera.screen_to_world(screen_pos);
        
        assert!((back_to_world.x - world_pos.x).abs() < 0.1);
        assert!((back_to_world.y - world_pos.y).abs() < 0.1);
    }
}

