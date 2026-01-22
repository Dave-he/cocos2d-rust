use super::transition_scene::{TransitionScene, TransitionOrientation};
use crate::Scene;
use std::rc::Rc;
use std::cell::RefCell;

/// 翻转过渡
pub struct FlipTransition {
    /// 基础过渡
    transition: TransitionScene,
    /// 翻转角度（度）
    flip_angle: f32,
}

impl FlipTransition {
    /// 创建翻转过渡
    pub fn new(
        duration: f32,
        in_scene: Rc<RefCell<Scene>>,
        orientation: TransitionOrientation,
    ) -> Self {
        let mut transition = TransitionScene::new(duration, in_scene);
        transition.set_orientation(orientation);

        Self {
            transition,
            flip_angle: 0.0,
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
        self.flip_angle = 0.0;
    }

    /// 更新过渡
    pub fn update(&mut self, dt: f32) {
        self.transition.update(dt);
        
        if !self.transition.is_finished() {
            let progress = self.transition.progress();
            
            // 计算翻转角度（0 到 180 度）
            self.flip_angle = progress * 180.0;
            
            self.apply_flip(self.flip_angle);
        }
    }

    /// 应用翻转效果
    fn apply_flip(&self, angle: f32) {
        // TODO: 实现 3D 翻转效果
        // 需要使用 3D 变换矩阵
        let _ = angle;
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
    fn test_flip_transition_creation() {
        let in_scene = create_test_scene();
        let flip = FlipTransition::new(
            1.0,
            in_scene,
            TransitionOrientation::LeftToRight,
        );
        
        assert_eq!(flip.transition().duration(), 1.0);
        assert_eq!(flip.flip_angle, 0.0);
    }

    #[test]
    fn test_flip_transition_update() {
        let in_scene = create_test_scene();
        let mut flip = FlipTransition::new(
            2.0,
            in_scene,
            TransitionOrientation::LeftToRight,
        );
        
        flip.start();
        
        flip.update(1.0); // 50% 进度
        assert!((flip.flip_angle - 90.0).abs() < 0.01);
        
        flip.update(1.0); // 100% 进度
        assert!((flip.flip_angle - 180.0).abs() < 0.01);
        assert!(flip.is_finished());
    }
}
