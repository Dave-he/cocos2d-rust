use crate::Scene;
use std::rc::Rc;
use std::cell::RefCell;

/// 过渡方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionOrientation {
    /// 左到右
    LeftToRight,
    /// 右到左
    RightToLeft,
    /// 上到下
    UpToDown,
    /// 下到上
    DownToUp,
}

/// 场景过渡基类
/// 用于在两个场景之间创建平滑的过渡效果
pub struct TransitionScene {
    /// 进入的场景
    in_scene: Option<Rc<RefCell<Scene>>>,
    /// 离开的场景
    out_scene: Option<Rc<RefCell<Scene>>>,
    /// 过渡持续时间（秒）
    duration: f32,
    /// 已过去的时间
    elapsed: f32,
    /// 是否完成
    finished: bool,
    /// 过渡方向
    orientation: TransitionOrientation,
}

impl TransitionScene {
    /// 创建新的场景过渡
    pub fn new(duration: f32, in_scene: Rc<RefCell<Scene>>) -> Self {
        Self {
            in_scene: Some(in_scene),
            out_scene: None,
            duration: duration.max(0.0),
            elapsed: 0.0,
            finished: false,
            orientation: TransitionOrientation::LeftToRight,
        }
    }

    /// 设置离开的场景
    pub fn set_out_scene(&mut self, scene: Rc<RefCell<Scene>>) {
        self.out_scene = Some(scene);
    }

    /// 获取进入的场景
    pub fn in_scene(&self) -> Option<Rc<RefCell<Scene>>> {
        self.in_scene.clone()
    }

    /// 获取离开的场景
    pub fn out_scene(&self) -> Option<Rc<RefCell<Scene>>> {
        self.out_scene.clone()
    }

    /// 获取持续时间
    pub fn duration(&self) -> f32 {
        self.duration
    }

    /// 获取已过去的时间
    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    /// 获取进度（0.0-1.0）
    pub fn progress(&self) -> f32 {
        if self.duration <= 0.0 {
            return 1.0;
        }
        (self.elapsed / self.duration).min(1.0)
    }

    /// 是否完成
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// 设置过渡方向
    pub fn set_orientation(&mut self, orientation: TransitionOrientation) {
        self.orientation = orientation;
    }

    /// 获取过渡方向
    pub fn orientation(&self) -> TransitionOrientation {
        self.orientation
    }

    /// 开始过渡
    pub fn start(&mut self) {
        self.elapsed = 0.0;
        self.finished = false;
        self.on_enter();
    }

    /// 更新过渡
    pub fn update(&mut self, dt: f32) {
        if self.finished {
            return;
        }

        self.elapsed += dt;

        if self.elapsed >= self.duration {
            self.elapsed = self.duration;
            self.finished = true;
            self.on_exit();
        } else {
            self.on_update(self.progress());
        }
    }

    /// 过渡进入时调用（子类可重写）
    pub fn on_enter(&mut self) {
        // 默认实现：显示进入的场景
    }

    /// 过渡更新时调用（子类可重写）
    pub fn on_update(&mut self, _progress: f32) {
        // 子类实现具体的过渡效果
    }

    /// 过渡退出时调用（子类可重写）
    pub fn on_exit(&mut self) {
        // 默认实现：隐藏离开的场景
    }

    /// 停止过渡
    pub fn stop(&mut self) {
        self.finished = true;
        self.on_exit();
    }

    /// 重置过渡
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.finished = false;
    }
}

impl std::fmt::Debug for TransitionScene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransitionScene")
            .field("duration", &self.duration)
            .field("elapsed", &self.elapsed)
            .field("finished", &self.finished)
            .field("orientation", &self.orientation)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_scene() -> Rc<RefCell<Scene>> {
        Rc::new(RefCell::new(Scene::new()))
    }

    #[test]
    fn test_transition_creation() {
        let in_scene = create_test_scene();
        let transition = TransitionScene::new(1.0, in_scene);
        
        assert_eq!(transition.duration(), 1.0);
        assert_eq!(transition.elapsed(), 0.0);
        assert!(!transition.is_finished());
    }

    #[test]
    fn test_transition_progress() {
        let in_scene = create_test_scene();
        let mut transition = TransitionScene::new(2.0, in_scene);
        
        assert_eq!(transition.progress(), 0.0);
        
        transition.update(1.0);
        assert!((transition.progress() - 0.5).abs() < 0.01);
        
        transition.update(1.0);
        assert_eq!(transition.progress(), 1.0);
        assert!(transition.is_finished());
    }

    #[test]
    fn test_transition_orientation() {
        let in_scene = create_test_scene();
        let mut transition = TransitionScene::new(1.0, in_scene);
        
        assert_eq!(transition.orientation(), TransitionOrientation::LeftToRight);
        
        transition.set_orientation(TransitionOrientation::RightToLeft);
        assert_eq!(transition.orientation(), TransitionOrientation::RightToLeft);
    }

    #[test]
    fn test_transition_start_stop() {
        let in_scene = create_test_scene();
        let mut transition = TransitionScene::new(1.0, in_scene);
        
        transition.start();
        assert!(!transition.is_finished());
        
        transition.update(0.5);
        assert!(!transition.is_finished());
        
        transition.stop();
        assert!(transition.is_finished());
    }

    #[test]
    fn test_transition_reset() {
        let in_scene = create_test_scene();
        let mut transition = TransitionScene::new(1.0, in_scene);
        
        transition.update(0.5);
        assert_eq!(transition.elapsed(), 0.5);
        
        transition.reset();
        assert_eq!(transition.elapsed(), 0.0);
        assert!(!transition.is_finished());
    }

    #[test]
    fn test_transition_scenes() {
        let in_scene = create_test_scene();
        let out_scene = create_test_scene();
        let mut transition = TransitionScene::new(1.0, in_scene.clone());
        
        transition.set_out_scene(out_scene.clone());
        
        assert!(transition.in_scene().is_some());
        assert!(transition.out_scene().is_some());
    }

    #[test]
    fn test_zero_duration() {
        let in_scene = create_test_scene();
        let transition = TransitionScene::new(0.0, in_scene);
        
        assert_eq!(transition.progress(), 1.0);
    }

    #[test]
    fn test_negative_duration() {
        let in_scene = create_test_scene();
        let transition = TransitionScene::new(-1.0, in_scene);
        
        // 负数持续时间应该被修正为 0
        assert_eq!(transition.duration(), 0.0);
    }
}
