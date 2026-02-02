use crate::base::{Node, Ref, RefPtr};
use crate::base::types::{Color3B, Rect, Size};
use crate::base::types::Color4F;
use crate::math::Vec2;
use crate::renderer::command::{Triangles, TrianglesCommand, Vertex};
use crate::renderer::Renderer;
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
            // For now, let's just make a simple quad based on rect or default size
            let width = if data.rect.size.width > 0.0 { data.rect.size.width } else { 100.0 };
            let height = if data.rect.size.height > 0.0 { data.rect.size.height } else { 100.0 };
            
            let x = -width / 2.0;
            let y = -height / 2.0;
            
            let mut bl = Vertex::with_position(x, y, 0.0);
            bl.tex_coord = [0.0, 1.0];
            let mut br = Vertex::with_position(x + width, y, 0.0);
            br.tex_coord = [1.0, 1.0];
            let mut tl = Vertex::with_position(x, y + height, 0.0);
            tl.tex_coord = [0.0, 0.0];
            let mut tr = Vertex::with_position(x + width, y + height, 0.0);
            tr.tex_coord = [1.0, 0.0];
            
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
            cmd.init(0.0, vertices, indices, (data.blend_func.src, data.blend_func.dst), *transform);
            
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

/// Blend function for rendering
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlendFunc {
    pub src: u32,
    pub dst: u32,
}

impl BlendFunc {
    pub const DISABLE: BlendFunc = BlendFunc { src: 0, dst: 0 };
    pub const ALPHA_NON_PREMULTIPLIED: BlendFunc = BlendFunc { src: 770, dst: 771 };
    pub const ALPHA_PREMULTIPLIED: BlendFunc = BlendFunc { src: 1, dst: 771 };
    pub const ADDITIVE: BlendFunc = BlendFunc { src: 1, dst: 1 };

    pub fn new(src: u32, dst: u32) -> Self {
        BlendFunc { src, dst }
    }
}

/// Texture2D represents an OpenGL texture
#[derive(Debug)]
pub struct Texture2D {
    name: u32,
    width: u32,
    height: u32,
    path: String,
}

impl Texture2D {
    /// Creates a new texture with the given dimensions
    pub fn new(width: u32, height: u32) -> Texture2D {
        Texture2D {
            name: 0,
            width,
            height,
            path: String::new(),
        }
    }

    /// Gets the texture name
    pub fn get_name(&self) -> u32 {
        self.name
    }

    /// Gets the texture width
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Gets the texture height
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Gets the texture path
    pub fn get_path(&self) -> &str {
        &self.path
    }
}

/// TextureCache manages all textures
#[derive(Debug)]
pub struct TextureCache {
    textures: std::collections::HashMap<String, RefPtr<Texture2D>>,
}

impl TextureCache {
    /// Gets the singleton instance
    pub fn get_instance() -> &'static mut TextureCache {
        static mut TEXTURE_CACHE: Option<TextureCache> = None;
        unsafe {
            if TEXTURE_CACHE.is_none() {
                TEXTURE_CACHE = Some(TextureCache::new());
            }
            TEXTURE_CACHE.as_mut().unwrap()
        }
    }

    /// Creates a new texture cache
    pub fn new() -> TextureCache {
        TextureCache {
            textures: std::collections::HashMap::new(),
        }
    }

    /// Adds a texture from a file
    pub fn add_image(&mut self, path: &str) -> Option<RefPtr<Texture2D>> {
        if let Some(texture) = self.textures.get(path) {
            return Some(texture.clone());
        }

        // In a real implementation, this would load the texture from file
        let texture = RefPtr::new(Texture2D::new(0, 0));
        self.textures.insert(path.to_string(), texture.clone());
        Some(texture)
    }

    /// Adds a texture with a key
    pub fn add_texture(&mut self, key: &str, texture: RefPtr<Texture2D>) {
        self.textures.insert(key.to_string(), texture);
    }

    /// Gets a texture by key
    pub fn get_texture(&self, key: &str) -> Option<&RefPtr<Texture2D>> {
        self.textures.get(key)
    }

    /// Removes a texture
    pub fn remove_texture(&mut self, key: &str) {
        self.textures.remove(key);
    }

    /// Removes all textures
    pub fn remove_all_textures(&mut self) {
        self.textures.clear();
    }
}
