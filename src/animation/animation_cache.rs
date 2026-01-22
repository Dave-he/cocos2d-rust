use super::animation::Animation;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// 动画缓存
/// 管理所有加载的动画，避免重复创建
pub struct AnimationCache {
    /// 动画缓存
    animations: HashMap<String, Rc<RefCell<Animation>>>,
}

impl AnimationCache {
    /// 创建新的动画缓存
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }

    /// 添加动画
    pub fn add_animation(&mut self, animation: Animation) {
        let name = animation.name().to_string();
        self.animations.insert(name, Rc::new(RefCell::new(animation)));
    }

    /// 添加动画（使用 Rc）
    pub fn add_animation_rc(&mut self, animation: Rc<RefCell<Animation>>) {
        let name = animation.borrow().name().to_string();
        self.animations.insert(name, animation);
    }

    /// 通过名称获取动画
    pub fn get_animation(&self, name: &str) -> Option<Rc<RefCell<Animation>>> {
        self.animations.get(name).cloned()
    }

    /// 移除动画
    pub fn remove_animation(&mut self, name: &str) -> bool {
        self.animations.remove(name).is_some()
    }

    /// 清空所有动画
    pub fn clear(&mut self) {
        self.animations.clear();
    }

    /// 获取动画数量
    pub fn animation_count(&self) -> usize {
        self.animations.len()
    }

    /// 检查是否存在指定动画
    pub fn has_animation(&self, name: &str) -> bool {
        self.animations.contains_key(name)
    }

    /// 获取所有动画名称
    pub fn animation_names(&self) -> Vec<String> {
        self.animations.keys().cloned().collect()
    }

    /// 从 plist 文件加载动画
    pub fn load_animations_from_plist(&mut self, plist_file: &str) -> Result<(), String> {
        use std::fs::File;
        use std::io::BufReader;
        use super::sprite_frame_cache::SpriteFrameCache;
        
        // 读取 plist 文件
        let file = File::open(plist_file)
            .map_err(|e| format!("Failed to open plist file '{}': {}", plist_file, e))?;
        
        let reader = BufReader::new(file);
        let plist_data: plist::Value = plist::from_reader(reader)
            .map_err(|e| format!("Failed to parse plist file: {}", e))?;
        
        // 解析 animations 字典
        if let plist::Value::Dictionary(root) = plist_data {
            if let Some(plist::Value::Dictionary(animations)) = root.get("animations") {
                let frame_cache = SpriteFrameCache::shared();
                
                for (anim_name, anim_data) in animations.iter() {
                    if let plist::Value::Dictionary(anim_dict) = anim_data {
                        // 解析延迟时间
                        let delay = if let Some(plist::Value::Real(d)) = anim_dict.get("delayPerUnit") {
                            *d as f32
                        } else if let Some(plist::Value::Integer(d)) = anim_dict.get("delayPerUnit") {
                            *d as f32
                        } else {
                            0.1 // 默认延迟
                        };
                        
                        // 解析帧名称数组
                        let mut frames = Vec::new();
                        if let Some(plist::Value::Array(frame_names)) = anim_dict.get("frames") {
                            for frame_name_val in frame_names {
                                if let plist::Value::String(frame_name) = frame_name_val {
                                    if let Some(frame) = frame_cache.borrow().get_frame(frame_name) {
                                        frames.push(frame);
                                    }
                                }
                            }
                        }
                        
                        // 创建动画
                        if !frames.is_empty() {
                            let animation = Animation::with_sprite_frames(
                                anim_name.clone(),
                                frames,
                                delay,
                            );
                            self.add_animation(animation);
                        }
                    }
                }
                Ok(())
            } else {
                Err("No 'animations' key found in plist".to_string())
            }
        } else {
            Err("Invalid plist root structure".to_string())
        }
    }

    /// 批量添加动画
    pub fn add_animations(&mut self, animations: Vec<Animation>) {
        for animation in animations {
            self.add_animation(animation);
        }
    }

    /// 移除所有包含指定前缀的动画
    pub fn remove_animations_with_prefix(&mut self, prefix: &str) -> usize {
        let keys_to_remove: Vec<String> = self.animations
            .keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect();
        
        let count = keys_to_remove.len();
        for key in keys_to_remove {
            self.animations.remove(&key);
        }
        count
    }

    /// 克隆动画（创建新实例）
    pub fn clone_animation(&self, name: &str) -> Option<Animation> {
        self.animations.get(name).map(|anim| {
            anim.borrow().clone_animation()
        })
    }

    /// 获取共享实例（单例模式）
    pub fn shared() -> &'static RefCell<AnimationCache> {
        use std::sync::OnceLock;
        static INSTANCE: OnceLock<RefCell<AnimationCache>> = OnceLock::new();
        INSTANCE.get_or_init(|| RefCell::new(AnimationCache::new()))
    }

    /// 预加载常用动画
    pub fn preload_common_animations(&mut self) {
        // 这里可以添加一些预定义的通用动画
        // 例如：淡入淡出、缩放等
    }
}

impl Default for AnimationCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::SpriteFrame;

    fn create_test_animation(name: &str, frame_count: usize) -> Animation {
        let frames: Vec<Rc<RefCell<SpriteFrame>>> = (0..frame_count)
            .map(|i| Rc::new(RefCell::new(SpriteFrame::new(format!("frame_{}", i)))))
            .collect();
        
        let mut anim = Animation::with_frames(frames, 0.1);
        anim.set_name(name);
        anim
    }

    #[test]
    fn test_animation_cache_creation() {
        let cache = AnimationCache::new();
        assert_eq!(cache.animation_count(), 0);
    }

    #[test]
    fn test_add_and_get_animation() {
        let mut cache = AnimationCache::new();
        let anim = create_test_animation("walk", 5);
        
        cache.add_animation(anim);
        assert_eq!(cache.animation_count(), 1);
        
        let retrieved = cache.get_animation("walk");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().borrow().name(), "walk");
    }

    #[test]
    fn test_remove_animation() {
        let mut cache = AnimationCache::new();
        cache.add_animation(create_test_animation("walk", 5));
        cache.add_animation(create_test_animation("run", 8));
        
        assert_eq!(cache.animation_count(), 2);
        
        assert!(cache.remove_animation("walk"));
        assert_eq!(cache.animation_count(), 1);
        
        assert!(!cache.remove_animation("nonexistent"));
        assert_eq!(cache.animation_count(), 1);
    }

    #[test]
    fn test_has_animation() {
        let mut cache = AnimationCache::new();
        cache.add_animation(create_test_animation("walk", 5));
        
        assert!(cache.has_animation("walk"));
        assert!(!cache.has_animation("run"));
    }

    #[test]
    fn test_clear() {
        let mut cache = AnimationCache::new();
        cache.add_animation(create_test_animation("walk", 5));
        cache.add_animation(create_test_animation("run", 8));
        
        assert_eq!(cache.animation_count(), 2);
        
        cache.clear();
        assert_eq!(cache.animation_count(), 0);
    }

    #[test]
    fn test_animation_names() {
        let mut cache = AnimationCache::new();
        cache.add_animation(create_test_animation("walk", 5));
        cache.add_animation(create_test_animation("run", 8));
        cache.add_animation(create_test_animation("jump", 3));
        
        let names = cache.animation_names();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"walk".to_string()));
        assert!(names.contains(&"run".to_string()));
        assert!(names.contains(&"jump".to_string()));
    }

    #[test]
    fn test_add_animations_batch() {
        let mut cache = AnimationCache::new();
        let animations = vec![
            create_test_animation("walk", 5),
            create_test_animation("run", 8),
            create_test_animation("jump", 3),
        ];
        
        cache.add_animations(animations);
        assert_eq!(cache.animation_count(), 3);
    }

    #[test]
    fn test_remove_animations_with_prefix() {
        let mut cache = AnimationCache::new();
        cache.add_animation(create_test_animation("player_walk", 5));
        cache.add_animation(create_test_animation("player_run", 8));
        cache.add_animation(create_test_animation("player_jump", 3));
        cache.add_animation(create_test_animation("enemy_idle", 4));
        
        let removed = cache.remove_animations_with_prefix("player_");
        assert_eq!(removed, 3);
        assert_eq!(cache.animation_count(), 1);
        assert!(cache.has_animation("enemy_idle"));
    }

    #[test]
    fn test_clone_animation() {
        let mut cache = AnimationCache::new();
        cache.add_animation(create_test_animation("walk", 5));
        
        let cloned = cache.clone_animation("walk");
        assert!(cloned.is_some());
        
        let cloned = cloned.unwrap();
        assert_eq!(cloned.name(), "walk");
        assert_eq!(cloned.frame_count(), 5);
    }

    #[test]
    fn test_shared_instance() {
        let cache1 = AnimationCache::shared();
        let cache2 = AnimationCache::shared();
        
        // 验证是同一个实例
        cache1.borrow_mut().add_animation(create_test_animation("shared_anim", 5));
        assert!(cache2.borrow().has_animation("shared_anim"));
        
        // 清理
        cache1.borrow_mut().clear();
    }
}
