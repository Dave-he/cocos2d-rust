use crate::base::Node;
use crate::math::Vec2;

/// 运动轨迹特效
/// 
/// 为移动的节点添加拖尾效果
pub struct MotionStreak {
    node: Node,
    fade_time: f32,
    min_seg: f32,
    stroke: f32,
    points: Vec<Vec2>,
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
        }
    }
    
    /// 添加点
    pub fn add_point(&mut self, point: Vec2) {
        self.points.push(point);
    }
    
    /// 清除轨迹
    pub fn reset(&mut self) {
        self.points.clear();
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
