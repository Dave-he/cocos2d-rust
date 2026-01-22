use crate::math::Vec2;
use std::time::Instant;

/// 唯一的触摸 ID
pub type TouchId = u64;

/// 触摸阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchPhase {
    /// 触摸开始
    Began,
    /// 触摸移动
    Moved,
    /// 触摸静止
    Stationary,
    /// 触摸结束
    Ended,
    /// 触摸取消
    Cancelled,
}

/// 触摸事件
#[derive(Debug, Clone)]
pub struct Touch {
    /// 触摸 ID
    id: TouchId,
    /// 当前位置（屏幕坐标）
    location: Vec2,
    /// 前一个位置
    previous_location: Vec2,
    /// 起始位置
    start_location: Vec2,
    /// 触摸阶段
    phase: TouchPhase,
    /// 触摸开始时间
    start_time: Instant,
    /// 当前时间戳
    timestamp: Instant,
    /// 压力（0.0-1.0，部分设备支持）
    pressure: f32,
}

impl Touch {
    /// 创建新的触摸
    pub fn new(id: TouchId, location: Vec2) -> Self {
        let now = Instant::now();
        Self {
            id,
            location,
            previous_location: location,
            start_location: location,
            phase: TouchPhase::Began,
            start_time: now,
            timestamp: now,
            pressure: 1.0,
        }
    }

    /// 获取触摸 ID
    pub fn id(&self) -> TouchId {
        self.id
    }

    /// 获取当前位置
    pub fn location(&self) -> Vec2 {
        self.location
    }

    /// 获取视图坐标系中的位置
    pub fn location_in_view(&self) -> Vec2 {
        // 默认与 location 相同
        // 实际使用中可能需要坐标转换
        self.location
    }

    /// 获取前一个位置
    pub fn previous_location(&self) -> Vec2 {
        self.previous_location
    }

    /// 获取前一个位置（视图坐标）
    pub fn previous_location_in_view(&self) -> Vec2 {
        self.previous_location
    }

    /// 获取起始位置
    pub fn start_location(&self) -> Vec2 {
        self.start_location
    }

    /// 获取起始位置（视图坐标）
    pub fn start_location_in_view(&self) -> Vec2 {
        self.start_location
    }

    /// 获取触摸阶段
    pub fn phase(&self) -> TouchPhase {
        self.phase
    }

    /// 获取触摸持续时间
    pub fn duration(&self) -> std::time::Duration {
        self.timestamp.duration_since(self.start_time)
    }

    /// 获取压力
    pub fn pressure(&self) -> f32 {
        self.pressure
    }

    /// 获取时间戳
    pub fn timestamp(&self) -> Instant {
        self.timestamp
    }

    /// 获取移动增量
    pub fn delta(&self) -> Vec2 {
        self.location - self.previous_location
    }

    /// 更新触摸位置
    pub fn update_location(&mut self, location: Vec2, phase: TouchPhase) {
        self.previous_location = self.location;
        self.location = location;
        self.phase = phase;
        self.timestamp = Instant::now();
    }

    /// 设置压力值
    pub fn set_pressure(&mut self, pressure: f32) {
        self.pressure = pressure.clamp(0.0, 1.0);
    }

    /// 结束触摸
    pub fn end(&mut self) {
        self.phase = TouchPhase::Ended;
        self.timestamp = Instant::now();
    }

    /// 取消触摸
    pub fn cancel(&mut self) {
        self.phase = TouchPhase::Cancelled;
        self.timestamp = Instant::now();
    }
}

impl PartialEq for Touch {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Touch {}

impl std::hash::Hash for Touch {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touch_creation() {
        let touch = Touch::new(1, Vec2::new(100.0, 200.0));
        assert_eq!(touch.id(), 1);
        assert_eq!(touch.location(), Vec2::new(100.0, 200.0));
        assert_eq!(touch.phase(), TouchPhase::Began);
    }

    #[test]
    fn test_touch_update() {
        let mut touch = Touch::new(1, Vec2::new(100.0, 200.0));
        touch.update_location(Vec2::new(150.0, 250.0), TouchPhase::Moved);
        
        assert_eq!(touch.location(), Vec2::new(150.0, 250.0));
        assert_eq!(touch.previous_location(), Vec2::new(100.0, 200.0));
        assert_eq!(touch.delta(), Vec2::new(50.0, 50.0));
        assert_eq!(touch.phase(), TouchPhase::Moved);
    }

    #[test]
    fn test_touch_pressure() {
        let mut touch = Touch::new(1, Vec2::new(100.0, 200.0));
        
        touch.set_pressure(0.5);
        assert_eq!(touch.pressure(), 0.5);
        
        // 测试压力范围限制
        touch.set_pressure(1.5);
        assert_eq!(touch.pressure(), 1.0);
        
        touch.set_pressure(-0.5);
        assert_eq!(touch.pressure(), 0.0);
    }
}
