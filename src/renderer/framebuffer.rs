use crate::base::types::Color4F;
use crate::renderer::Texture2D;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachmentType {
    Color0,
    Color1,
    Color2,
    Color3,
    Depth,
    Stencil,
    DepthStencil,
}

pub struct FrameBuffer {
    id: u32,
    width: u32,
    height: u32,
    color_attachments: Vec<Option<Rc<Texture2D>>>,
    depth_attachment: Option<Rc<Texture2D>>,
    stencil_attachment: Option<Rc<Texture2D>>,
    is_complete: bool,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            id: 0,
            width,
            height,
            color_attachments: vec![None, None, None, None],
            depth_attachment: None,
            stencil_attachment: None,
            is_complete: false,
        }
    }

    pub fn with_id(id: u32, width: u32, height: u32) -> Self {
        Self {
            id,
            width,
            height,
            color_attachments: vec![None, None, None, None],
            depth_attachment: None,
            stencil_attachment: None,
            is_complete: false,
        }
    }

    pub fn attach_color(&mut self, texture: Rc<Texture2D>, index: usize) -> bool {
        if index >= 4 {
            return false;
        }
        
        self.color_attachments[index] = Some(texture);
        self.is_complete = false;
        true
    }

    pub fn attach_depth(&mut self, texture: Rc<Texture2D>) {
        self.depth_attachment = Some(texture);
        self.is_complete = false;
    }

    pub fn attach_stencil(&mut self, texture: Rc<Texture2D>) {
        self.stencil_attachment = Some(texture);
        self.is_complete = false;
    }

    pub fn detach_color(&mut self, index: usize) -> Option<Rc<Texture2D>> {
        if index >= 4 {
            return None;
        }
        
        let texture = self.color_attachments[index].take();
        self.is_complete = false;
        texture
    }

    pub fn detach_depth(&mut self) -> Option<Rc<Texture2D>> {
        let texture = self.depth_attachment.take();
        self.is_complete = false;
        texture
    }

    pub fn detach_stencil(&mut self) -> Option<Rc<Texture2D>> {
        let texture = self.stencil_attachment.take();
        self.is_complete = false;
        texture
    }

    pub fn get_color_attachment(&self, index: usize) -> Option<&Rc<Texture2D>> {
        if index >= 4 {
            return None;
        }
        self.color_attachments[index].as_ref()
    }

    pub fn get_depth_attachment(&self) -> Option<&Rc<Texture2D>> {
        self.depth_attachment.as_ref()
    }

    pub fn get_stencil_attachment(&self) -> Option<&Rc<Texture2D>> {
        self.stencil_attachment.as_ref()
    }

    pub fn bind(&mut self) {
        self.is_complete = true;
    }

    pub fn unbind(&self) {
    }

    pub fn clear(&self, _color: Color4F) {
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.is_complete = false;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn has_color_attachment(&self, index: usize) -> bool {
        index < 4 && self.color_attachments[index].is_some()
    }

    pub fn has_depth_attachment(&self) -> bool {
        self.depth_attachment.is_some()
    }

    pub fn has_stencil_attachment(&self) -> bool {
        self.stencil_attachment.is_some()
    }

    pub fn color_attachment_count(&self) -> usize {
        self.color_attachments.iter().filter(|a| a.is_some()).count()
    }
}

pub struct FrameBufferPool {
    buffers: HashMap<String, Rc<FrameBuffer>>,
    temp_buffers: Vec<Rc<FrameBuffer>>,
    next_id: u32,
}

impl FrameBufferPool {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
            temp_buffers: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create_buffer(&mut self, width: u32, height: u32) -> Rc<FrameBuffer> {
        let id = self.next_id;
        self.next_id += 1;
        
        Rc::new(FrameBuffer::with_id(id, width, height))
    }

    pub fn get_or_create(&mut self, name: &str, width: u32, height: u32) -> Rc<FrameBuffer> {
        if let Some(buffer) = self.buffers.get(name) {
            Rc::clone(buffer)
        } else {
            let buffer = self.create_buffer(width, height);
            self.buffers.insert(name.to_string(), Rc::clone(&buffer));
            buffer
        }
    }

    pub fn acquire_temp(&mut self, width: u32, height: u32) -> Rc<FrameBuffer> {
        for buffer in &self.temp_buffers {
            if buffer.get_width() == width && buffer.get_height() == height {
                return Rc::clone(buffer);
            }
        }
        
        let buffer = self.create_buffer(width, height);
        self.temp_buffers.push(Rc::clone(&buffer));
        buffer
    }

    pub fn release_temp(&mut self, _buffer: Rc<FrameBuffer>) {
    }

    pub fn get(&self, name: &str) -> Option<Rc<FrameBuffer>> {
        self.buffers.get(name).map(Rc::clone)
    }

    pub fn remove(&mut self, name: &str) -> Option<Rc<FrameBuffer>> {
        self.buffers.remove(name)
    }

    pub fn clear(&mut self) {
        self.buffers.clear();
        self.temp_buffers.clear();
    }

    pub fn clear_temp(&mut self) {
        self.temp_buffers.clear();
    }

    pub fn buffer_count(&self) -> usize {
        self.buffers.len()
    }

    pub fn temp_buffer_count(&self) -> usize {
        self.temp_buffers.len()
    }
}

impl Default for FrameBufferPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framebuffer_creation() {
        let fb = FrameBuffer::new(800, 600);
        assert_eq!(fb.get_width(), 800);
        assert_eq!(fb.get_height(), 600);
        assert_eq!(fb.get_id(), 0);
        assert!(!fb.is_complete());
    }

    #[test]
    fn test_framebuffer_with_id() {
        let fb = FrameBuffer::with_id(42, 1024, 768);
        assert_eq!(fb.get_id(), 42);
        assert_eq!(fb.get_width(), 1024);
        assert_eq!(fb.get_height(), 768);
    }

    #[test]
    fn test_framebuffer_resize() {
        let mut fb = FrameBuffer::new(800, 600);
        fb.resize(1920, 1080);
        assert_eq!(fb.get_width(), 1920);
        assert_eq!(fb.get_height(), 1080);
        assert!(!fb.is_complete());
    }

    #[test]
    fn test_color_attachment_count() {
        let fb = FrameBuffer::new(800, 600);
        assert_eq!(fb.color_attachment_count(), 0);
    }

    #[test]
    fn test_has_attachments() {
        let fb = FrameBuffer::new(800, 600);
        assert!(!fb.has_color_attachment(0));
        assert!(!fb.has_depth_attachment());
        assert!(!fb.has_stencil_attachment());
    }

    #[test]
    fn test_bind_unbind() {
        let mut fb = FrameBuffer::new(800, 600);
        fb.bind();
        assert!(fb.is_complete());
        fb.unbind();
    }

    #[test]
    fn test_clear() {
        let fb = FrameBuffer::new(800, 600);
        fb.clear(Color4F::BLACK);
    }

    #[test]
    fn test_framebuffer_pool_creation() {
        let pool = FrameBufferPool::new();
        assert_eq!(pool.buffer_count(), 0);
        assert_eq!(pool.temp_buffer_count(), 0);
    }

    #[test]
    fn test_framebuffer_pool_create() {
        let mut pool = FrameBufferPool::new();
        let fb = pool.create_buffer(800, 600);
        assert_eq!(fb.get_width(), 800);
        assert_eq!(fb.get_height(), 600);
        assert_eq!(fb.get_id(), 1);
    }

    #[test]
    fn test_framebuffer_pool_get_or_create() {
        let mut pool = FrameBufferPool::new();
        
        let fb1 = pool.get_or_create("main", 800, 600);
        assert_eq!(pool.buffer_count(), 1);
        
        let fb2 = pool.get_or_create("main", 800, 600);
        assert_eq!(pool.buffer_count(), 1);
        assert_eq!(fb1.get_id(), fb2.get_id());
    }

    #[test]
    fn test_framebuffer_pool_acquire_temp() {
        let mut pool = FrameBufferPool::new();
        
        let fb = pool.acquire_temp(800, 600);
        assert_eq!(pool.temp_buffer_count(), 1);
        assert_eq!(fb.get_width(), 800);
    }

    #[test]
    fn test_framebuffer_pool_get() {
        let mut pool = FrameBufferPool::new();
        pool.get_or_create("test", 800, 600);
        
        let fb = pool.get("test");
        assert!(fb.is_some());
        
        let fb2 = pool.get("nonexistent");
        assert!(fb2.is_none());
    }

    #[test]
    fn test_framebuffer_pool_remove() {
        let mut pool = FrameBufferPool::new();
        pool.get_or_create("test", 800, 600);
        
        assert_eq!(pool.buffer_count(), 1);
        
        let removed = pool.remove("test");
        assert!(removed.is_some());
        assert_eq!(pool.buffer_count(), 0);
    }

    #[test]
    fn test_framebuffer_pool_clear() {
        let mut pool = FrameBufferPool::new();
        pool.get_or_create("test1", 800, 600);
        pool.get_or_create("test2", 1024, 768);
        pool.acquire_temp(640, 480);
        
        pool.clear();
        assert_eq!(pool.buffer_count(), 0);
        assert_eq!(pool.temp_buffer_count(), 0);
    }

    #[test]
    fn test_framebuffer_pool_clear_temp() {
        let mut pool = FrameBufferPool::new();
        pool.acquire_temp(800, 600);
        pool.acquire_temp(1024, 768);
        
        assert_eq!(pool.temp_buffer_count(), 2);
        
        pool.clear_temp();
        assert_eq!(pool.temp_buffer_count(), 0);
    }

    #[test]
    fn test_framebuffer_pool_default() {
        let pool = FrameBufferPool::default();
        assert_eq!(pool.buffer_count(), 0);
    }

    #[test]
    fn test_attachment_type_equality() {
        assert_eq!(AttachmentType::Color0, AttachmentType::Color0);
        assert_ne!(AttachmentType::Color0, AttachmentType::Color1);
        assert_ne!(AttachmentType::Depth, AttachmentType::Stencil);
    }

    #[test]
    fn test_multiple_color_attachments() {
        let fb = FrameBuffer::new(800, 600);
        assert!(!fb.has_color_attachment(0));
        assert!(!fb.has_color_attachment(1));
        assert!(!fb.has_color_attachment(2));
        assert!(!fb.has_color_attachment(3));
    }

    #[test]
    fn test_framebuffer_pool_multiple_buffers() {
        let mut pool = FrameBufferPool::new();
        
        pool.get_or_create("buffer1", 800, 600);
        pool.get_or_create("buffer2", 1024, 768);
        pool.get_or_create("buffer3", 1920, 1080);
        
        assert_eq!(pool.buffer_count(), 3);
    }

    #[test]
    fn test_framebuffer_id_increment() {
        let mut pool = FrameBufferPool::new();
        
        let fb1 = pool.create_buffer(800, 600);
        let fb2 = pool.create_buffer(800, 600);
        let fb3 = pool.create_buffer(800, 600);
        
        assert_eq!(fb1.get_id(), 1);
        assert_eq!(fb2.get_id(), 2);
        assert_eq!(fb3.get_id(), 3);
    }

    #[test]
    fn test_acquire_temp_reuse() {
        let mut pool = FrameBufferPool::new();
        
        let fb1 = pool.acquire_temp(800, 600);
        let fb2 = pool.acquire_temp(800, 600);
        
        assert_eq!(fb1.get_id(), fb2.get_id());
    }

    #[test]
    fn test_attach_color_valid_index() {
        let mut fb = FrameBuffer::new(800, 600);
        let texture = Rc::new(Texture2D::new());
        
        assert!(fb.attach_color(texture, 0));
        assert!(fb.has_color_attachment(0));
        assert_eq!(fb.color_attachment_count(), 1);
    }

    #[test]
    fn test_attach_color_invalid_index() {
        let mut fb = FrameBuffer::new(800, 600);
        let texture = Rc::new(Texture2D::new());
        
        assert!(!fb.attach_color(texture, 4));
        assert!(!fb.has_color_attachment(4));
    }

    #[test]
    fn test_attach_multiple_colors() {
        let mut fb = FrameBuffer::new(800, 600);
        
        fb.attach_color(Rc::new(Texture2D::new()), 0);
        fb.attach_color(Rc::new(Texture2D::new()), 1);
        fb.attach_color(Rc::new(Texture2D::new()), 2);
        
        assert_eq!(fb.color_attachment_count(), 3);
        assert!(fb.has_color_attachment(0));
        assert!(fb.has_color_attachment(1));
        assert!(fb.has_color_attachment(2));
    }

    #[test]
    fn test_attach_depth() {
        let mut fb = FrameBuffer::new(800, 600);
        let texture = Rc::new(Texture2D::new());
        
        fb.attach_depth(texture);
        assert!(fb.has_depth_attachment());
    }

    #[test]
    fn test_attach_stencil() {
        let mut fb = FrameBuffer::new(800, 600);
        let texture = Rc::new(Texture2D::new());
        
        fb.attach_stencil(texture);
        assert!(fb.has_stencil_attachment());
    }

    #[test]
    fn test_detach_color() {
        let mut fb = FrameBuffer::new(800, 600);
        let texture = Rc::new(Texture2D::new());
        
        fb.attach_color(Rc::clone(&texture), 0);
        assert!(fb.has_color_attachment(0));
        
        let detached = fb.detach_color(0);
        assert!(detached.is_some());
        assert!(!fb.has_color_attachment(0));
    }

    #[test]
    fn test_detach_color_invalid_index() {
        let mut fb = FrameBuffer::new(800, 600);
        let detached = fb.detach_color(4);
        assert!(detached.is_none());
    }

    #[test]
    fn test_detach_depth() {
        let mut fb = FrameBuffer::new(800, 600);
        fb.attach_depth(Rc::new(Texture2D::new()));
        
        let detached = fb.detach_depth();
        assert!(detached.is_some());
        assert!(!fb.has_depth_attachment());
    }

    #[test]
    fn test_detach_stencil() {
        let mut fb = FrameBuffer::new(800, 600);
        fb.attach_stencil(Rc::new(Texture2D::new()));
        
        let detached = fb.detach_stencil();
        assert!(detached.is_some());
        assert!(!fb.has_stencil_attachment());
    }

    #[test]
    fn test_get_color_attachment() {
        let mut fb = FrameBuffer::new(800, 600);
        let texture = Rc::new(Texture2D::new());
        
        fb.attach_color(Rc::clone(&texture), 0);
        
        let attachment = fb.get_color_attachment(0);
        assert!(attachment.is_some());
    }

    #[test]
    fn test_get_color_attachment_invalid() {
        let fb = FrameBuffer::new(800, 600);
        let attachment = fb.get_color_attachment(4);
        assert!(attachment.is_none());
    }

    #[test]
    fn test_get_depth_attachment() {
        let mut fb = FrameBuffer::new(800, 600);
        fb.attach_depth(Rc::new(Texture2D::new()));
        
        let attachment = fb.get_depth_attachment();
        assert!(attachment.is_some());
    }

    #[test]
    fn test_get_stencil_attachment() {
        let mut fb = FrameBuffer::new(800, 600);
        fb.attach_stencil(Rc::new(Texture2D::new()));
        
        let attachment = fb.get_stencil_attachment();
        assert!(attachment.is_some());
    }

    #[test]
    fn test_attachment_invalidates_complete() {
        let mut fb = FrameBuffer::new(800, 600);
        fb.bind();
        assert!(fb.is_complete());
        
        fb.attach_color(Rc::new(Texture2D::new()), 0);
        assert!(!fb.is_complete());
    }

    #[test]
    fn test_detachment_invalidates_complete() {
        let mut fb = FrameBuffer::new(800, 600);
        fb.attach_color(Rc::new(Texture2D::new()), 0);
        fb.bind();
        assert!(fb.is_complete());
        
        fb.detach_color(0);
        assert!(!fb.is_complete());
    }

    #[test]
    fn test_resize_invalidates_complete() {
        let mut fb = FrameBuffer::new(800, 600);
        fb.bind();
        assert!(fb.is_complete());
        
        fb.resize(1920, 1080);
        assert!(!fb.is_complete());
    }

    #[test]
    fn test_mrt_support() {
        let mut fb = FrameBuffer::new(800, 600);
        
        fb.attach_color(Rc::new(Texture2D::new()), 0);
        fb.attach_color(Rc::new(Texture2D::new()), 1);
        fb.attach_color(Rc::new(Texture2D::new()), 2);
        fb.attach_color(Rc::new(Texture2D::new()), 3);
        
        assert_eq!(fb.color_attachment_count(), 4);
    }
}

