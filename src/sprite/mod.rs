use crate::base::{Node, Ref, RefPtr};
use crate::base::types::{Color3B, Rect, Size};
use crate::math::Vec2;

/// Sprite is a 2D image that can be rendered
#[derive(Debug)]
pub struct Sprite {
    node: Node,
    texture: Option<RefPtr<Texture2D>>,
    color: Color3B,
    opacity: u8,
    flipped_x: bool,
    flipped_y: bool,
    blend_func: BlendFunc,
    rect: Rect,
}

impl Sprite {
    /// Creates a new empty sprite
    pub fn new() -> Sprite {
        Sprite {
            node: Node::new(),
            texture: None,
            color: Color3B::WHITE,
            opacity: 255,
            flipped_x: false,
            flipped_y: false,
            blend_func: BlendFunc::ALPHA_PREMULTIPLIED,
            rect: Rect::ZERO,
        }
    }

    /// Creates a sprite with a texture
    pub fn with_texture(texture: RefPtr<Texture2D>) -> Sprite {
        let mut sprite = Sprite {
            node: Node::new(),
            texture: Some(texture),
            color: Color3B::WHITE,
            opacity: 255,
            flipped_x: false,
            flipped_y: false,
            blend_func: BlendFunc::ALPHA_PREMULTIPLIED,
            rect: Rect::ZERO,
        };
        sprite.rect = Rect::new(0.0, 0.0, 0.0, 0.0);
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
        sprite.rect = rect;
        sprite
    }

    /// Gets the texture
    pub fn get_texture(&self) -> Option<&RefPtr<Texture2D>> {
        self.texture.as_ref()
    }

    /// Sets the texture
    pub fn set_texture(&mut self, texture: RefPtr<Texture2D>) {
        self.texture = Some(texture);
    }

    /// Gets the color
    pub fn get_color(&self) -> Color3B {
        self.color
    }

    /// Sets the color
    pub fn set_color(&mut self, color: Color3B) {
        self.color = color;
    }

    /// Gets the opacity
    pub fn get_opacity(&self) -> u8 {
        self.opacity
    }

    /// Sets the opacity
    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity = opacity;
    }

    /// Gets the blend function
    pub fn get_blend_func(&self) -> BlendFunc {
        self.blend_func
    }

    /// Sets the blend function
    pub fn set_blend_func(&mut self, blend_func: BlendFunc) {
        self.blend_func = blend_func;
    }

    /// Gets the texture rect
    pub fn get_texture_rect(&self) -> Rect {
        self.rect
    }

    /// Sets the texture rect
    pub fn set_texture_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    /// Checks if the sprite is flipped on X axis
    pub fn is_flipped_x(&self) -> bool {
        self.flipped_x
    }

    /// Sets the flipped X state
    pub fn set_flipped_x(&mut self, flipped_x: bool) {
        self.flipped_x = flipped_x;
    }

    /// Checks if the sprite is flipped on Y axis
    pub fn is_flipped_y(&self) -> bool {
        self.flipped_y
    }

    /// Sets the flipped Y state
    pub fn set_flipped_y(&mut self, flipped_y: bool) {
        self.flipped_y = flipped_y;
    }

    /// Gets the node
    pub fn get_node(&self) -> &Node {
        &self.node
    }

    /// Gets mutable node
    pub fn get_node_mut(&mut self) -> &mut Node {
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
        let texture = Ref::new(Texture2D::new(0, 0));
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
