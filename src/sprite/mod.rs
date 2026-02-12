use crate::scene::Node;
use crate::base::types::{Color3B, Rect, BlendFunc};
use crate::math::Vec2;
use crate::math::geometry::Size;

#[derive(Debug, Clone)]
pub struct Sprite {
    node: Node,
    color: Color3B,
    opacity: u8,
    flipped_x: bool,
    flipped_y: bool,
    blend_func: BlendFunc,
    rect: Rect,
}

impl Sprite {
    pub fn new() -> Self {
        let node = Node::new();
        
        Sprite {
            node,
            color: Color3B::WHITE,
            opacity: 255,
            flipped_x: false,
            flipped_y: false,
            blend_func: BlendFunc::ALPHA_PREMULTIPLIED,
            rect: Rect::ZERO,
        }
    }

    pub fn with_file(filename: &str) -> Option<Self> {
        let _ = filename;
        Some(Self::new())
    }

    pub fn get_node(&self) -> &Node {
        &self.node
    }

    pub fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    // ===== 委托Node方法 =====
    
    pub fn set_position(&mut self, pos: Vec2) {
        self.node.set_position(pos);
    }

    pub fn set_color(&mut self, color: Color3B) {
        self.color = color;
    }

    pub fn set_texture_rect(&mut self, rect: Rect) {
        self.rect = rect;
        self.node.set_content_size(Size { width: rect.size.width, height: rect.size.height });
    }

    pub fn set_tag(&mut self, tag: i32) {
        self.node.set_tag(tag);
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_creation() {
        let sprite = Sprite::new();
        let _ = sprite.get_node();
    }

    #[test]
    fn test_sprite_default() {
        let sprite = Sprite::default();
        let _ = sprite.get_node();
    }
}
