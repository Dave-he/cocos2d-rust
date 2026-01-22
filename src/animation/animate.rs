use super::animation::Animation;
use super::sprite_frame::SpriteFrame;
use std::rc::Rc;
use std::cell::RefCell;

/// Animate 动作
/// 播放动画序列的动作
pub struct Animate {
    /// 关联的动画
    animation: Rc<RefCell<Animation>>,
    /// 原始精灵帧（用于恢复）
    original_frame: Option<Rc<RefCell<SpriteFrame>>>,
    /// 当前播放时间
    elapsed: f32,
    /// 当前帧索引
    current_frame_index: usize,
    /// 已执行的循环次数
    executed_loops: u32,
    /// 是否完成
    done: bool,
}

impl Animate {
    /// 创建动画动作
    pub fn new(animation: Rc<RefCell<Animation>>) -> Self {
        Self {
            animation,
            original_frame: None,
            elapsed: 0.0,
            current_frame_index: 0,
            executed_loops: 0,
            done: false,
        }
    }

    /// 从动画创建（便捷方法）
    pub fn create(animation: Animation) -> Self {
        Self::new(Rc::new(RefCell::new(animation)))
    }

    /// 获取动画
    pub fn animation(&self) -> Rc<RefCell<Animation>> {
        self.animation.clone()
    }

    /// 开始播放
    pub fn start(&mut self, original_frame: Option<Rc<RefCell<SpriteFrame>>>) {
        self.original_frame = original_frame;
        self.elapsed = 0.0;
        self.current_frame_index = 0;
        self.executed_loops = 0;
        self.done = false;
    }

    /// 停止播放
    pub fn stop(&mut self) {
        self.done = true;
        
        // 恢复原始帧
        if self.animation.borrow().restore_original_frame() {
            // 这里需要在实际使用时将 original_frame 应用到精灵上
        }
    }

    /// 更新动画
    /// 返回当前应该显示的帧
    pub fn update(&mut self, dt: f32) -> Option<Rc<RefCell<SpriteFrame>>> {
        if self.done {
            return None;
        }

        let animation = self.animation.borrow();
        
        // 检查是否有帧
        if animation.frame_count() == 0 {
            self.done = true;
            return None;
        }

        self.elapsed += dt;
        
        // 计算当前循环内的时间
        let duration = animation.duration();
        if duration <= 0.0 {
            self.done = true;
            return None;
        }

        let loops = animation.loops();
        
        // 检查是否完成所有循环
        if loops > 0 && self.elapsed >= duration * loops as f32 {
            self.done = true;
            
            // 恢复原始帧
            if animation.restore_original_frame() {
                return self.original_frame.clone();
            }
            
            // 否则返回最后一帧
            return animation.get_frame(animation.frame_count() - 1);
        }

        // 计算当前帧索引
        let loop_time = self.elapsed % duration;
        let new_frame_index = animation.get_frame_index_at_time(loop_time);
        
        // 更新循环计数
        let new_loop = (self.elapsed / duration) as u32;
        if new_loop > self.executed_loops {
            self.executed_loops = new_loop;
        }

        self.current_frame_index = new_frame_index;
        animation.get_frame(new_frame_index)
    }

    /// 是否完成
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// 获取当前帧索引
    pub fn current_frame_index(&self) -> usize {
        self.current_frame_index
    }

    /// 获取已执行的循环次数
    pub fn executed_loops(&self) -> u32 {
        self.executed_loops
    }

    /// 获取播放进度（0.0-1.0）
    pub fn progress(&self) -> f32 {
        let animation = self.animation.borrow();
        let total_duration = animation.total_duration();
        
        if total_duration.is_infinite() {
            return 0.0;
        }
        
        if total_duration <= 0.0 {
            return 1.0;
        }

        (self.elapsed / total_duration).min(1.0)
    }

    /// 重置动画
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.current_frame_index = 0;
        self.executed_loops = 0;
        self.done = false;
    }

    /// 跳转到指定时间
    pub fn seek(&mut self, time: f32) {
        self.elapsed = time.max(0.0);
        
        let animation = self.animation.borrow();
        let duration = animation.duration();
        
        if duration > 0.0 {
            let loop_time = self.elapsed % duration;
            self.current_frame_index = animation.get_frame_index_at_time(loop_time);
            self.executed_loops = (self.elapsed / duration) as u32;
        }
    }

    /// 克隆动作
    pub fn clone_action(&self) -> Self {
        Self {
            animation: self.animation.clone(),
            original_frame: self.original_frame.clone(),
            elapsed: 0.0,
            current_frame_index: 0,
            executed_loops: 0,
            done: false,
        }
    }
}

impl std::fmt::Debug for Animate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Animate")
            .field("elapsed", &self.elapsed)
            .field("current_frame_index", &self.current_frame_index)
            .field("executed_loops", &self.executed_loops)
            .field("done", &self.done)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_animation(frame_count: usize, delay: f32, loops: u32) -> Animation {
        let frames: Vec<Rc<RefCell<SpriteFrame>>> = (0..frame_count)
            .map(|i| Rc::new(RefCell::new(SpriteFrame::new(format!("frame_{}", i)))))
            .collect();
        
        let mut anim = Animation::with_frames(frames, delay);
        anim.set_loops(loops);
        anim
    }

    #[test]
    fn test_animate_creation() {
        let anim = create_test_animation(5, 0.1, 1);
        let animate = Animate::create(anim);
        
        assert!(!animate.is_done());
        assert_eq!(animate.current_frame_index(), 0);
    }

    #[test]
    fn test_animate_update() {
        let anim = create_test_animation(5, 0.1, 1);
        let mut animate = Animate::create(anim);
        animate.start(None);
        
        // 第一帧
        let frame = animate.update(0.05);
        assert!(frame.is_some());
        assert_eq!(animate.current_frame_index(), 0);
        
        // 第二帧
        let frame = animate.update(0.1);
        assert!(frame.is_some());
        assert_eq!(animate.current_frame_index(), 1);
        
        // 第三帧
        let frame = animate.update(0.1);
        assert!(frame.is_some());
        assert_eq!(animate.current_frame_index(), 2);
    }

    #[test]
    fn test_animate_completion() {
        let anim = create_test_animation(3, 0.1, 1);
        let mut animate = Animate::create(anim);
        animate.start(None);
        
        // 播放完整动画
        animate.update(0.3);
        
        assert!(animate.is_done());
    }

    #[test]
    fn test_animate_loops() {
        let anim = create_test_animation(3, 0.1, 2);
        let mut animate = Animate::create(anim);
        animate.start(None);
        
        // 第一次循环
        animate.update(0.3);
        assert!(!animate.is_done());
        assert_eq!(animate.executed_loops(), 1);
        
        // 第二次循环
        animate.update(0.3);
        assert!(animate.is_done());
        assert_eq!(animate.executed_loops(), 2);
    }

    #[test]
    fn test_animate_progress() {
        let anim = create_test_animation(5, 0.1, 1);
        let mut animate = Animate::create(anim);
        animate.start(None);
        
        assert_eq!(animate.progress(), 0.0);
        
        animate.update(0.25); // 50%
        assert!((animate.progress() - 0.5).abs() < 0.01);
        
        animate.update(0.25); // 100%
        assert!((animate.progress() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_animate_reset() {
        let anim = create_test_animation(5, 0.1, 1);
        let mut animate = Animate::create(anim);
        animate.start(None);
        
        animate.update(0.25);
        assert_eq!(animate.current_frame_index(), 2);
        
        animate.reset();
        assert_eq!(animate.current_frame_index(), 0);
        assert_eq!(animate.executed_loops(), 0);
        assert!(!animate.is_done());
    }

    #[test]
    fn test_animate_seek() {
        let anim = create_test_animation(5, 0.1, 1);
        let mut animate = Animate::create(anim);
        animate.start(None);
        
        animate.seek(0.25);
        assert_eq!(animate.current_frame_index(), 2);
        
        animate.seek(0.0);
        assert_eq!(animate.current_frame_index(), 0);
    }

    #[test]
    fn test_animate_infinite_loop() {
        let anim = create_test_animation(3, 0.1, 0); // 无限循环
        let mut animate = Animate::create(anim);
        animate.start(None);
        
        // 播放多次循环
        for _ in 0..10 {
            animate.update(0.3);
            assert!(!animate.is_done()); // 永远不会完成
        }
        
        assert_eq!(animate.progress(), 0.0); // 无限循环进度为 0
    }

    #[test]
    fn test_animate_stop() {
        let anim = create_test_animation(5, 0.1, 1);
        let mut animate = Animate::create(anim);
        animate.start(None);
        
        animate.update(0.15);
        assert!(!animate.is_done());
        
        animate.stop();
        assert!(animate.is_done());
    }

    #[test]
    fn test_animate_clone() {
        let anim = create_test_animation(5, 0.1, 1);
        let mut animate = Animate::create(anim);
        animate.start(None);
        
        animate.update(0.15);
        
        let cloned = animate.clone_action();
        assert_eq!(cloned.current_frame_index(), 0); // 克隆后重置
        assert!(!cloned.is_done());
    }
}
