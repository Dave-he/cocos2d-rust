use super::transition_scene::{TransitionScene, TransitionOrientation};
use crate::Scene;
use crate::math::Vec2;
use std::rc::Rc;
use std::cell::RefCell;

/// 滑动过渡
pub struct SlideTransition {
    /// 基础过渡
    transition: TransitionScene,
    /// 起始位置偏移
    start_offset: Vec2,
    /// 结束位置偏移
    end_offset: Vec2,
}

impl SlideTransition {
    /// 创建滑动过渡
    pub fn new(
        duration: f32,
        in_scene: Rc<RefCell<Scene>>,
        orientation: TransitionOrientation,
    ) -> Self {
        let mut transition = TransitionScene::new(duration, in_scene);
        transition.set_orientation(orientation);

        // 根据方向设置偏移量
        let (start_offset, end_offset) = Self::calculate_offsets(orientation);

        Self {
            transition,
            start_offset,
            end_offset,
        }
    }

    /// 计算偏移量
    fn calculate_offsets(orientation: TransitionOrientation) -> (Vec2, Vec2) {
        // TODO: 这里需要根据屏幕尺寸计算
        // 暂时使用固定值
        let screen_width = 1024.0;
        let screen_height = 768.0;

        match orientation {
            TransitionOrientation::LeftToRight => {
                (Vec2::new(-screen_width, 0.0), Vec2::ZERO)
            }
            TransitionOrientation::RightToLeft => {
                (Vec2::new(screen_width, 0.0), Vec2::ZERO)
            }
            TransitionOrientation::UpToDown => {
                (Vec2::new(0.0, screen_height), Vec2::ZERO)
            }
            TransitionOrientation::DownToUp => {
                (Vec2::new(0.0, -screen_height), Vec2::ZERO)
            }
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
    }

    /// 更新过渡
    pub fn update(&mut self, dt: f32) {
        self.transition.update(dt);
        
        if !self.transition.is_finished() {
            let progress = self.transition.progress();
            let offset = self.start_offset + (self.end_offset - self.start_offset) * progress;
            self.apply_offset(offset);
        }
    }

    /// 应用偏移
    fn apply_offset(&self, offset: Vec2) {
        // TODO: 将偏移应用到场景位置
        let _ = offset;
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
    fn test_slide_transition_creation() {
        let in_scene = create_test_scene();
        let slide = SlideTransition::new(
            1.0,
            in_scene,
            TransitionOrientation::LeftToRight,
        );
        
        assert_eq!(slide.transition().duration(), 1.0);
        assert_eq!(
            slide.transition().orientation(),
            TransitionOrientation::LeftToRight
        );
    }

    #[test]
    fn test_slide_offsets() {
        let in_scene = create_test_scene();
        
        // 左到右
        let slide = SlideTransition::new(
            1.0,
            in_scene.clone(),
            TransitionOrientation::LeftToRight,
        );
        assert!(slide.start_offset.x < 0.0);
        assert_eq!(slide.end_offset, Vec2::ZERO);
        
        // 右到左
        let slide = SlideTransition::new(
            1.0,
            in_scene.clone(),
            TransitionOrientation::RightToLeft,
        );
        assert!(slide.start_offset.x > 0.0);
        
        // 上到下
        let slide = SlideTransition::new(
            1.0,
            in_scene.clone(),
            TransitionOrientation::UpToDown,
        );
        assert!(slide.start_offset.y > 0.0);
        
        // 下到上
        let slide = SlideTransition::new(
            1.0,
            in_scene,
            TransitionOrientation::DownToUp,
        );
        assert!(slide.start_offset.y < 0.0);
    }

    #[test]
    fn test_slide_transition_update() {
        let in_scene = create_test_scene();
        let mut slide = SlideTransition::new(
            2.0,
            in_scene,
            TransitionOrientation::LeftToRight,
        );
        
        slide.start();
        slide.update(1.0);
        
        assert!(!slide.is_finished());
        assert_eq!(slide.transition().progress(), 0.5);
        
        slide.update(1.0);
        assert!(slide.is_finished());
    }
}
