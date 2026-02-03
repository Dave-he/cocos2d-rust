use crate::base::Node;
use crate::base::types::Color3B;

#[derive(Debug, Clone)]
pub struct Sprite {
    node: Node,
    color: Color3B,
    opacity: u8,
}

impl Sprite {
    pub fn new() -> Self {
        Sprite {
            node: Node::new(),
            color: Color3B::WHITE,
            opacity: 255,
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
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite::new()
    }
}

#[derive(Debug, Clone)]
pub struct Texture2D {
    width: u32,
    height: u32,
}

impl Texture2D {
    pub fn new(width: u32, height: u32) -> Self {
        Texture2D { width, height }
    }
}

impl Default for Texture2D {
    fn default() -> Self {
        Texture2D::new(0, 0)
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

    #[test]
    fn test_texture2d_creation() {
        let texture = Texture2D::new(128, 128);
        assert_eq!(texture.width, 128);
        assert_eq!(texture.height, 128);
    }

    #[test]
    fn test_texture2d_default() {
        let texture = Texture2D::default();
        assert_eq!(texture.width, 0);
        assert_eq!(texture.height, 0);
    }
}
