use super::transition_scene::TransitionScene;
use crate::Scene;
use std::rc::Rc;
use std::cell::RefCell;

/// 旋转过渡
pub struct RotateTransition {
    /// 基础过渡
    transition: TransitionScene,
    /// 起始角度（度）
    start_angle: f32,
    /// 结束角度（度）
    end_angle: f32,
    /// 当前角度
    current_angle: f32,
}

impl RotateTransition {
    /// 创建旋转过渡
    pub fn new(duration: f32, in_scene: Rc<RefCell<Scene>>) -> Self {
        Self {
            transition: TransitionScene::new(duration, in_scene),
            start_angle: 0.0,
            end_angle: 360.0,
            current_angle: 0.0,
        }
    }

    /// 创建顺时针旋转过渡
    pub fn clockwise(duration: f32, in_scene: Rc<RefCell<Scene>>) -> Self {
        Self::new(duration, in_scene)
    }

    /// 创建逆时针旋转过渡
    pub fn counter_clockwise(duration: f32, in_scene: Rc<RefCell<Scene>>) -> Self {
        Self {
            transition: TransitionScene::new(duration, in_scene),
            start_angle: 0.0,
            end_angle: -360.0,
            current_angle: 0.0,
        }
    }

    /// 创建自定义角度旋转过渡
    pub fn with_angles(
        duration: f32,
        in_scene: Rc<RefCell<Scene>>,
        start_angle: f32,
        end_angle: f32,
    ) -> Self {
        Self {
            transition: TransitionScene::new(duration, in_scene),
            start_angle,
            end_angle,
            current_angle: start_angle,
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
        self.current_angle = self.start_angle;
    }

    /// 更新过渡
    pub fn update(&mut self, dt: f32) {
        self.transition.update(dt);
        
        if !self.transition.is_finished() {
            let progress = self.transition.progress();
            self.current_angle = self.start_angle + (self.end_angle - self.start_angle) * progress;
            self.apply_rotation(self.current_angle);
        }
    }

    /// 应用旋转
    fn apply_rotation(&self, angle: f32) {
        // TODO: 将旋转应用到场景
        let _ = angle;
    }

    /// 获取当前角度
    pub fn current_angle(&self) -> f32 {
        self.current_angle
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
    fn test_rotate_transition_creation() {
        let in_scene = create_test_scene();
        let rotate = RotateTransition::new(1.0, in_scene);
        
        assert_eq!(rotate.start_angle, 0.0);
        assert_eq!(rotate.end_angle, 360.0);
    }

    #[test]
    fn test_clockwise() {
        let in_scene = create_test_scene();
        let rotate = RotateTransition::clockwise(1.0, in_scene);
        
        assert_eq!(rotate.end_angle, 360.0);
    }

    #[test]
    fn test_counter_clockwise() {
        let in_scene = create_test_scene();
        let rotate = RotateTransition::counter_clockwise(1.0, in_scene);
        
        assert_eq!(rotate.end_angle, -360.0);
    }

    #[test]
    fn test_with_angles() {
        let in_scene = create_test_scene();
        let rotate = RotateTransition::with_angles(1.0, in_scene, 45.0, 180.0);
        
        assert_eq!(rotate.start_angle, 45.0);
        assert_eq!(rotate.end_angle, 180.0);
    }

    #[test]
    fn test_rotate_transition_update() {
        let in_scene = create_test_scene();
        let mut rotate = RotateTransition::new(2.0, in_scene);
        
        rotate.start();
        assert_eq!(rotate.current_angle(), 0.0);
        
        rotate.update(1.0); // 50% 进度
        assert!((rotate.current_angle() - 180.0).abs() < 0.01);
        
        rotate.update(1.0); // 100% 进度
        assert!((rotate.current_angle() - 360.0).abs() < 0.01);
        assert!(rotate.is_finished());
    }
}
