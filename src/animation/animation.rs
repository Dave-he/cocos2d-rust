use super::sprite_frame::SpriteFrame;
use std::rc::Rc;
use std::cell::RefCell;

/// 动画
/// 包含一系列精灵帧，定义了帧序列和播放速度
pub struct Animation {
    /// 动画名称
    name: String,
    /// 精灵帧序列
    frames: Vec<Rc<RefCell<SpriteFrame>>>,
    /// 每帧持续时间（秒）
    delay_per_unit: f32,
    /// 总持续时间（秒）
    duration: f32,
    /// 循环次数（0 表示无限循环）
    loops: u32,
    /// 是否恢复原始帧（动画结束后）
    restore_original_frame: bool,
}

impl Animation {
    /// 创建新动画
    pub fn new() -> Self {
        Self {
            name: String::new(),
            frames: Vec::new(),
            delay_per_unit: 0.1,
            duration: 0.0,
            loops: 1,
            restore_original_frame: false,
        }
    }

    /// 从帧序列创建动画
    pub fn with_frames(frames: Vec<Rc<RefCell<SpriteFrame>>>, delay: f32) -> Self {
        let duration = frames.len() as f32 * delay;
        Self {
            name: String::new(),
            frames,
            delay_per_unit: delay,
            duration,
            loops: 1,
            restore_original_frame: false,
        }
    }

    /// 从帧序列创建动画（带名称）
    pub fn with_sprite_frames(name: String, frames: Vec<Rc<RefCell<SpriteFrame>>>, delay: f32) -> Self {
        let duration = frames.len() as f32 * delay;
        Self {
            name,
            frames,
            delay_per_unit: delay,
            duration,
            loops: 1,
            restore_original_frame: false,
        }
    }

    /// 从帧序列和帧间隔数组创建动画
    pub fn with_frame_delays(
        frames: Vec<Rc<RefCell<SpriteFrame>>>,
        delays: Vec<f32>,
    ) -> Result<Self, String> {
        if frames.len() != delays.len() {
            return Err("Frames and delays must have the same length".to_string());
        }

        let duration: f32 = delays.iter().sum();
        
        Ok(Self {
            name: String::new(),
            frames,
            delay_per_unit: if !delays.is_empty() { delays[0] } else { 0.0 },
            duration,
            loops: 1,
            restore_original_frame: false,
        })
    }

    /// 设置名称
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// 获取名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 添加帧
    pub fn add_frame(&mut self, frame: Rc<RefCell<SpriteFrame>>) {
        self.frames.push(frame);
        self.update_duration();
    }

    /// 添加多个帧
    pub fn add_frames(&mut self, frames: Vec<Rc<RefCell<SpriteFrame>>>) {
        self.frames.extend(frames);
        self.update_duration();
    }

    /// 获取帧序列
    pub fn frames(&self) -> &[Rc<RefCell<SpriteFrame>>] {
        &self.frames
    }

    /// 获取帧数量
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// 获取指定索引的帧
    pub fn get_frame(&self, index: usize) -> Option<Rc<RefCell<SpriteFrame>>> {
        self.frames.get(index).cloned()
    }

    /// 设置每帧延迟
    pub fn set_delay_per_unit(&mut self, delay: f32) {
        self.delay_per_unit = delay.max(0.0);
        self.update_duration();
    }

    /// 获取每帧延迟
    pub fn delay_per_unit(&self) -> f32 {
        self.delay_per_unit
    }

    /// 获取总持续时间
    pub fn duration(&self) -> f32 {
        self.duration
    }

    /// 设置循环次数
    pub fn set_loops(&mut self, loops: u32) {
        self.loops = loops;
    }

    /// 获取循环次数
    pub fn loops(&self) -> u32 {
        self.loops
    }

    /// 设置是否恢复原始帧
    pub fn set_restore_original_frame(&mut self, restore: bool) {
        self.restore_original_frame = restore;
    }

    /// 是否恢复原始帧
    pub fn restore_original_frame(&self) -> bool {
        self.restore_original_frame
    }

    /// 更新总持续时间
    fn update_duration(&mut self) {
        self.duration = self.frames.len() as f32 * self.delay_per_unit;
    }

    /// 获取总播放时间（包括循环）
    pub fn total_duration(&self) -> f32 {
        if self.loops == 0 {
            f32::INFINITY
        } else {
            self.duration * self.loops as f32
        }
    }

    /// 根据时间获取帧索引
    pub fn get_frame_index_at_time(&self, time: f32) -> usize {
        if self.frames.is_empty() || self.delay_per_unit <= 0.0 {
            return 0;
        }

        let frame_time = time % self.duration;
        let index = (frame_time / self.delay_per_unit) as usize;
        index.min(self.frames.len() - 1)
    }

    /// 克隆动画
    pub fn clone_animation(&self) -> Self {
        Self {
            name: self.name.clone(),
            frames: self.frames.clone(),
            delay_per_unit: self.delay_per_unit,
            duration: self.duration,
            loops: self.loops,
            restore_original_frame: self.restore_original_frame,
        }
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Animation")
            .field("name", &self.name)
            .field("frame_count", &self.frames.len())
            .field("delay_per_unit", &self.delay_per_unit)
            .field("duration", &self.duration)
            .field("loops", &self.loops)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_frames(count: usize) -> Vec<Rc<RefCell<SpriteFrame>>> {
        (0..count)
            .map(|i| Rc::new(RefCell::new(SpriteFrame::new(format!("frame_{}", i)))))
            .collect()
    }

    #[test]
    fn test_animation_creation() {
        let anim = Animation::new();
        assert_eq!(anim.frame_count(), 0);
        assert_eq!(anim.duration(), 0.0);
    }

    #[test]
    fn test_animation_with_frames() {
        let frames = create_test_frames(5);
        let anim = Animation::with_frames(frames, 0.1);
        
        assert_eq!(anim.frame_count(), 5);
        assert_eq!(anim.delay_per_unit(), 0.1);
        assert_eq!(anim.duration(), 0.5);
    }

    #[test]
    fn test_add_frames() {
        let mut anim = Animation::new();
        anim.set_delay_per_unit(0.1);
        
        let frame = Rc::new(RefCell::new(SpriteFrame::new("frame1")));
        anim.add_frame(frame);
        
        assert_eq!(anim.frame_count(), 1);
        assert_eq!(anim.duration(), 0.1);
    }

    #[test]
    fn test_animation_loops() {
        let frames = create_test_frames(5);
        let mut anim = Animation::with_frames(frames, 0.1);
        
        anim.set_loops(3);
        assert_eq!(anim.loops(), 3);
        assert_eq!(anim.total_duration(), 1.5); // 0.5 * 3
        
        anim.set_loops(0); // 无限循环
        assert_eq!(anim.total_duration(), f32::INFINITY);
    }

    #[test]
    fn test_get_frame_index_at_time() {
        let frames = create_test_frames(5);
        let anim = Animation::with_frames(frames, 0.1);
        
        assert_eq!(anim.get_frame_index_at_time(0.0), 0);
        assert_eq!(anim.get_frame_index_at_time(0.05), 0);
        assert_eq!(anim.get_frame_index_at_time(0.1), 1);
        assert_eq!(anim.get_frame_index_at_time(0.2), 2);
        assert_eq!(anim.get_frame_index_at_time(0.45), 4);
        
        // 循环测试
        assert_eq!(anim.get_frame_index_at_time(0.5), 0); // 回到开始
        assert_eq!(anim.get_frame_index_at_time(0.6), 1);
    }

    #[test]
    fn test_restore_original_frame() {
        let mut anim = Animation::new();
        assert!(!anim.restore_original_frame());
        
        anim.set_restore_original_frame(true);
        assert!(anim.restore_original_frame());
    }

    #[test]
    fn test_animation_name() {
        let mut anim = Animation::new();
        anim.set_name("walk");
        assert_eq!(anim.name(), "walk");
    }

    #[test]
    fn test_get_frame() {
        let frames = create_test_frames(3);
        let anim = Animation::with_frames(frames, 0.1);
        
        let frame = anim.get_frame(1);
        assert!(frame.is_some());
        assert_eq!(frame.unwrap().borrow().name(), "frame_1");
        
        assert!(anim.get_frame(10).is_none());
    }

    #[test]
    fn test_animation_with_frame_delays() {
        let frames = create_test_frames(3);
        let delays = vec![0.1, 0.2, 0.15];
        
        let anim = Animation::with_frame_delays(frames, delays).unwrap();
        assert_eq!(anim.frame_count(), 3);
        assert_eq!(anim.duration(), 0.45);
    }

    #[test]
    fn test_animation_with_frame_delays_error() {
        let frames = create_test_frames(3);
        let delays = vec![0.1, 0.2]; // 长度不匹配
        
        let result = Animation::with_frame_delays(frames, delays);
        assert!(result.is_err());
    }

    #[test]
    fn test_clone_animation() {
        let frames = create_test_frames(5);
        let mut anim = Animation::with_frames(frames, 0.1);
        anim.set_name("test");
        anim.set_loops(3);
        
        let cloned = anim.clone_animation();
        assert_eq!(cloned.name(), "test");
        assert_eq!(cloned.frame_count(), 5);
        assert_eq!(cloned.loops(), 3);
    }
}
