use crate::base::Node;
use crate::math::Vec2;
use crate::sprite::Sprite;

/// 进度条类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressTimerType {
    /// 径向进度（扇形，顺时针）
    Radial,
    /// 条形进度（水平或垂直）
    Bar,
}

/// 进度方向（用于条形进度）
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BarChangeRate {
    pub x: f32,
    pub y: f32,
}

impl BarChangeRate {
    pub fn new(x: f32, y: f32) -> Self {
        BarChangeRate { x, y }
    }
}

/// ProgressTimer - 进度条特效组件
pub struct ProgressTimer {
    node: Node,
    sprite: Option<Sprite>,
    timer_type: ProgressTimerType,
    percentage: f32,
    midpoint: Vec2,
    reverse: bool,
    bar_change_rate: BarChangeRate,
    midpoint_bar: Vec2,
}

impl std::fmt::Debug for ProgressTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgressTimer")
            .field("node", &self.node)
            .field("timer_type", &self.timer_type)
            .field("percentage", &self.percentage)
            .finish()
    }
}

impl ProgressTimer {
    pub fn create(sprite: Sprite) -> Self {
        ProgressTimer {
            node: Node::new(),
            sprite: Some(sprite),
            timer_type: ProgressTimerType::Radial,
            percentage: 0.0,
            midpoint: Vec2::new(0.5, 0.5),
            reverse: false,
            bar_change_rate: BarChangeRate::new(1.0, 0.0),
            midpoint_bar: Vec2::new(0.5, 0.5),
        }
    }
    
    pub fn set_type(&mut self, timer_type: ProgressTimerType) {
        self.timer_type = timer_type;
    }
    
    pub fn get_type(&self) -> ProgressTimerType {
        self.timer_type
    }
    
    pub fn set_percentage(&mut self, percentage: f32) {
        self.percentage = percentage.clamp(0.0, 100.0);
    }
    
    pub fn get_percentage(&self) -> f32 {
        self.percentage
    }
    
    pub fn set_midpoint(&mut self, midpoint: Vec2) {
        self.midpoint = Vec2::new(
            midpoint.x.clamp(0.0, 1.0),
            midpoint.y.clamp(0.0, 1.0),
        );
    }
    
    pub fn get_midpoint(&self) -> Vec2 {
        self.midpoint
    }
    
    pub fn set_reverse_direction(&mut self, reverse: bool) {
        self.reverse = reverse;
    }
    
    pub fn is_reverse_direction(&self) -> bool {
        self.reverse
    }
    
    pub fn set_bar_change_rate(&mut self, rate: BarChangeRate) {
        self.bar_change_rate = rate;
    }
    
    pub fn get_bar_change_rate(&self) -> BarChangeRate {
        self.bar_change_rate
    }
    
    pub fn set_midpoint_bar(&mut self, midpoint: Vec2) {
        self.midpoint_bar = Vec2::new(
            midpoint.x.clamp(0.0, 1.0),
            midpoint.y.clamp(0.0, 1.0),
        );
    }
    
    pub fn get_midpoint_bar(&self) -> Vec2 {
        self.midpoint_bar
    }
    
    pub fn get_sprite(&self) -> Option<&Sprite> {
        self.sprite.as_ref()
    }
    
    pub fn get_sprite_mut(&mut self) -> Option<&mut Sprite> {
        self.sprite.as_mut()
    }
    
    pub fn set_sprite(&mut self, sprite: Sprite) {
        self.sprite = Some(sprite);
    }
    
    pub fn update_progress(&mut self) {
        // Simplified implementation
    }
    
    pub fn get_node(&self) -> &Node {
        &self.node
    }
    
    pub fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Default for ProgressTimer {
    fn default() -> Self {
        ProgressTimer {
            node: Node::new(),
            sprite: None,
            timer_type: ProgressTimerType::Radial,
            percentage: 0.0,
            midpoint: Vec2::new(0.5, 0.5),
            reverse: false,
            bar_change_rate: BarChangeRate::new(1.0, 0.0),
            midpoint_bar: Vec2::new(0.5, 0.5),
        }
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_progress_timer_creation() {
        let sprite = Sprite::default();
        let timer = ProgressTimer::create(sprite);
        assert_eq!(timer.get_percentage(), 0.0);
        assert_eq!(timer.get_type(), ProgressTimerType::Radial);
    }
    
    #[test]
    fn test_set_percentage() {
        let sprite = Sprite::default();
        let mut timer = ProgressTimer::create(sprite);
        
        timer.set_percentage(50.0);
        assert_eq!(timer.get_percentage(), 50.0);
        
        timer.set_percentage(100.0);
        assert_eq!(timer.get_percentage(), 100.0);
        
        timer.set_percentage(150.0);
        assert_eq!(timer.get_percentage(), 100.0);
        
        timer.set_percentage(-10.0);
        assert_eq!(timer.get_percentage(), 0.0);
    }
    
    #[test]
    fn test_set_type() {
        let sprite = Sprite::default();
        let mut timer = ProgressTimer::create(sprite);
        
        assert_eq!(timer.get_type(), ProgressTimerType::Radial);
        
        timer.set_type(ProgressTimerType::Bar);
        assert_eq!(timer.get_type(), ProgressTimerType::Bar);
    }
    
    #[test]
    fn test_radial_midpoint() {
        let sprite = Sprite::default();
        let mut timer = ProgressTimer::create(sprite);
        
        let midpoint = Vec2::new(0.3, 0.7);
        timer.set_midpoint(midpoint);
        
        let result = timer.get_midpoint();
        assert!((result.x - 0.3).abs() < 0.01);
        assert!((result.y - 0.7).abs() < 0.01);
    }
    
    #[test]
    fn test_midpoint_clamping() {
        let sprite = Sprite::default();
        let mut timer = ProgressTimer::create(sprite);
        
        timer.set_midpoint(Vec2::new(-0.5, 1.5));
        let result = timer.get_midpoint();
        
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 1.0);
    }
    
    #[test]
    fn test_reverse_direction() {
        let sprite = Sprite::default();
        let mut timer = ProgressTimer::create(sprite);
        
        assert!(!timer.is_reverse_direction());
        
        timer.set_reverse_direction(true);
        assert!(timer.is_reverse_direction());
        
        timer.set_reverse_direction(false);
        assert!(!timer.is_reverse_direction());
    }
    
    #[test]
    fn test_bar_change_rate() {
        let sprite = Sprite::default();
        let mut timer = ProgressTimer::create(sprite);
        
        timer.set_type(ProgressTimerType::Bar);
        
        let rate = timer.get_bar_change_rate();
        assert_eq!(rate.x, 1.0);
        assert_eq!(rate.y, 0.0);
        
        timer.set_bar_change_rate(BarChangeRate::new(0.0, 1.0));
        let rate = timer.get_bar_change_rate();
        assert_eq!(rate.x, 0.0);
        assert_eq!(rate.y, 1.0);
    }
    
    #[test]
    fn test_bar_midpoint() {
        let sprite = Sprite::default();
        let mut timer = ProgressTimer::create(sprite);
        
        timer.set_type(ProgressTimerType::Bar);
        timer.set_midpoint_bar(Vec2::new(0.0, 0.5));
        
        let midpoint = timer.get_midpoint_bar();
        assert_eq!(midpoint.x, 0.0);
        assert_eq!(midpoint.y, 0.5);
    }
    
    #[test]
    fn test_update_progress() {
        let sprite = Sprite::default();
        let mut timer = ProgressTimer::create(sprite);
        
        timer.set_percentage(75.0);
        timer.update_progress();
        
        assert_eq!(timer.get_percentage(), 75.0);
    }
    
    #[test]
    fn test_set_sprite() {
        let sprite1 = Sprite::default();
        let mut timer = ProgressTimer::create(sprite1);
        
        assert!(timer.get_sprite().is_some());
        
        let sprite2 = Sprite::default();
        timer.set_sprite(sprite2);
        
        assert!(timer.get_sprite().is_some());
    }
}
