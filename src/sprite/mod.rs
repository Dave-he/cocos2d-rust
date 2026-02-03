use crate::base::{Node, Ref, RefPtr};
use crate::base::types::{Color3B, Rect, Size, BlendFunc, Color4F};
use crate::math::Vec2;
use crate::renderer::command::{Triangles, TrianglesCommand, Vertex};
use crate::renderer::{Renderer, Texture2D, TextureCache};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct SpriteData {
    texture: Option<RefPtr<Texture2D>>,
    color: Color3B,
    opacity: u8,
    flipped_x: bool,
    flipped_y: bool,
    blend_func: BlendFunc,
    rect: Rect,
}

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
        let mut data = Rc::new(RefCell::new(SpriteData {
            texture: None,
            color: Color3B::WHITE,
            opacity: 255,
            flipped_x: false,
            flipped_y: false,
            blend_func: BlendFunc::ALPHA_PREMULTIPLIED,
            rect: Rect::ZERO,
        }));

        let data_clone = data.clone();
        let mut node = Node::new();
        node.set_on_draw(Box::new(move |renderer, transform| {
            let data = data_clone.borrow();
            
            // Generate Quad/Triangles
            let mut width = 100.0;
            let mut height = 100.0;
            
            if let Some(texture) = &data.texture {
                if data.rect.size.width <= 0.0 {
                     width = texture.borrow().get_width() as f32;
                     height = texture.borrow().get_height() as f32;
                } else {
                     width = data.rect.size.width;
                     height = data.rect.size.height;
                }
            } else if data.rect.size.width > 0.0 {
                 width = data.rect.size.width;
                 height = data.rect.size.height;
            }
            
            let x = -width / 2.0;
            let y = -height / 2.0;
            
            // Texture coordinates
            let bl_uv = [0.0, 1.0];
            let br_uv = [1.0, 1.0];
            let tl_uv = [0.0, 0.0];
            let tr_uv = [1.0, 0.0];
            
            let mut bl = Vertex::with_position(x, y, 0.0);
            bl.tex_coord = bl_uv;
            let mut br = Vertex::with_position(x + width, y, 0.0);
            br.tex_coord = br_uv;
            let mut tl = Vertex::with_position(x, y + height, 0.0);
            tl.tex_coord = tl_uv;
            let mut tr = Vertex::with_position(x + width, y + height, 0.0);
            tr.tex_coord = tr_uv;
            
            let color = Color4F::new(
                data.color.r as f32 / 255.0,
                data.color.g as f32 / 255.0,
                data.color.b as f32 / 255.0,
                data.opacity as f32 / 255.0
            );
            
            bl.color = color;
            br.color = color;
            tl.color = color;
            tr.color = color;
            
            let vertices = vec![bl, br, tl, tr];
            let indices = vec![0, 1, 2, 2, 1, 3];
            
            let mut cmd = Box::new(TrianglesCommand::new());
            let texture = data.texture.clone();
            
            cmd.init(0.0, texture, vertices, indices, (data.blend_func.src, data.blend_func.dst), *transform);
            
            renderer.add_command(cmd);
        }));

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
