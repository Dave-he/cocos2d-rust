use cocos2d_rust::renderer::{Renderer, Texture, Material};
use cocos2d_rust::renderer::command::RenderCommand;

#[test]
fn test_renderer_creation() {
    let renderer = Renderer::new();
    assert!(!renderer.is_initialized());
}

#[test]
fn test_texture_creation() {
    let texture = Texture::new();
    
    assert_eq!(texture.get_width(), 0);
    assert_eq!(texture.get_height(), 0);
}

#[test]
fn test_texture_properties() {
    let mut texture = Texture::new();
    
    texture.set_alias_tex_parameters();
    texture.set_anti_alias_tex_parameters();
}

#[test]
fn test_material_creation() {
    let material = Material::new();
    assert!(material.get_name().is_empty());
}

#[test]
fn test_render_command_creation() {
    let cmd = RenderCommand::new();
    assert_eq!(cmd.get_type(), RenderCommandType::Unknown);
}

#[test]
fn test_renderer_clear_color() {
    let mut renderer = Renderer::new();
    
    renderer.set_clear_color(Color4F::new(1.0, 0.0, 0.0, 1.0));
}

#[test]
fn test_renderer_viewport() {
    let mut renderer = Renderer::new();
    
    renderer.set_viewport(0, 0, 800, 600);
}

#[test]
fn test_texture_pixel_format() {
    let mut texture = Texture::new();
    
    texture.set_pixel_format(PixelFormat::RGBA8888);
    assert_eq!(texture.get_pixel_format(), PixelFormat::RGBA8888);
}

#[test]
fn test_texture_content_size() {
    let mut texture = Texture::new();
    
    texture.set_content_size(512.0, 512.0);
    assert_eq!(texture.get_content_width(), 512.0);
    assert_eq!(texture.get_content_height(), 512.0);
}
