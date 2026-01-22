use super::transition_scene::TransitionScene;
use crate::Scene;
use std::rc::Rc;
use std::cell::RefCell;

/// 缩放过渡
pub struct ZoomTransition {
    /// 基础过渡
    transition: TransitionScene,
    /// 起始缩放
    start_scale: f32,
    /// 结束缩放
    end_scale: f32,
    /// 当前缩放
    current_scale: f32,
}

impl ZoomTransition {
    /// 创建缩放过渡
    pub fn new(duration: f32, in_scene: Rc<RefCell<Scene>>) -> Self {
        Self {
            transition: TransitionScene::new(duration, in_scene),
            start_scale: 0.0,
            end_scale: 1.0,
            current_scale: 0.0,
        }
    }

    /// 创建放大过渡
    pub fn zoom_in(duration: f32, in_scene: Rc<RefCell<Scene>>) -> Self {
        Self::new(duration, in_scene)
    }

    /// 创建缩小过渡
    pub fn zoom_out(duration: f32, in_scene: Rc<RefCell<Scene>>) -> Self {
        Self {
            transition: TransitionScene::new(duration, in_scene),
            start_scale: 1.0,
            end_scale: 0.0,
            current_scale: 1.0,
        }
    }

    /// 获取基础过渡
    pub fn transition(&self) -> &TransitionScene {
        &self.transition
    }

    /// 获取基础过渡（可变）
    pub fn transition_mut(&mut self) -> &mut TransitionScene {
        &mut self.transition
    }

    /// 开始过渡
    pub fn start(&mut self) {
        self.transition.start();
        self.current_scale = self.start_scale;
    }

    /// 更新过渡
    pub fn update(&mut self, dt: f32) {
        self.transition.update(dt);
        
        if !self.transition.is_finished() {
            let progress = self.transition.progress();
            self.current_scale = self.start_scale + (self.end_scale - self.start_scale) * progress;
            self.apply_scale(self.current_scale);
        }
    }

    /// 应用缩放
    fn apply_scale(&self, scale: f32) {
        // TODO: 将缩放应用到场景
        let _ = scale;
    }

    /// 获取当前缩放
    pub fn current_scale(&self) -> f32 {
        self.current_scale
    }

    /// 是否完成
    pub fn is_finished(&self) -> bool {
        self.transition.is_finished()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_scene() -> Rc<RefCell<Scene>> {
        Rc::new(RefCell::new(Scene::new()))
    }

    #[test]
    fn test_zoom_transition_creation() {
        let in_scene = create_test_scene();
        let zoom = ZoomTransition::new(1.0, in_scene);
        
        assert_eq!(zoom.start_scale, 0.0);
        assert_eq!(zoom.end_scale, 1.0);
    }

    #[test]
    fn test_zoom_in() {
        let in_scene = create_test_scene();
        let zoom = ZoomTransition::zoom_in(1.0, in_scene);
        
        assert_eq!(zoom.start_scale, 0.0);
        assert_eq!(zoom.end_scale, 1.0);
    }

    #[test]
    fn test_zoom_out() {
        let in_scene = create_test_scene();
        let zoom = ZoomTransition::zoom_out(1.0, in_scene);
        
        assert_eq!(zoom.start_scale, 1.0);
        assert_eq!(zoom.end_scale, 0.0);
    }

    #[test]
    fn test_zoom_transition_update() {
        let in_scene = create_test_scene();
        let mut zoom = ZoomTransition::new(2.0, in_scene);
        
        zoom.start();
        assert_eq!(zoom.current_scale(), 0.0);
        
        zoom.update(1.0); // 50% 进度
        assert!((zoom.current_scale() - 0.5).abs() < 0.01);
        
        zoom.update(1.0); // 100% 进度
        assert!((zoom.current_scale() - 1.0).abs() < 0.01);
        assert!(zoom.is_finished());
    }
}
