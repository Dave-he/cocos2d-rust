// 渲染器测试 - 验证渲染系统核心功能

use cocos2d_rust::renderer::{Renderer, Texture2D, Material};

#[test]
fn test_renderer_creation() {
    let renderer = Renderer::new();
    // 验证渲染器可以被创建
    let width = renderer.get_width();
    let height = renderer.get_height();
    // 渲染器应该被成功创建
    assert!(width >= 0);
    assert!(height >= 0);
}

#[test]
fn test_texture2d_creation() {
    let texture = Texture2D::new();
    
    assert_eq!(texture.get_width(), 0);
    assert_eq!(texture.get_height(), 0);
}

#[test]
fn test_texture2d_update() {
    let mut texture = Texture2D::new();
    use cocos2d_rust::renderer::texture::PixelFormat;
    
    // 更新纹理数据（空数据）
    texture.update(&[], 256, 256, PixelFormat::RGBA8888);
    assert_eq!(texture.get_width(), 256);
    assert_eq!(texture.get_height(), 256);
}

#[test]
fn test_texture2d_pixel_format() {
    let texture = Texture2D::new();
    use cocos2d_rust::renderer::texture::PixelFormat;
    
    // 默认格式是 RGBA8888
    assert_eq!(texture.get_pixel_format(), PixelFormat::RGBA8888);
}

#[test]
fn test_material_creation() {
    let material = Material::new();
    assert!(material.get_name().is_empty());
}

#[test]
fn test_renderer_scale() {
    let renderer = Renderer::new();
    // 初始缩放比例
    let scale_x = renderer.get_scaleX();
    let scale_y = renderer.get_scaleY();
    assert!(scale_x >= 0.0);
    assert!(scale_y >= 0.0);
}
