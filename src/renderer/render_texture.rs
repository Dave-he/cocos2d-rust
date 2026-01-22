use crate::renderer::Texture2D;
use crate::renderer::Texture;
use crate::math::Rect;
use std::rc::Rc;
use std::cell::RefCell;

/// 渲染纹理
/// 允许将渲染结果输出到纹理而不是屏幕
pub struct RenderTexture {
    /// 纹理对象
    texture: Rc<RefCell<Texture2D>>,
    /// 帧缓冲对象 ID
    framebuffer_id: u32,
    /// 深度缓冲对象 ID
    depth_buffer_id: u32,
    /// 模板缓冲对象 ID
    stencil_buffer_id: u32,
    /// 纹理宽度
    width: u32,
    /// 纹理高度
    height: u32,
    /// 是否启用深度缓冲
    depth_enabled: bool,
    /// 是否启用模板缓冲
    stencil_enabled: bool,
    /// 清除颜色
    clear_color: [f32; 4],
    /// 是否自动清除
    auto_clear: bool,
}

impl RenderTexture {
    /// 创建新的渲染纹理
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            texture: Rc::new(RefCell::new(Texture::new())),
            framebuffer_id: 0,
            depth_buffer_id: 0,
            stencil_buffer_id: 0,
            width,
            height,
            depth_enabled: false,
            stencil_enabled: false,
            clear_color: [0.0, 0.0, 0.0, 0.0],
            auto_clear: true,
        }
    }

    /// 创建带深度缓冲的渲染纹理
    pub fn with_depth(width: u32, height: u32) -> Self {
        let mut rt = Self::new(width, height);
        rt.depth_enabled = true;
        rt
    }

    /// 创建带深度和模板缓冲的渲染纹理
    pub fn with_depth_stencil(width: u32, height: u32) -> Self {
        let mut rt = Self::new(width, height);
        rt.depth_enabled = true;
        rt.stencil_enabled = true;
        rt
    }

    /// 初始化渲染纹理
    pub fn init(&mut self) -> Result<(), String> {
        // TODO: 实现 OpenGL 初始化逻辑
        // 1. 创建帧缓冲对象 (glGenFramebuffers)
        // 2. 创建纹理对象并设置参数
        // 3. 绑定纹理到帧缓冲
        // 4. 如果启用深度/模板，创建相应的渲染缓冲对象
        // 5. 检查帧缓冲完整性 (glCheckFramebufferStatus)

        // 模拟成功初始化
        self.framebuffer_id = 1;
        
        if self.depth_enabled {
            self.depth_buffer_id = 1;
        }
        
        if self.stencil_enabled {
            self.stencil_buffer_id = 1;
        }

        Ok(())
    }

    /// 获取纹理
    pub fn texture(&self) -> Rc<RefCell<Texture2D>> {
        self.texture.clone()
    }

    /// 获取帧缓冲 ID
    pub fn framebuffer_id(&self) -> u32 {
        self.framebuffer_id
    }

    /// 获取宽度
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 获取高度
    pub fn height(&self) -> u32 {
        self.height
    }

    /// 是否启用深度缓冲
    pub fn is_depth_enabled(&self) -> bool {
        self.depth_enabled
    }

    /// 是否启用模板缓冲
    pub fn is_stencil_enabled(&self) -> bool {
        self.stencil_enabled
    }

    /// 设置清除颜色
    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.clear_color = [r, g, b, a];
    }

    /// 获取清除颜色
    pub fn clear_color(&self) -> [f32; 4] {
        self.clear_color
    }

    /// 设置是否自动清除
    pub fn set_auto_clear(&mut self, auto_clear: bool) {
        self.auto_clear = auto_clear;
    }

    /// 是否自动清除
    pub fn is_auto_clear(&self) -> bool {
        self.auto_clear
    }

    /// 开始渲染（绑定帧缓冲）
    pub fn begin(&self) {
        // 保存状态并绑定帧缓冲
        // 注意：实际OpenGL实现需要：
        // 1. 保存当前帧缓冲
        // 2. glBindFramebuffer(GL_FRAMEBUFFER, self.framebuffer_id)
        // 3. glViewport(0, 0, self.width as i32, self.height as i32)
        
        if self.auto_clear {
            self.clear();
        }
    }

    /// 结束渲染（恢复默认帧缓冲）
    pub fn end(&self) {
        // 恢复之前的帧缓冲和视口
        // 注意：实际OpenGL实现需要：
        // 1. glBindFramebuffer(GL_FRAMEBUFFER, 0) // 恢复默认帧缓冲
        // 2. 恢复之前的视口设置
    }

    /// 清除缓冲区
    pub fn clear(&self) {
        // 清除颜色/深度/模板缓冲
        // 注意：实际OpenGL实现需要：
        // 1. glClearColor(clear_color[0], clear_color[1], clear_color[2], clear_color[3])
        // 2. 根据depth_enabled和stencil_enabled设置清除标志
        // 3. glClear(flags)
        let _ = self.clear_color; // 避免未使用警告
    }

    /// 保存到文件
    pub fn save_to_file(&self, filename: &str) -> Result<(), String> {
        // 获取像素数据
        let pixels = self.get_pixels()?;
        
        // 注意：实际实现需要使用 image crate 保存为文件
        // 例如：
        // use image::{RgbaImage, ImageBuffer};
        // let img = ImageBuffer::from_raw(self.width, self.height, pixels)
        //     .ok_or("Failed to create image buffer")?;
        // img.save(filename).map_err(|e| e.to_string())?;
        
        let _ = (filename, pixels);
        Err("Save to file requires image crate (not yet added)".to_string())
    }

    /// 获取像素数据
    pub fn get_pixels(&self) -> Result<Vec<u8>, String> {
        // 读取帧缓冲的像素数据
        // 注意：实际OpenGL实现需要：
        // 1. 保存当前绑定的帧缓冲
        // 2. glBindFramebuffer(GL_FRAMEBUFFER, self.framebuffer_id)
        // 3. 创建足够大小的缓冲区
        // 4. glReadPixels(0, 0, width, height, GL_RGBA, GL_UNSIGNED_BYTE, buffer)
        // 5. 恢复之前的帧缓冲
        
        let size = (self.width * self.height * 4) as usize; // RGBA
        Ok(vec![0; size]) // 返回空数据作为占位
    }

    /// 调整大小
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), String> {
        if width == self.width && height == self.height {
            return Ok(());
        }

        self.width = width;
        self.height = height;

        // 重新初始化
        self.destroy();
        self.init()
    }

    /// 清理资源
    pub fn destroy(&mut self) {
        if self.framebuffer_id != 0 {
            // TODO: 调用 glDeleteFramebuffers
            self.framebuffer_id = 0;
        }

        if self.depth_buffer_id != 0 {
            // TODO: 调用 glDeleteRenderbuffers
            self.depth_buffer_id = 0;
        }

        if self.stencil_buffer_id != 0 {
            // TODO: 调用 glDeleteRenderbuffers
            self.stencil_buffer_id = 0;
        }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        self.destroy();
    }
}

/// 渲染纹理辅助函数
impl RenderTexture {
    /// 创建快照（捕获当前屏幕）
    pub fn create_snapshot(width: u32, height: u32) -> Result<Self, String> {
        let mut rt = Self::new(width, height);
        rt.init()?;
        
        // TODO: 捕获当前屏幕内容到纹理
        
        Ok(rt)
    }

    /// 执行渲染到纹理的操作
    pub fn render_to_texture<F>(&self, mut render_fn: F)
    where
        F: FnMut(),
    {
        self.begin();
        render_fn();
        self.end();
    }
}

impl std::fmt::Debug for RenderTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderTexture")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("framebuffer_id", &self.framebuffer_id)
            .field("depth_enabled", &self.depth_enabled)
            .field("stencil_enabled", &self.stencil_enabled)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_texture_creation() {
        let rt = RenderTexture::new(512, 512);
        assert_eq!(rt.width(), 512);
        assert_eq!(rt.height(), 512);
        assert!(!rt.is_depth_enabled());
        assert!(!rt.is_stencil_enabled());
    }

    #[test]
    fn test_render_texture_with_depth() {
        let rt = RenderTexture::with_depth(512, 512);
        assert!(rt.is_depth_enabled());
        assert!(!rt.is_stencil_enabled());
    }

    #[test]
    fn test_render_texture_with_depth_stencil() {
        let rt = RenderTexture::with_depth_stencil(512, 512);
        assert!(rt.is_depth_enabled());
        assert!(rt.is_stencil_enabled());
    }

    #[test]
    fn test_render_texture_init() {
        let mut rt = RenderTexture::new(512, 512);
        let result = rt.init();
        assert!(result.is_ok());
        assert_ne!(rt.framebuffer_id(), 0);
    }

    #[test]
    fn test_clear_color() {
        let mut rt = RenderTexture::new(512, 512);
        
        rt.set_clear_color(1.0, 0.5, 0.2, 1.0);
        assert_eq!(rt.clear_color(), [1.0, 0.5, 0.2, 1.0]);
    }

    #[test]
    fn test_auto_clear() {
        let mut rt = RenderTexture::new(512, 512);
        
        assert!(rt.is_auto_clear());
        
        rt.set_auto_clear(false);
        assert!(!rt.is_auto_clear());
    }

    #[test]
    fn test_resize() {
        let mut rt = RenderTexture::new(512, 512);
        rt.init().unwrap();
        
        let result = rt.resize(1024, 768);
        assert!(result.is_ok());
        assert_eq!(rt.width(), 1024);
        assert_eq!(rt.height(), 768);
    }

    #[test]
    fn test_resize_same_size() {
        let mut rt = RenderTexture::new(512, 512);
        rt.init().unwrap();
        
        let result = rt.resize(512, 512);
        assert!(result.is_ok());
    }

    #[test]
    fn test_render_to_texture() {
        let mut rt = RenderTexture::new(512, 512);
        rt.init().unwrap();
        
        let mut called = false;
        rt.render_to_texture(|| {
            called = true;
        });
        
        assert!(called);
    }

    #[test]
    fn test_texture_reference() {
        let rt = RenderTexture::new(512, 512);
        let texture = rt.texture();
        assert!(Rc::strong_count(&texture) >= 1);
    }
}
