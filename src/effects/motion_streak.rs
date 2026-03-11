use crate::base::Node;
use crate::math::Vec2;
use crate::base::types::Color4F;

/// 运动轨迹特效
/// 
/// 为移动的节点添加拖尾效果
pub struct MotionStreak {
    node: Node,
    fade_time: f32,
    min_seg: f32,
    stroke: f32,
    points: Vec<Vec2>,
    color: Color4F,
    last_position: Option<Vec2>,
    is_starting_initialized: bool,
}

impl std::fmt::Debug for MotionStreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MotionStreak")
            .field("fade_time", &self.fade_time)
            .field("stroke", &self.stroke)
            .finish()
    }
}

impl MotionStreak {
    /// 创建运动轨迹
    pub fn create(fade_time: f32, min_seg: f32, stroke: f32) -> Self {
        MotionStreak {
            node: Node::new(),
            fade_time,
            min_seg,
            stroke,
            points: Vec::new(),
            color: Color4F::WHITE,
            last_position: None,
            is_starting_initialized: false,
        }
    }

    /// 创建带颜色的运动轨迹
    pub fn new(fade_time: f32, min_seg: f32, stroke: f32, color: Color4F, _texture: Option<()>) -> Self {
        let mut streak = Self::create(fade_time, min_seg, stroke);
        streak.color = color;
        streak
    }
    
    /// 添加点
    pub fn add_point(&mut self, point: Vec2) {
        self.points.push(point);
    }
    
    /// 清除轨迹
    pub fn reset(&mut self) {
        self.points.clear();
        self.last_position = None;
        self.is_starting_initialized = false;
    }
    
    /// 更新轨迹（传入当前位置）
    pub fn update(&mut self, _delta: f32, position: Vec2) {
        if let Some(last_pos) = self.last_position {
            let dist = {
                let dx = position.x - last_pos.x;
                let dy = position.y - last_pos.y;
                (dx * dx + dy * dy).sqrt()
            };
            if dist >= self.min_seg {
                self.points.push(position);
                self.last_position = Some(position);
            }
        } else {
            self.last_position = Some(position);
            self.is_starting_initialized = true;
        }
    }

    /// 获取淡出时间
    pub fn get_fade_time(&self) -> f32 {
        self.fade_time
    }

    /// 设置淡出时间
    pub fn set_fade_time(&mut self, fade_time: f32) {
        self.fade_time = fade_time;
    }

    /// 获取最小间距
    pub fn get_min_seg(&self) -> f32 {
        self.min_seg
    }

    /// 获取线条宽度
    pub fn get_stroke(&self) -> f32 {
        self.stroke
    }

    /// 获取颜色
    pub fn get_color(&self) -> Color4F {
        self.color
    }

    /// 设置颜色
    pub fn set_color(&mut self, color: Color4F) {
        self.color = color;
    }

    /// 着色（别名 set_color）
    pub fn tint(&mut self, color: Color4F) {
        self.color = color;
    }

    /// 获取当前轨迹点数量
    pub fn get_point_count(&self) -> usize {
        self.points.len()
    }

    /// 是否已经初始化了起始位置
    pub fn is_starting_position_initialized(&self) -> bool {
        self.is_starting_initialized
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

impl Default for MotionStreak {
    fn default() -> Self {
        Self::create(1.0, 1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_streak_creation() {
        let streak = MotionStreak::create(2.0, 0.5, 3.0);
        assert_eq!(streak.fade_time, 2.0);
        assert_eq!(streak.min_seg, 0.5);
        assert_eq!(streak.stroke, 3.0);
    }

    #[test]
    fn test_motion_streak_default() {
        let streak = MotionStreak::default();
        assert_eq!(streak.fade_time, 1.0);
        assert_eq!(streak.min_seg, 1.0);
        assert_eq!(streak.stroke, 1.0);
    }

    #[test]
    fn test_add_point() {
        let mut streak = MotionStreak::create(1.0, 1.0, 1.0);
        streak.add_point(Vec2::new(100.0, 200.0));
        streak.add_point(Vec2::new(150.0, 250.0));
        assert_eq!(streak.points.len(), 2);
    }

    #[test]
    fn test_reset() {
        let mut streak = MotionStreak::create(1.0, 1.0, 1.0);
        streak.add_point(Vec2::new(100.0, 200.0));
        streak.add_point(Vec2::new(150.0, 250.0));
        streak.reset();
        assert_eq!(streak.points.len(), 0);
    }

    #[test]
    fn test_get_node() {
        let streak = MotionStreak::create(1.0, 1.0, 1.0);
        let _node = streak.get_node();
    }
}
