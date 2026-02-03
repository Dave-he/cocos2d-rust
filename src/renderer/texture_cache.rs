use crate::base::Director;
use crate::base::RefPtr;
use crate::platform::Image;
use crate::renderer::texture::Texture2D;
use std::collections::HashMap;

/// TextureCache manages all textures
#[derive(Debug)]
pub struct TextureCache {
    textures: HashMap<String, RefPtr<Texture2D>>,
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
            textures: HashMap::new(),
        }
    }

    /// Adds a texture from a file
    pub fn add_image(&mut self, path: &str) -> Option<RefPtr<Texture2D>> {
        if let Some(texture) = self.textures.get(path) {
            return Some(texture.clone());
        }

        let image = Image::with_file(path)?;

        // Create texture from image via Renderer/Backend
        // This requires access to the Director -> Renderer
        let director = Director::get_instance();
        let mut director = director.borrow_mut();
        let renderer = director.get_renderer();
        let mut renderer = renderer.borrow_mut();

        if let Some(texture) = renderer.create_texture_from_image(&image) {
            let texture_ptr = RefPtr::new(texture);
            self.textures.insert(path.to_string(), texture_ptr.clone());
            Some(texture_ptr)
        } else {
            None
        }
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
