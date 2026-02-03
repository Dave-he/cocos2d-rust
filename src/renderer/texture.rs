use crate::base::{Ref, RefPtr};
use std::collections::HashMap;

pub trait Texture {
    fn get_name(&self) -> u32;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn has_mipmaps(&self) -> bool;
    fn get_bits_per_pixel(&self) -> u32;
    fn get_pixel_format(&self) -> PixelFormat;
    fn get_texture_type(&self) -> TextureType;
    fn update_tex_image(&mut self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureType {
    Texture2D,
    TextureCube,
    Texture3D,
    Texture2DArray,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    NONE,
    RGB888,
    RGBA8888,
    RGB565,
    RGBA4444,
    RGB5A1,
    AI88,
    I8,
    AI48,
    RGBA16F,
    RGB16F,
    R16F,
    RGBA32F,
    RGB32F,
    SRGB8,
    SRGB8_A8,
    DEPTH,
    DEPTH_STENCIL,
}

impl PixelFormat {
    pub fn get_bytes_per_pixel(&self) -> u32 {
        match self {
            PixelFormat::RGB888 => 3,
            PixelFormat::RGBA8888 => 4,
            PixelFormat::RGB565 => 2,
            PixelFormat::RGBA4444 => 2,
            PixelFormat::RGB5A1 => 2,
            PixelFormat::AI88 => 2,
            PixelFormat::I8 => 1,
            PixelFormat::AI48 => 2,
            PixelFormat::RGBA16F => 8,
            PixelFormat::RGB16F => 6,
            PixelFormat::R16F => 2,
            PixelFormat::RGBA32F => 16,
            PixelFormat::RGB32F => 12,
            PixelFormat::SRGB8 => 3,
            PixelFormat::SRGB8_A8 => 4,
            _ => 0,
        }
    }

    pub fn is_compressed(&self) -> bool {
        matches!(self, PixelFormat::NONE)
    }

    pub fn is_float(&self) -> bool {
        matches!(
            self,
            PixelFormat::RGBA16F
                | PixelFormat::RGB16F
                | PixelFormat::R16F
                | PixelFormat::RGBA32F
                | PixelFormat::RGB32F
        )
    }

    pub fn has_alpha(&self) -> bool {
        matches!(
            self,
            PixelFormat::RGBA8888
                | PixelFormat::RGBA4444
                | PixelFormat::RGB5A1
                | PixelFormat::AI88
                | PixelFormat::SRGB8_A8
                | PixelFormat::RGBA16F
                | PixelFormat::RGBA32F
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureDescriptor {
    pub width: u32,
    pub height: u32,
    pub pixel_format: PixelFormat,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
    pub wrap_s: TextureWrap,
    pub wrap_t: TextureWrap,
    pub mipmap_level: u32,
}

impl TextureDescriptor {
    pub fn new() -> TextureDescriptor {
        TextureDescriptor {
            width: 0,
            height: 0,
            pixel_format: PixelFormat::RGBA8888,
            min_filter: TextureFilter::LINEAR,
            mag_filter: TextureFilter::LINEAR,
            wrap_s: TextureWrap::CLAMP_TO_EDGE,
            wrap_t: TextureWrap::CLAMP_TO_EDGE,
            mipmap_level: 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFilter {
    NEAREST,
    LINEAR,
    NEAREST_MIPMAP_NEAREST,
    LINEAR_MIPMAP_NEAREST,
    NEAREST_MIPMAP_LINEAR,
    LINEAR_MIPMAP_LINEAR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureWrap {
    REPEAT,
    CLAMP_TO_EDGE,
    MIRRORED_REPEAT,
}

#[derive(Debug)]
pub struct Texture2D {
    name: u32,
    width: u32,
    height: u32,
    pixel_format: PixelFormat,
    texture_type: TextureType,
    bits_per_pixel: u32,
    has_mipmaps: bool,
}

impl Texture2D {
    pub fn new() -> Texture2D {
        Texture2D {
            name: 0,
            width: 0,
            height: 0,
            pixel_format: PixelFormat::RGBA8888,
            texture_type: TextureType::Texture2D,
            bits_per_pixel: 32,
            has_mipmaps: false,
        }
    }

    pub fn get_name(&self) -> u32 {
        self.name
    }

    pub fn set_name(&mut self, name: u32) {
        self.name = name;
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_pixel_format(&self) -> PixelFormat {
        self.pixel_format
    }

    pub fn get_bits_per_pixel(&self) -> u32 {
        self.bits_per_pixel
    }

    pub fn has_mipmaps(&self) -> bool {
        self.has_mipmaps
    }

    pub fn update(&mut self, data: &[u8], width: u32, height: u32, pixel_format: PixelFormat) {
        self.width = width;
        self.height = height;
        self.pixel_format = pixel_format;
        self.bits_per_pixel = pixel_format.get_bytes_per_pixel() * 8;
    }
}

impl Texture for Texture2D {
    fn get_name(&self) -> u32 {
        self.name
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn has_mipmaps(&self) -> bool {
        self.has_mipmaps
    }

    fn get_bits_per_pixel(&self) -> u32 {
        self.bits_per_pixel
    }

    fn get_pixel_format(&self) -> PixelFormat {
        self.pixel_format
    }

    fn get_texture_type(&self) -> TextureType {
        self.texture_type
    }

    fn update_tex_image(&mut self) {}
}

#[derive(Debug)]
pub struct TextureAtlas {
    texture: Option<RefPtr<Texture2D>>,
    capacity: u32,
    quads: Vec<TextureQuad>,
    indices: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct TextureQuad {
    tl: TexturedVertex,
    tr: TexturedVertex,
    bl: TexturedVertex,
    br: TexturedVertex,
}

#[derive(Debug, Clone, Copy)]
pub struct TexturedVertex {
    x: f32,
    y: f32,
    z: f32,
    u: f32,
    v: f32,
}

impl TextureAtlas {
    pub fn new() -> TextureAtlas {
        TextureAtlas {
            texture: None,
            capacity: 0,
            quads: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn init(&mut self, texture: RefPtr<Texture2D>, capacity: u32) {
        self.texture = Some(texture);
        self.capacity = capacity;
        self.quads.resize(capacity as usize, TextureQuad::new());
    }

    pub fn update_quad(&mut self, quad: TextureQuad, index: u32) {
        if index < self.capacity {
            self.quads[index as usize] = quad;
        }
    }

    pub fn insert_quad(&mut self, quad: TextureQuad, index: u32) {
        if index < self.quads.len() as u32 {
            self.quads.insert(index as usize, quad);
        }
    }

    pub fn remove_quad_at(&mut self, index: u32) {
        if index < self.quads.len() as u32 {
            self.quads.remove(index as usize);
        }
    }

    pub fn get_total_quads(&self) -> u32 {
        self.quads.len() as u32
    }

    pub fn get_capacity(&self) -> u32 {
        self.capacity
    }

    pub fn get_texture(&self) -> Option<&RefPtr<Texture2D>> {
        self.texture.as_ref()
    }
}

impl TextureQuad {
    pub fn new() -> TextureQuad {
        TextureQuad {
            tl: TexturedVertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                u: 0.0,
                v: 0.0,
            },
            tr: TexturedVertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                u: 1.0,
                v: 0.0,
            },
            bl: TexturedVertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                u: 0.0,
                v: 1.0,
            },
            br: TexturedVertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                u: 1.0,
                v: 1.0,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sampler {
    min_filter: TextureFilter,
    mag_filter: TextureFilter,
    wrap_s: TextureWrap,
    wrap_t: TextureWrap,
    compare_mode: bool,
    compare_func: CompareFunc,
}

impl Sampler {
    pub fn new() -> Sampler {
        Sampler {
            min_filter: TextureFilter::LINEAR,
            mag_filter: TextureFilter::LINEAR,
            wrap_s: TextureWrap::CLAMP_TO_EDGE,
            wrap_t: TextureWrap::CLAMP_TO_EDGE,
            compare_mode: false,
            compare_func: CompareFunc::LEQUAL,
        }
    }

    pub fn set_min_filter(&mut self, filter: TextureFilter) {
        self.min_filter = filter;
    }

    pub fn set_mag_filter(&mut self, filter: TextureFilter) {
        self.mag_filter = filter;
    }

    pub fn set_wrap_s(&mut self, wrap: TextureWrap) {
        self.wrap_s = wrap;
    }

    pub fn set_wrap_t(&mut self, wrap: TextureWrap) {
        self.wrap_t = wrap;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareFunc {
    NEVER,
    LESS,
    EQUAL,
    LEQUAL,
    GREATER,
    NOTEQUAL,
    GEQUAL,
    ALWAYS,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_format_bytes_per_pixel() {
        assert_eq!(PixelFormat::I8.get_bytes_per_pixel(), 1);
        assert_eq!(PixelFormat::RGB888.get_bytes_per_pixel(), 3);
        assert_eq!(PixelFormat::RGBA8888.get_bytes_per_pixel(), 4);
        assert_eq!(PixelFormat::RGB565.get_bytes_per_pixel(), 2);
        assert_eq!(PixelFormat::DEPTH.get_bytes_per_pixel(), 0);
    }

    #[test]
    fn test_pixel_format_is_compressed() {
        assert!(PixelFormat::NONE.is_compressed());
        assert!(!PixelFormat::RGBA8888.is_compressed());
    }

    #[test]
    fn test_pixel_format_is_float() {
        assert!(PixelFormat::RGBA16F.is_float());
        assert!(PixelFormat::RGB32F.is_float());
        assert!(!PixelFormat::RGBA8888.is_float());
    }

    #[test]
    fn test_pixel_format_has_alpha() {
        assert!(PixelFormat::RGBA8888.has_alpha());
        assert!(PixelFormat::AI88.has_alpha());
        assert!(!PixelFormat::RGB888.has_alpha());
    }

    #[test]
    fn test_texture_descriptor_new() {
        let desc = TextureDescriptor::new();
        assert_eq!(desc.width, 0);
        assert_eq!(desc.height, 0);
        assert_eq!(desc.pixel_format, PixelFormat::RGBA8888);
        assert_eq!(desc.min_filter, TextureFilter::LINEAR);
        assert_eq!(desc.mag_filter, TextureFilter::LINEAR);
        assert_eq!(desc.mipmap_level, 1);
    }

    #[test]
    fn test_texture_type_variants() {
        assert_eq!(TextureType::Texture2D, TextureType::Texture2D);
        assert_eq!(TextureType::TextureCube, TextureType::TextureCube);
        assert_ne!(TextureType::Texture2D, TextureType::TextureCube);
    }

    #[test]
    fn test_texture_filter_variants() {
        assert_eq!(TextureFilter::NEAREST, TextureFilter::NEAREST);
        assert_eq!(TextureFilter::LINEAR, TextureFilter::LINEAR);
    }

    #[test]
    fn test_texture_wrap_variants() {
        assert_eq!(TextureWrap::REPEAT, TextureWrap::REPEAT);
        assert_eq!(TextureWrap::CLAMP_TO_EDGE, TextureWrap::CLAMP_TO_EDGE);
    }

    #[test]
    fn test_texture2d_new() {
        let texture = Texture2D::new();
        assert_eq!(texture.get_name(), 0);
        assert_eq!(texture.get_width(), 0);
        assert_eq!(texture.get_height(), 0);
        assert_eq!(texture.get_pixel_format(), PixelFormat::RGBA8888);
        assert_eq!(texture.get_bits_per_pixel(), 32);
        assert!(!texture.has_mipmaps());
    }

    #[test]
    fn test_texture2d_set_name() {
        let mut texture = Texture2D::new();
        texture.set_name(123);
        assert_eq!(texture.get_name(), 123);
    }

    #[test]
    fn test_texture2d_update() {
        let mut texture = Texture2D::new();
        let data = &[0u8; 4];
        texture.update(data, 256, 256, PixelFormat::RGBA8888);
        assert_eq!(texture.get_width(), 256);
        assert_eq!(texture.get_height(), 256);
        assert_eq!(texture.get_pixel_format(), PixelFormat::RGBA8888);
    }

    #[test]
    fn test_texture_atlas_new() {
        let atlas = TextureAtlas::new();
        assert!(atlas.get_texture().is_none());
        assert_eq!(atlas.get_total_quads(), 0);
        assert_eq!(atlas.get_capacity(), 0);
    }

    #[test]
    fn test_texture_atlas_init() {
        let mut atlas = TextureAtlas::new();
        let texture = RefPtr::new(Texture2D::new());
        atlas.init(texture.clone(), 10);
        assert!(atlas.get_texture().is_some());
        assert_eq!(atlas.get_capacity(), 10);
        assert_eq!(atlas.get_total_quads(), 10);
    }

    #[test]
    fn test_texture_atlas_update_quad() {
        let mut atlas = TextureAtlas::new();
        let texture = RefPtr::new(Texture2D::new());
        atlas.init(texture, 5);
        let quad = TextureQuad::new();
        atlas.update_quad(quad, 2);
        assert_eq!(atlas.get_total_quads(), 5);
    }

    #[test]
    fn test_texture_atlas_insert_quad() {
        let mut atlas = TextureAtlas::new();
        let texture = RefPtr::new(Texture2D::new());
        atlas.init(texture, 3);
        let quad = TextureQuad::new();
        atlas.insert_quad(quad, 1);
        assert_eq!(atlas.get_total_quads(), 4);
    }

    #[test]
    fn test_texture_atlas_remove_quad_at() {
        let mut atlas = TextureAtlas::new();
        let texture = RefPtr::new(Texture2D::new());
        atlas.init(texture, 3);
        atlas.remove_quad_at(1);
        assert_eq!(atlas.get_total_quads(), 2);
    }

    #[test]
    fn test_texture_quad_new() {
        let quad = TextureQuad::new();
        assert_eq!(quad.tl.u, 0.0);
        assert_eq!(quad.tr.u, 1.0);
        assert_eq!(quad.bl.v, 1.0);
    }

    #[test]
    fn test_sampler_new() {
        let sampler = Sampler::new();
        assert_eq!(sampler.min_filter, TextureFilter::LINEAR);
        assert_eq!(sampler.mag_filter, TextureFilter::LINEAR);
        assert_eq!(sampler.wrap_s, TextureWrap::CLAMP_TO_EDGE);
        assert_eq!(sampler.wrap_t, TextureWrap::CLAMP_TO_EDGE);
        assert!(!sampler.compare_mode);
    }

    #[test]
    fn test_sampler_setters() {
        let mut sampler = Sampler::new();
        sampler.set_min_filter(TextureFilter::NEAREST);
        sampler.set_mag_filter(TextureFilter::NEAREST);
        sampler.set_wrap_s(TextureWrap::REPEAT);
        sampler.set_wrap_t(TextureWrap::REPEAT);

        assert_eq!(sampler.min_filter, TextureFilter::NEAREST);
        assert_eq!(sampler.mag_filter, TextureFilter::NEAREST);
        assert_eq!(sampler.wrap_s, TextureWrap::REPEAT);
        assert_eq!(sampler.wrap_t, TextureWrap::REPEAT);
    }

    #[test]
    fn test_compare_func_variants() {
        let funcs = [
            CompareFunc::NEVER,
            CompareFunc::LESS,
            CompareFunc::EQUAL,
            CompareFunc::LEQUAL,
            CompareFunc::GREATER,
            CompareFunc::NOTEQUAL,
            CompareFunc::GEQUAL,
            CompareFunc::ALWAYS,
        ];
        assert_eq!(funcs.len(), 8);
    }
}
