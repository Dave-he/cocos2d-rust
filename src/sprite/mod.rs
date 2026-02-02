use crate::base::{Node, Ref, RefPtr};
use crate::base::types::{Color3B, Rect, Size, BlendFunc};
use crate::base::types::Color4F;
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

/// Sprite is a 2D image that can be rendered
#[derive(Debug)]
pub struct Sprite {
    node: RefPtr<Node>,
    data: Rc<RefCell<SpriteData>>,
}

impl Sprite {
    /// Creates a new empty sprite
    pub fn new() -> Sprite {
        let mut node = Node::new();
        
        let data = Rc::new(RefCell::new(SpriteData {
            texture: None,
            color: Color3B::WHITE,
            opacity: 255,
            flipped_x: false,
            flipped_y: false,
            blend_func: BlendFunc::ALPHA_PREMULTIPLIED,
            rect: Rect::ZERO,
        }));

        let data_clone = data.clone();
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
            let mut bl_uv = [0.0, 1.0];
            let mut br_uv = [1.0, 1.0];
            let mut tl_uv = [0.0, 0.0];
            let mut tr_uv = [1.0, 0.0];
            
            // Handle texture rect if needed (simplified for now, full UV calculation would require texture size)
            // If rect is used, we should map it to UVs
            
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
            node: RefPtr::new(node),
            data,
        }
    }

    /// Creates a sprite with a texture
    pub fn with_texture(texture: RefPtr<Texture2D>) -> Sprite {
        let mut sprite = Sprite::new();
        sprite.set_texture(texture);
        sprite
    }

    /// Creates a sprite with a file path
    pub fn with_file(file_path: &str) -> Option<Sprite> {
        let texture = TextureCache::get_instance().add_image(file_path)?;
        Some(Sprite::with_texture(texture))
    }

    /// Creates a sprite with a rect from a texture
    pub fn with_texture_rect(texture: RefPtr<Texture2D>, rect: Rect) -> Sprite {
        let mut sprite = Sprite::with_texture(texture);
        sprite.set_texture_rect(rect);
        sprite
    }

    /// Gets the texture
    pub fn get_texture(&self) -> Option<RefPtr<Texture2D>> {
        self.data.borrow().texture.clone()
    }

    /// Sets the texture
    pub fn set_texture(&mut self, texture: RefPtr<Texture2D>) {
        self.data.borrow_mut().texture = Some(texture);
    }

    /// Gets the color
    pub fn get_color(&self) -> Color3B {
        self.data.borrow().color
    }

    /// Sets the color
    pub fn set_color(&mut self, color: Color3B) {
        self.data.borrow_mut().color = color;
    }

    /// Gets the opacity
    pub fn get_opacity(&self) -> u8 {
        self.data.borrow().opacity
    }

    /// Sets the opacity
    pub fn set_opacity(&mut self, opacity: u8) {
        self.data.borrow_mut().opacity = opacity;
    }

    /// Gets the blend function
    pub fn get_blend_func(&self) -> BlendFunc {
        self.data.borrow().blend_func
    }

    /// Sets the blend function
    pub fn set_blend_func(&mut self, blend_func: BlendFunc) {
        self.data.borrow_mut().blend_func = blend_func;
    }

    /// Gets the texture rect
    pub fn get_texture_rect(&self) -> Rect {
        self.data.borrow().rect
    }

    /// Sets the texture rect
    pub fn set_texture_rect(&mut self, rect: Rect) {
        self.data.borrow_mut().rect = rect;
    }

    /// Checks if the sprite is flipped on X axis
    pub fn is_flipped_x(&self) -> bool {
        self.data.borrow().flipped_x
    }

    /// Sets the flipped X state
    pub fn set_flipped_x(&mut self, flipped_x: bool) {
        self.data.borrow_mut().flipped_x = flipped_x;
    }

    /// Checks if the sprite is flipped on Y axis
    pub fn is_flipped_y(&self) -> bool {
        self.data.borrow().flipped_y
    }

    /// Sets the flipped Y state
    pub fn set_flipped_y(&mut self, flipped_y: bool) {
        self.data.borrow_mut().flipped_y = flipped_y;
    }

    /// Gets the node
    pub fn get_node(&self) -> &RefPtr<Node> {
        &self.node
    }

    /// Gets mutable node
    pub fn get_node_mut(&mut self) -> &mut RefPtr<Node> {
        &mut self.node
    }
}
