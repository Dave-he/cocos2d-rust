use super::transition_scene::TransitionScene;
use crate::Scene;
use std::rc::Rc;
use std::cell::RefCell;

/// 淡入淡出过渡
pub struct FadeTransition {
    /// 基础过渡
    transition: TransitionScene,
    /// 起始不透明度
    start_opacity: f32,
    /// 结束不透明度
    end_opacity: f32,
}

impl FadeTransition {
    /// 创建淡入淡出过渡
    pub fn new(duration: f32, in_scene: Rc<RefCell<Scene>>) -> Self {
        Self {
            transition: TransitionScene::new(duration, in_scene),
            start_opacity: 0.0,
            end_opacity: 1.0,
        }
    }

    /// 创建淡入过渡（从黑色淡入）
    pub fn fade_in(duration: f32, in_scene: Rc<RefCell<Scene>>) -> Self {
        Self {
            transition: TransitionScene::new(duration, in_scene),
            start_opacity: 0.0,
            end_opacity: 1.0,
        }
    }

    /// 创建淡出过渡（淡出到黑色）
    pub fn fade_out(duration: f32, in_scene: Rc<RefCell<Scene>>) -> Self {
        Self {
            transition: TransitionScene::new(duration, in_scene),
            start_opacity: 1.0,
            end_opacity: 0.0,
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
            let opacity = self.start_opacity + (self.end_opacity - self.start_opacity) * progress;
            self.apply_opacity(opacity);
        }
    }

    /// 应用不透明度
    fn apply_opacity(&self, opacity: f32) {
        // TODO: 将不透明度应用到场景
        // 实际实现需要设置场景的不透明度或使用着色器
        let _ = opacity;
    }

    /// 是否完成
    pub fn is_finished(&self) -> bool {
        self.transition.is_finished()
    }
}

/// 淡入到白色过渡
pub struct FadeWhiteTransition {
    /// 基础过渡
    transition: TransitionScene,
    /// 白色覆盖不透明度
    white_opacity: f32,
}

impl FadeWhiteTransition {
    /// 创建淡入到白色过渡
    pub fn new(duration: f32, in_scene: Rc<RefCell<Scene>>) -> Self {
        Self {
            transition: TransitionScene::new(duration, in_scene),
            white_opacity: 0.0,
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
            
            // 前半段：淡出到白色
            // 后半段：从白色淡入新场景
            if progress < 0.5 {
                self.white_opacity = progress * 2.0;
            } else {
                self.white_opacity = (1.0 - progress) * 2.0;
            }
            
            self.apply_white_overlay(self.white_opacity);
        }
    }

    /// 应用白色覆盖
    fn apply_white_overlay(&self, opacity: f32) {
        // TODO: 绘制白色覆盖层
        let _ = opacity;
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
    fn test_fade_transition_creation() {
        let in_scene = create_test_scene();
        let fade = FadeTransition::new(1.0, in_scene);
        
        assert_eq!(fade.transition().duration(), 1.0);
    }

    #[test]
    fn test_fade_in() {
        let in_scene = create_test_scene();
        let fade = FadeTransition::fade_in(1.0, in_scene);
        
        assert_eq!(fade.start_opacity, 0.0);
        assert_eq!(fade.end_opacity, 1.0);
    }

    #[test]
    fn test_fade_out() {
        let in_scene = create_test_scene();
        let fade = FadeTransition::fade_out(1.0, in_scene);
        
        assert_eq!(fade.start_opacity, 1.0);
        assert_eq!(fade.end_opacity, 0.0);
    }

    #[test]
    fn test_fade_transition_update() {
        let in_scene = create_test_scene();
        let mut fade = FadeTransition::new(2.0, in_scene);
        
        fade.start();
        fade.update(1.0);
        
        assert!(!fade.is_finished());
        assert_eq!(fade.transition().progress(), 0.5);
    }

    #[test]
    fn test_fade_white_transition() {
        let in_scene = create_test_scene();
        let mut fade = FadeWhiteTransition::new(2.0, in_scene);
        
        fade.start();
        
        // 前半段：白色覆盖增加
        fade.update(0.5);
        assert!(fade.white_opacity > 0.0);
        
        // 中点：白色覆盖最大
        fade.update(0.5);
        
        // 后半段：白色覆盖减少
        fade.update(0.5);
        
        // 结束
        fade.update(0.5);
        assert!(fade.is_finished());
    }
}
