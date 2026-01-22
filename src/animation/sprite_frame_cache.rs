use super::sprite_frame::SpriteFrame;
use crate::math::Rect;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// 解析字符串形式的矩形（例如 "{{x,y},{w,h}}"）
fn parse_rect_from_string(s: &str) -> Result<Rect, String> {
    // 简化的解析，实际格式可能是 "{{x,y},{w,h}}" 或 "{x,y,w,h}"
    let cleaned = s.trim_matches(|c| c == '{' || c == '}');
    let parts: Vec<&str> = cleaned.split(',').collect();
    
    if parts.len() == 4 {
        let x = parts[0].trim().parse::<f32>().map_err(|_| "Invalid x coordinate".to_string())?;
        let y = parts[1].trim().parse::<f32>().map_err(|_| "Invalid y coordinate".to_string())?;
        let w = parts[2].trim().parse::<f32>().map_err(|_| "Invalid width".to_string())?;
        let h = parts[3].trim().parse::<f32>().map_err(|_| "Invalid height".to_string())?;
        Ok(Rect::new(x, y, w, h))
    } else {
        Err("Invalid rect format".to_string())
    }
}

/// 解析字符串形式的尺寸（例如 "{w,h}"）
fn parse_size_from_string(s: &str) -> Result<(f32, f32), String> {
    let cleaned = s.trim_matches(|c| c == '{' || c == '}');
    let parts: Vec<&str> = cleaned.split(',').collect();
    
    if parts.len() == 2 {
        let w = parts[0].trim().parse::<f32>().map_err(|_| "Invalid width".to_string())?;
        let h = parts[1].trim().parse::<f32>().map_err(|_| "Invalid height".to_string())?;
        Ok((w, h))
    } else {
        Err("Invalid size format".to_string())
    }
}

/// 解析字符串形式的点（例如 "{x,y}"）
fn parse_point_from_string(s: &str) -> Result<(f32, f32), String> {
    parse_size_from_string(s) // 格式相同
}


/// 精灵帧缓存
/// 管理所有加载的精灵帧，避免重复加载
pub struct SpriteFrameCache {
    /// 帧缓存
    frames: HashMap<String, Rc<RefCell<SpriteFrame>>>,
}

impl SpriteFrameCache {
    /// 创建新的精灵帧缓存
    pub fn new() -> Self {
        Self {
            frames: HashMap::new(),
        }
    }

    /// 添加精灵帧
    pub fn add_frame(&mut self, frame: SpriteFrame) {
        let name = frame.name().to_string();
        self.frames.insert(name, Rc::new(RefCell::new(frame)));
    }

    /// 添加精灵帧（使用 Rc）
    pub fn add_frame_rc(&mut self, frame: Rc<RefCell<SpriteFrame>>) {
        let name = frame.borrow().name().to_string();
        self.frames.insert(name, frame);
    }

    /// 通过名称获取精灵帧
    pub fn get_frame(&self, name: &str) -> Option<Rc<RefCell<SpriteFrame>>> {
        self.frames.get(name).cloned()
    }

    /// 移除精灵帧
    pub fn remove_frame(&mut self, name: &str) -> bool {
        self.frames.remove(name).is_some()
    }

    /// 清空所有帧
    pub fn clear(&mut self) {
        self.frames.clear();
    }

    /// 获取帧数量
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// 检查是否存在指定帧
    pub fn has_frame(&self, name: &str) -> bool {
        self.frames.contains_key(name)
    }

    /// 获取所有帧名称
    pub fn frame_names(&self) -> Vec<String> {
        self.frames.keys().cloned().collect()
    }

    /// 从 plist 文件加载帧（简化版本，实际需要解析 XML/plist）
    pub fn load_frames_from_plist(&mut self, plist_file: &str) -> Result<(), String> {
        use std::fs::File;
        use std::io::BufReader;
        
        // 读取 plist 文件
        let file = File::open(plist_file)
            .map_err(|e| format!("Failed to open plist file '{}': {}", plist_file, e))?;
        
        let reader = BufReader::new(file);
        let plist_data: plist::Value = plist::from_reader(reader)
            .map_err(|e| format!("Failed to parse plist file: {}", e))?;
        
        // 解析 frames 字典
        if let plist::Value::Dictionary(root) = plist_data {
            if let Some(plist::Value::Dictionary(frames)) = root.get("frames") {
                for (frame_name, frame_data) in frames.iter() {
                    if let plist::Value::Dictionary(frame_dict) = frame_data {
                        // 解析帧矩形
                        let rect = if let Some(plist::Value::String(rect_str)) = frame_dict.get("frame") {
                            parse_rect_from_string(rect_str)?
                        } else {
                            continue;
                        };
                        
                        // 解析是否旋转
                        let rotated = if let Some(plist::Value::Boolean(r)) = frame_dict.get("rotated") {
                            *r
                        } else {
                            false
                        };
                        
                        // 解析原始尺寸
                        let original_size = if let Some(plist::Value::String(size_str)) = frame_dict.get("sourceSize") {
                            parse_size_from_string(size_str)?
                        } else {
                            (rect.width(), rect.height())
                        };
                        
                        // 解析偏移
                        let offset = if let Some(plist::Value::String(offset_str)) = frame_dict.get("offset") {
                            parse_point_from_string(offset_str)?
                        } else {
                            (0.0, 0.0)
                        };
                        
                        // 创建精灵帧
                        let frame = SpriteFrame::with_details(
                            frame_name.clone(),
                            Rc::new(RefCell::new(crate::renderer::Texture::new())),
                            rect,
                            rotated,
                            original_size,
                            offset,
                        );
                        
                        self.add_frame(frame);
                    }
                }
                Ok(())
            } else {
                Err("No 'frames' key found in plist".to_string())
            }
        } else {
            Err("Invalid plist root structure".to_string())
        }
    }

    /// 从纹理图集加载帧
    pub fn load_frames_from_texture_atlas(
        &mut self,
        _atlas_file: &str,
        _texture_file: &str,
    ) -> Result<(), String> {
        // TODO: 实现图集解析
        Err("Texture atlas parsing not implemented yet".to_string())
    }

    /// 批量添加帧
    pub fn add_frames(&mut self, frames: Vec<SpriteFrame>) {
        for frame in frames {
            self.add_frame(frame);
        }
    }

    /// 移除所有包含指定前缀的帧
    pub fn remove_frames_with_prefix(&mut self, prefix: &str) -> usize {
        let keys_to_remove: Vec<String> = self.frames
            .keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect();
        
        let count = keys_to_remove.len();
        for key in keys_to_remove {
            self.frames.remove(&key);
        }
        count
    }

    /// 获取共享实例（单例模式）
    pub fn shared() -> &'static RefCell<SpriteFrameCache> {
        use std::sync::OnceLock;
        static INSTANCE: OnceLock<RefCell<SpriteFrameCache>> = OnceLock::new();
        INSTANCE.get_or_init(|| RefCell::new(SpriteFrameCache::new()))
    }
}

impl Default for SpriteFrameCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_frame_cache_creation() {
        let cache = SpriteFrameCache::new();
        assert_eq!(cache.frame_count(), 0);
    }

    #[test]
    fn test_add_and_get_frame() {
        let mut cache = SpriteFrameCache::new();
        let frame = SpriteFrame::new("test_frame");
        
        cache.add_frame(frame);
        assert_eq!(cache.frame_count(), 1);
        
        let retrieved = cache.get_frame("test_frame");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().borrow().name(), "test_frame");
    }

    #[test]
    fn test_remove_frame() {
        let mut cache = SpriteFrameCache::new();
        cache.add_frame(SpriteFrame::new("frame1"));
        cache.add_frame(SpriteFrame::new("frame2"));
        
        assert_eq!(cache.frame_count(), 2);
        
        assert!(cache.remove_frame("frame1"));
        assert_eq!(cache.frame_count(), 1);
        
        assert!(!cache.remove_frame("nonexistent"));
        assert_eq!(cache.frame_count(), 1);
    }

    #[test]
    fn test_has_frame() {
        let mut cache = SpriteFrameCache::new();
        cache.add_frame(SpriteFrame::new("test"));
        
        assert!(cache.has_frame("test"));
        assert!(!cache.has_frame("nonexistent"));
    }

    #[test]
    fn test_clear() {
        let mut cache = SpriteFrameCache::new();
        cache.add_frame(SpriteFrame::new("frame1"));
        cache.add_frame(SpriteFrame::new("frame2"));
        
        assert_eq!(cache.frame_count(), 2);
        
        cache.clear();
        assert_eq!(cache.frame_count(), 0);
    }

    #[test]
    fn test_frame_names() {
        let mut cache = SpriteFrameCache::new();
        cache.add_frame(SpriteFrame::new("frame1"));
        cache.add_frame(SpriteFrame::new("frame2"));
        cache.add_frame(SpriteFrame::new("frame3"));
        
        let names = cache.frame_names();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"frame1".to_string()));
        assert!(names.contains(&"frame2".to_string()));
        assert!(names.contains(&"frame3".to_string()));
    }

    #[test]
    fn test_add_frames_batch() {
        let mut cache = SpriteFrameCache::new();
        let frames = vec![
            SpriteFrame::new("frame1"),
            SpriteFrame::new("frame2"),
            SpriteFrame::new("frame3"),
        ];
        
        cache.add_frames(frames);
        assert_eq!(cache.frame_count(), 3);
    }

    #[test]
    fn test_remove_frames_with_prefix() {
        let mut cache = SpriteFrameCache::new();
        cache.add_frame(SpriteFrame::new("player_walk_1"));
        cache.add_frame(SpriteFrame::new("player_walk_2"));
        cache.add_frame(SpriteFrame::new("player_run_1"));
        cache.add_frame(SpriteFrame::new("enemy_idle_1"));
        
        let removed = cache.remove_frames_with_prefix("player_");
        assert_eq!(removed, 3);
        assert_eq!(cache.frame_count(), 1);
        assert!(cache.has_frame("enemy_idle_1"));
    }

    #[test]
    fn test_shared_instance() {
        let cache1 = SpriteFrameCache::shared();
        let cache2 = SpriteFrameCache::shared();
        
        // 验证是同一个实例
        cache1.borrow_mut().add_frame(SpriteFrame::new("shared_frame"));
        assert!(cache2.borrow().has_frame("shared_frame"));
        
        // 清理
        cache1.borrow_mut().clear();
    }
}
