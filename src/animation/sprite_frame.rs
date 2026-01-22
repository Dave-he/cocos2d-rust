use crate::math::Rect;
use crate::renderer::Texture2D;
use std::rc::Rc;
use std::cell::RefCell;

/// 精灵帧
/// 代表精灵图集（Sprite Sheet）中的一帧
#[derive(Clone)]
pub struct SpriteFrame {
    /// 帧名称
    name: String,
    /// 纹理引用
    texture: Option<Rc<RefCell<Texture2D>>>,
    /// 纹理矩形（纹理坐标系）
    rect: Rect,
    /// 是否旋转（某些图集打包工具会旋转图片以节省空间）
    rotated: bool,
    /// 原始大小（裁剪前的大小）
    original_size: (f32, f32),
    /// 偏移量（相对于原始大小的偏移）
    offset: (f32, f32),
}

impl SpriteFrame {
    /// 创建新的精灵帧
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            texture: None,
            rect: Rect::new(0.0, 0.0, 0.0, 0.0),
            rotated: false,
            original_size: (0.0, 0.0),
            offset: (0.0, 0.0),
        }
    }

    /// 从纹理和矩形创建精灵帧
    pub fn from_texture(
        name: impl Into<String>,
        texture: Rc<RefCell<Texture2D>>,
        rect: Rect,
    ) -> Self {
        let original_size = (rect.width(), rect.height());
        Self {
            name: name.into(),
            texture: Some(texture),
            rect,
            rotated: false,
            original_size,
            offset: (0.0, 0.0),
        }
    }

    /// 完整构造
    pub fn with_details(
        name: impl Into<String>,
        texture: Rc<RefCell<Texture2D>>,
        rect: Rect,
        rotated: bool,
        original_size: (f32, f32),
        offset: (f32, f32),
    ) -> Self {
        Self {
            name: name.into(),
            texture: Some(texture),
            rect,
            rotated,
            original_size,
            offset,
        }
    }

    /// 获取帧名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 设置纹理
    pub fn set_texture(&mut self, texture: Rc<RefCell<Texture2D>>) {
        self.texture = Some(texture);
    }

    /// 获取纹理
    pub fn texture(&self) -> Option<Rc<RefCell<Texture2D>>> {
        self.texture.clone()
    }

    /// 设置矩形
    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    /// 获取矩形
    pub fn rect(&self) -> Rect {
        self.rect
    }

    /// 设置是否旋转
    pub fn set_rotated(&mut self, rotated: bool) {
        self.rotated = rotated;
    }

    /// 是否旋转
    pub fn is_rotated(&self) -> bool {
        self.rotated
    }

    /// 设置原始大小
    pub fn set_original_size(&mut self, width: f32, height: f32) {
        self.original_size = (width, height);
    }

    /// 获取原始大小
    pub fn original_size(&self) -> (f32, f32) {
        self.original_size
    }

    /// 设置偏移量
    pub fn set_offset(&mut self, x: f32, y: f32) {
        self.offset = (x, y);
    }

    /// 获取偏移量
    pub fn offset(&self) -> (f32, f32) {
        self.offset
    }

    /// 获取宽度
    pub fn width(&self) -> f32 {
        if self.rotated {
            self.rect.height()
        } else {
            self.rect.width()
        }
    }

    /// 获取高度
    pub fn height(&self) -> f32 {
        if self.rotated {
            self.rect.width()
        } else {
            self.rect.height()
        }
    }

    /// 克隆精灵帧
    pub fn clone_frame(&self) -> Self {
        self.clone()
    }
}

impl std::fmt::Debug for SpriteFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpriteFrame")
            .field("name", &self.name)
            .field("rect", &self.rect)
            .field("rotated", &self.rotated)
            .field("original_size", &self.original_size)
            .field("offset", &self.offset)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_frame_creation() {
        let frame = SpriteFrame::new("test_frame");
        assert_eq!(frame.name(), "test_frame");
        assert!(frame.texture().is_none());
    }

    #[test]
    fn test_sprite_frame_properties() {
        let mut frame = SpriteFrame::new("test");
        
        frame.set_rect(Rect::new(0.0, 0.0, 100.0, 100.0));
        assert_eq!(frame.width(), 100.0);
        assert_eq!(frame.height(), 100.0);
        
        frame.set_rotated(true);
        assert_eq!(frame.width(), 100.0); // 旋转后宽高互换
        assert_eq!(frame.height(), 100.0);
    }

    #[test]
    fn test_sprite_frame_offset() {
        let mut frame = SpriteFrame::new("test");
        
        frame.set_offset(10.0, 20.0);
        assert_eq!(frame.offset(), (10.0, 20.0));
        
        frame.set_original_size(200.0, 300.0);
        assert_eq!(frame.original_size(), (200.0, 300.0));
    }

    #[test]
    fn test_sprite_frame_rotated() {
        let frame = SpriteFrame::new("test");
        assert!(!frame.is_rotated());
        
        let rect = Rect::new(0.0, 0.0, 50.0, 100.0);
        let mut frame = SpriteFrame::from_texture(
            "test",
            Rc::new(RefCell::new(Texture2D::new())),
            rect,
        );
        
        assert_eq!(frame.width(), 50.0);
        assert_eq!(frame.height(), 100.0);
        
        frame.set_rotated(true);
        assert_eq!(frame.width(), 100.0);
        assert_eq!(frame.height(), 50.0);
    }
}
