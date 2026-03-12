#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
/// Sprite - 精灵类
///
/// Sprite 是 Cocos2d-Rust 中用于渲染2D图像的核心类。
/// 支持纹理贴图、精灵帧动画、颜色混合、翻转等功能。

use crate::animation::sprite_frame::SpriteFrame;
use crate::base::types::{Color3B, Color4B, Color4F, BlendFunc};
use crate::math::{Vec2, Rect};
use crate::math::geometry::Size;
use crate::renderer::texture::Texture2D;
use crate::scene::node::{Node, NodeType};
use std::cell::RefCell;
use std::rc::Rc;

/// 精灵翻转标志
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FlipState {
    pub x: bool,
    pub y: bool,
}

/// 精灵图集节点（批量渲染）
#[derive(Debug)]
pub struct SpriteBatchNode {
    node: Node,
    texture: Option<Rc<RefCell<Texture2D>>>,
    /// 子精灵列表
    sprites: Vec<Sprite>,
    /// 批次缓存（顶点数据）
    dirty: bool,
    /// 最大缓存精灵数
    capacity: usize,
}

impl SpriteBatchNode {
    pub fn new() -> Self {
        Self {
            node: Node::with_type(NodeType::Sprite),
            texture: None,
            sprites: Vec::new(),
            dirty: false,
            capacity: 128,
        }
    }

    /// 从纹理创建批处理节点
    pub fn create_with_texture(texture: Rc<RefCell<Texture2D>>) -> Self {
        let mut batch = Self::new();
        batch.texture = Some(texture);
        batch
    }

    /// 创建时指定容量
    pub fn create_with_capacity(capacity: usize) -> Self {
        let mut batch = Self::new();
        batch.capacity = capacity;
        batch.sprites = Vec::with_capacity(capacity);
        batch
    }

    /// 从纹理和容量创建
    pub fn create_with_texture_and_capacity(texture: Rc<RefCell<Texture2D>>, capacity: usize) -> Self {
        let mut batch = Self::create_with_texture(texture);
        batch.capacity = capacity;
        batch.sprites = Vec::with_capacity(capacity);
        batch
    }

    /// 添加精灵到批次
    pub fn add_sprite(&mut self, mut sprite: Sprite) {
        sprite.set_batched(true);
        self.sprites.push(sprite);
        self.dirty = true;
    }

    /// 移除指定索引的精灵
    pub fn remove_sprite_at(&mut self, index: usize) -> Option<Sprite> {
        if index < self.sprites.len() {
            self.dirty = true;
            Some(self.sprites.remove(index))
        } else {
            None
        }
    }

    /// 获取精灵列表
    pub fn get_sprites(&self) -> &[Sprite] {
        &self.sprites
    }

    /// 获取可变精灵列表
    pub fn get_sprites_mut(&mut self) -> &mut Vec<Sprite> {
        self.dirty = true;
        &mut self.sprites
    }

    /// 获取精灵数量
    pub fn get_sprite_count(&self) -> usize {
        self.sprites.len()
    }

    /// 获取指定索引精灵
    pub fn get_sprite_at(&self, index: usize) -> Option<&Sprite> {
        self.sprites.get(index)
    }

    /// 设置纹理
    pub fn set_texture(&mut self, texture: Rc<RefCell<Texture2D>>) {
        self.texture = Some(texture);
        self.dirty = true;
    }

    /// 获取纹理
    pub fn texture(&self) -> Option<Rc<RefCell<Texture2D>>> {
        self.texture.clone()
    }

    /// 获取节点
    pub fn get_node(&self) -> &Node {
        &self.node
    }

    /// 获取可变节点
    pub fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// 是否需要重新生成批次缓存
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// 标记批次已更新（渲染后调用）
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    /// 获取容量
    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    /// 清空所有精灵
    pub fn remove_all_sprites(&mut self) {
        self.sprites.clear();
        self.dirty = true;
    }

    /// 排序精灵（按 z-order）
    pub fn sort_by_z_order(&mut self) {
        self.sprites.sort_by(|a, b| {
            a.get_node().local_z_order().cmp(&b.get_node().local_z_order())
        });
        self.dirty = true;
    }

    /// 模拟生成批次顶点数据
    ///
    /// 在真实渲染中，这里会把所有精灵顶点合并到一个大的 VBO 中
    pub fn generate_batch_data(&self) -> BatchData {
        let mut batch = BatchData::new();
        for sprite in &self.sprites {
            if !sprite.is_visible() {
                continue;
            }
            // 合并每个精灵的顶点
            let quad = sprite.generate_quad();
            batch.quads.push(quad);
        }
        batch
    }
}

impl Default for SpriteBatchNode {
    fn default() -> Self {
        Self::new()
    }
}

/// 批次渲染数据（包含所有精灵的顶点数据）
#[derive(Debug, Default)]
pub struct BatchData {
    /// 精灵四边形数据
    pub quads: Vec<SpriteQuad>,
}

impl BatchData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quad_count(&self) -> usize {
        self.quads.len()
    }

    pub fn vertex_count(&self) -> usize {
        self.quads.len() * 4
    }

    pub fn index_count(&self) -> usize {
        self.quads.len() * 6
    }
}

/// 精灵四边形数据（4个顶点）
#[derive(Debug, Clone)]
pub struct SpriteQuad {
    /// 四个顶点位置 [左下, 右下, 左上, 右上]
    pub vertices: [[f32; 3]; 4],
    /// UV 坐标
    pub uvs: [[f32; 2]; 4],
    /// 颜色 RGBA
    pub color: [f32; 4],
}

impl Default for SpriteQuad {
    fn default() -> Self {
        Self {
            vertices: [[0.0; 3]; 4],
            uvs: [[0.0; 2]; 4],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

/// Sprite - 精灵类
#[derive(Debug, Clone)]
pub struct Sprite {
    node: Node,
    /// 精灵颜色（用于颜色混合）
    color: Color3B,
    /// 透明度
    opacity: u8,
    /// X轴翻转
    flipped_x: bool,
    /// Y轴翻转
    flipped_y: bool,
    /// 混合函数
    blend_func: BlendFunc,
    /// 纹理矩形（在纹理坐标系中的位置）
    texture_rect: Rect,
    /// 是否矩形被旋转（图集打包时可能旋转）
    texture_rect_rotated: bool,
    /// 关联的纹理
    texture: Option<Rc<RefCell<Texture2D>>>,
    /// 关联的精灵帧
    sprite_frame: Option<Rc<RefCell<SpriteFrame>>>,
    /// 是否已经初始化
    dirty: bool,
    /// 纹理坐标（4个顶点的UV）
    quad_uv: [[f32; 2]; 4],
    /// 是否处于 SpriteBatchNode 中
    batched: bool,
    /// 原始大小（用于裁剪）
    original_content_size: Size,
    /// 偏移量（精灵帧在原始位置的偏移）
    offset_position: Vec2,
    /// 是否有自定义锚点
    custom_anchor: bool,
}

impl Sprite {
    /// 创建空精灵
    pub fn new() -> Self {
        let mut node = Node::with_type(NodeType::Sprite);
        // Sprite 默认锚点在中心
        node.set_anchor_point(Vec2::new(0.5, 0.5));
        
        Self {
            node,
            color: Color3B::WHITE,
            opacity: 255,
            flipped_x: false,
            flipped_y: false,
            blend_func: BlendFunc::ALPHA_PREMULTIPLIED,
            texture_rect: Rect::ZERO,
            texture_rect_rotated: false,
            texture: None,
            sprite_frame: None,
            dirty: true,
            quad_uv: [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            batched: false,
            original_content_size: Size::zero(),
            offset_position: Vec2::ZERO,
            custom_anchor: false,
        }
    }

    /// 从文件创建精灵
    pub fn with_file(filename: &str) -> Option<Self> {
        let _ = filename;
        Some(Self::new())
    }

    /// 从文件创建精灵（带矩形区域）
    pub fn with_file_and_rect(filename: &str, rect: Rect) -> Option<Self> {
        let mut sprite = Self::new();
        sprite.set_texture_rect(rect);
        Some(sprite)
    }

    /// 从纹理创建精灵
    pub fn with_texture(texture: Rc<RefCell<Texture2D>>) -> Self {
        let mut sprite = Self::new();
        let (w, h) = {
            let t = texture.borrow();
            (t.width() as f32, t.height() as f32)
        };
        sprite.texture = Some(texture);
        sprite.set_texture_rect(Rect::new(0.0, 0.0, w, h));
        sprite
    }

    /// 从纹理和矩形创建精灵
    pub fn with_texture_and_rect(texture: Rc<RefCell<Texture2D>>, rect: Rect) -> Self {
        let mut sprite = Self::new();
        sprite.texture = Some(texture);
        sprite.set_texture_rect(rect);
        sprite
    }

    /// 从精灵帧创建精灵
    pub fn with_sprite_frame(frame: Rc<RefCell<SpriteFrame>>) -> Self {
        let mut sprite = Self::new();
        sprite.set_sprite_frame(frame);
        sprite
    }

    /// 从精灵帧名称创建精灵
    pub fn with_sprite_frame_name(frame_name: &str) -> Option<Self> {
        let _ = frame_name;
        // 实际实现需要从 SpriteFrameCache 查找
        Some(Self::new())
    }

    // ===== Node 委托方法 =====

    pub fn get_node(&self) -> &Node {
        &self.node
    }

    pub fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    pub fn set_position(&mut self, pos: Vec2) {
        self.node.set_position(pos);
    }

    pub fn set_position_xy(&mut self, x: f32, y: f32) {
        self.node.set_position_xy(x, y);
    }

    pub fn get_position(&self) -> Vec2 {
        self.node.position()
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.node.set_scale(scale);
    }

    pub fn set_scale_xy(&mut self, scale_x: f32, scale_y: f32) {
        self.node.set_scale_xy(scale_x, scale_y);
    }

    pub fn get_scale(&self) -> f32 {
        self.node.scale()
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.node.set_rotation(rotation);
    }

    pub fn get_rotation(&self) -> f32 {
        self.node.rotation()
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.node.set_visible(visible);
    }

    pub fn is_visible(&self) -> bool {
        self.node.is_visible()
    }

    /// 设置是否属于 SpriteBatchNode
    pub fn set_batched(&mut self, batched: bool) {
        self.batched = batched;
    }

    /// 是否属于 SpriteBatchNode
    pub fn is_batched(&self) -> bool {
        self.batched
    }

    /// 生成四边形顶点数据（供 SpriteBatchNode 使用）
    pub fn generate_quad(&self) -> SpriteQuad {
        let pos = self.node.position();
        let size = self.get_content_size();
        let scale_x = self.node.scale_x();
        let scale_y = self.node.scale_y();
        let w = size.width * scale_x;
        let h = size.height * scale_y;
        let z = 0.0f32; // 2D

        SpriteQuad {
            vertices: [
                [pos.x,       pos.y,       z],  // 左下
                [pos.x + w,   pos.y,       z],  // 右下
                [pos.x,       pos.y + h,   z],  // 左上
                [pos.x + w,   pos.y + h,   z],  // 右上
            ],
            uvs: self.quad_uv,
            color: {
                let r = self.color.r as f32 / 255.0;
                let g = self.color.g as f32 / 255.0;
                let b = self.color.b as f32 / 255.0;
                let a = self.opacity as f32 / 255.0;
                [r, g, b, a]
            },
        }
    }

    pub fn set_tag(&mut self, tag: i32) {
        self.node.set_tag(tag);
    }

    pub fn tag(&self) -> i32 {
        self.node.tag()
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.node.set_name(name);
    }

    pub fn name(&self) -> &str {
        self.node.name()
    }

    pub fn set_anchor_point(&mut self, anchor: Vec2) {
        self.node.set_anchor_point(anchor);
        self.custom_anchor = true;
    }

    pub fn get_anchor_point(&self) -> Vec2 {
        self.node.anchor_point()
    }

    pub fn set_z_order(&mut self, z: i32) {
        self.node.set_local_z_order(z);
    }

    pub fn get_z_order(&self) -> i32 {
        self.node.local_z_order()
    }

    // ===== 颜色和透明度 =====

    pub fn set_color(&mut self, color: Color3B) {
        self.color = color;
        self.node.set_color(color);
        self.dirty = true;
    }

    pub fn get_color(&self) -> Color3B {
        self.color
    }

    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity = opacity;
        self.node.set_opacity(opacity);
        self.dirty = true;
    }

    pub fn get_opacity(&self) -> u8 {
        self.opacity
    }

    pub fn set_color4b(&mut self, color: Color4B) {
        self.color = Color3B::new(color.r, color.g, color.b);
        self.opacity = color.a;
        self.node.set_color(self.color);
        self.node.set_opacity(color.a);
        self.dirty = true;
    }

    pub fn get_color4f(&self) -> Color4F {
        Color4F::new(
            self.color.r as f32 / 255.0,
            self.color.g as f32 / 255.0,
            self.color.b as f32 / 255.0,
            self.opacity as f32 / 255.0,
        )
    }

    // ===== 混合模式 =====

    pub fn set_blend_func(&mut self, blend: BlendFunc) {
        self.blend_func = blend;
    }

    pub fn get_blend_func(&self) -> BlendFunc {
        self.blend_func
    }

    // ===== 翻转 =====

    pub fn set_flip_x(&mut self, flipped_x: bool) {
        if self.flipped_x != flipped_x {
            self.flipped_x = flipped_x;
            self.update_uv_coords();
            self.dirty = true;
        }
    }

    pub fn is_flipped_x(&self) -> bool {
        self.flipped_x
    }

    pub fn set_flip_y(&mut self, flipped_y: bool) {
        if self.flipped_y != flipped_y {
            self.flipped_y = flipped_y;
            self.update_uv_coords();
            self.dirty = true;
        }
    }

    pub fn is_flipped_y(&self) -> bool {
        self.flipped_y
    }

    // ===== 纹理和精灵帧 =====

    pub fn set_texture(&mut self, texture: Rc<RefCell<Texture2D>>) {
        let (w, h) = {
            let t = texture.borrow();
            (t.width() as f32, t.height() as f32)
        };
        self.texture = Some(texture);
        if self.texture_rect == Rect::ZERO {
            self.set_texture_rect(Rect::new(0.0, 0.0, w, h));
        }
        self.dirty = true;
    }

    pub fn get_texture(&self) -> Option<Rc<RefCell<Texture2D>>> {
        self.texture.clone()
    }

    pub fn set_texture_rect(&mut self, rect: Rect) {
        self.texture_rect = rect;
        self.texture_rect_rotated = false;
        
        // 更新节点大小
        let size = Size::new(rect.size.width, rect.size.height);
        self.node.set_content_size(size);
        self.original_content_size = size;
        
        self.update_uv_coords();
        self.dirty = true;
    }

    pub fn get_texture_rect(&self) -> Rect {
        self.texture_rect
    }

    pub fn set_texture_rect_rotated(&mut self, rect: Rect, rotated: bool, untrimmed_size: Size) {
        self.texture_rect = rect;
        self.texture_rect_rotated = rotated;
        self.original_content_size = untrimmed_size;
        
        // 设置节点大小为裁剪后的大小
        let size = if rotated {
            Size::new(rect.size.height, rect.size.width)
        } else {
            Size::new(rect.size.width, rect.size.height)
        };
        self.node.set_content_size(size);
        
        self.update_uv_coords();
        self.dirty = true;
    }

    pub fn set_sprite_frame(&mut self, frame: Rc<RefCell<SpriteFrame>>) {
        let (rect, rotated, original_size, offset) = {
            let f = frame.borrow();
            (f.rect(), f.is_rotated(), f.original_size(), f.offset())
        };
        
        if let Some(ref f_ref) = frame.borrow().texture() {
            self.texture = Some(f_ref.clone());
        }
        
        self.sprite_frame = Some(frame);
        
        let untrimmed = Size::new(original_size.0, original_size.1);
        self.offset_position = Vec2::new(offset.0, offset.1);
        
        self.set_texture_rect_rotated(rect, rotated, untrimmed);
        self.dirty = true;
    }

    pub fn get_sprite_frame(&self) -> Option<Rc<RefCell<SpriteFrame>>> {
        self.sprite_frame.clone()
    }

    pub fn set_sprite_frame_name(&mut self, frame_name: &str) {
        // 从 SpriteFrameCache 获取帧（简化实现）
        let _ = frame_name;
    }

    // ===== 内容大小 =====

    pub fn get_content_size(&self) -> Size {
        self.node.content_size()
    }

    pub fn set_content_size(&mut self, size: Size) {
        self.node.set_content_size(size);
    }

    pub fn get_bounding_box(&self) -> Rect {
        let pos = self.node.position();
        let size = self.node.content_size();
        let anchor = self.node.anchor_point();
        Rect::new(
            pos.x - size.width * anchor.x,
            pos.y - size.height * anchor.y,
            size.width,
            size.height,
        )
    }

    // ===== 偏移 =====

    pub fn get_offset_position(&self) -> Vec2 {
        self.offset_position
    }

    // ===== 辅助方法 =====

    fn update_uv_coords(&mut self) {
        // 根据纹理矩形和翻转状态更新UV坐标
        if let Some(tex) = &self.texture {
            let tex = tex.borrow();
            let tw = tex.width() as f32;
            let th = tex.height() as f32;
            
            if tw > 0.0 && th > 0.0 {
                let rect = self.texture_rect;
                let left = rect.origin.x / tw;
                let right = (rect.origin.x + rect.size.width) / tw;
                let bottom = rect.origin.y / th;
                let top = (rect.origin.y + rect.size.height) / th;
                
                if self.texture_rect_rotated {
                    // 旋转UV
                    self.quad_uv = if !self.flipped_x && !self.flipped_y {
                        [[left, top], [left, bottom], [right, bottom], [right, top]]
                    } else if self.flipped_x && !self.flipped_y {
                        [[right, top], [right, bottom], [left, bottom], [left, top]]
                    } else if !self.flipped_x && self.flipped_y {
                        [[left, bottom], [left, top], [right, top], [right, bottom]]
                    } else {
                        [[right, bottom], [right, top], [left, top], [left, bottom]]
                    };
                } else {
                    self.quad_uv = if !self.flipped_x && !self.flipped_y {
                        [[left, bottom], [right, bottom], [right, top], [left, top]]
                    } else if self.flipped_x && !self.flipped_y {
                        [[right, bottom], [left, bottom], [left, top], [right, top]]
                    } else if !self.flipped_x && self.flipped_y {
                        [[left, top], [right, top], [right, bottom], [left, bottom]]
                    } else {
                        [[right, top], [left, top], [left, bottom], [right, bottom]]
                    };
                }
            }
        } else {
            // 无纹理时使用默认UV
            self.quad_uv = if !self.flipped_x && !self.flipped_y {
                [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]
            } else if self.flipped_x && !self.flipped_y {
                [[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]]
            } else if !self.flipped_x && self.flipped_y {
                [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]]
            } else {
                [[1.0, 1.0], [0.0, 1.0], [0.0, 0.0], [1.0, 0.0]]
            };
        }
    }

    pub fn get_quad_uv(&self) -> &[[f32; 2]; 4] {
        &self.quad_uv
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    /// 执行渲染前的更新（变换矩阵、UV等）
    pub fn update_transform(&mut self) {
        if self.dirty {
            self.dirty = false;
        }
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
        assert_eq!(sprite.get_opacity(), 255);
        assert_eq!(sprite.get_color(), Color3B::WHITE);
        assert!(!sprite.is_flipped_x());
        assert!(!sprite.is_flipped_y());
        assert!(sprite.is_visible());
    }

    #[test]
    fn test_sprite_default() {
        let sprite = Sprite::default();
        let _ = sprite.get_node();
    }

    #[test]
    fn test_sprite_color() {
        let mut sprite = Sprite::new();
        sprite.set_color(Color3B::new(255, 0, 0));
        assert_eq!(sprite.get_color(), Color3B::new(255, 0, 0));
    }

    #[test]
    fn test_sprite_opacity() {
        let mut sprite = Sprite::new();
        sprite.set_opacity(128);
        assert_eq!(sprite.get_opacity(), 128);
    }

    #[test]
    fn test_sprite_flip() {
        let mut sprite = Sprite::new();
        sprite.set_flip_x(true);
        assert!(sprite.is_flipped_x());
        sprite.set_flip_y(true);
        assert!(sprite.is_flipped_y());
    }

    #[test]
    fn test_sprite_position() {
        let mut sprite = Sprite::new();
        sprite.set_position(Vec2::new(100.0, 200.0));
        assert_eq!(sprite.get_position(), Vec2::new(100.0, 200.0));
    }

    #[test]
    fn test_sprite_scale() {
        let mut sprite = Sprite::new();
        sprite.set_scale(2.0);
        assert_eq!(sprite.get_scale(), 2.0);
    }

    #[test]
    fn test_sprite_rotation() {
        let mut sprite = Sprite::new();
        sprite.set_rotation(90.0);
        assert_eq!(sprite.get_rotation(), 90.0);
    }

    #[test]
    fn test_sprite_tag() {
        let mut sprite = Sprite::new();
        sprite.set_tag(42);
        assert_eq!(sprite.tag(), 42);
    }

    #[test]
    fn test_sprite_texture_rect() {
        let mut sprite = Sprite::new();
        sprite.set_texture_rect(Rect::new(0.0, 0.0, 100.0, 100.0));
        let size = sprite.get_content_size();
        assert_eq!(size.width, 100.0);
        assert_eq!(size.height, 100.0);
    }

    #[test]
    fn test_sprite_anchor_point() {
        let mut sprite = Sprite::new();
        // 默认锚点为 (0.5, 0.5)
        assert_eq!(sprite.get_anchor_point(), Vec2::new(0.5, 0.5));
        sprite.set_anchor_point(Vec2::new(0.0, 0.0));
        assert_eq!(sprite.get_anchor_point(), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_sprite_blend_func() {
        let mut sprite = Sprite::new();
        sprite.set_blend_func(BlendFunc::ALPHA_NON_PREMULTIPLIED);
        assert_eq!(sprite.get_blend_func(), BlendFunc::ALPHA_NON_PREMULTIPLIED);
    }

    #[test]
    fn test_sprite_bounding_box() {
        let mut sprite = Sprite::new();
        sprite.set_position(Vec2::new(100.0, 100.0));
        sprite.set_texture_rect(Rect::new(0.0, 0.0, 100.0, 100.0));
        let bbox = sprite.get_bounding_box();
        // 锚点在中心，所以 bbox.x = 100 - 100*0.5 = 50
        assert!((bbox.origin.x - 50.0).abs() < 0.01);
        assert!((bbox.origin.y - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_sprite_batch_node() {
        let mut batch = SpriteBatchNode::new();
        let sprite1 = Sprite::new();
        let sprite2 = Sprite::new();
        
        batch.add_sprite(sprite1);
        batch.add_sprite(sprite2);
        
        assert_eq!(batch.get_sprite_count(), 2);
    }

    #[test]
    fn test_sprite_batch_node_capacity() {
        let batch = SpriteBatchNode::create_with_capacity(256);
        assert_eq!(batch.get_capacity(), 256);
        assert_eq!(batch.get_sprite_count(), 0);
    }

    #[test]
    fn test_sprite_batch_node_remove() {
        let mut batch = SpriteBatchNode::new();
        batch.add_sprite(Sprite::new());
        batch.add_sprite(Sprite::new());
        batch.add_sprite(Sprite::new());
        assert_eq!(batch.get_sprite_count(), 3);

        batch.remove_sprite_at(1);
        assert_eq!(batch.get_sprite_count(), 2);

        batch.remove_all_sprites();
        assert_eq!(batch.get_sprite_count(), 0);
    }

    #[test]
    fn test_sprite_batch_node_dirty() {
        let mut batch = SpriteBatchNode::new();
        assert!(!batch.is_dirty());

        batch.add_sprite(Sprite::new());
        assert!(batch.is_dirty());

        batch.clear_dirty();
        assert!(!batch.is_dirty());
    }

    #[test]
    fn test_sprite_batch_node_generate_data() {
        let mut batch = SpriteBatchNode::new();

        let mut s1 = Sprite::new();
        s1.set_position(Vec2::new(0.0, 0.0));
        s1.set_texture_rect(Rect::new(0.0, 0.0, 64.0, 64.0));

        let mut s2 = Sprite::new();
        s2.set_position(Vec2::new(100.0, 100.0));
        s2.set_texture_rect(Rect::new(0.0, 0.0, 64.0, 64.0));

        let mut s3 = Sprite::new();
        s3.set_visible(false); // 隐藏精灵不应生成顶点

        batch.add_sprite(s1);
        batch.add_sprite(s2);
        batch.add_sprite(s3);

        let data = batch.generate_batch_data();
        assert_eq!(data.quad_count(), 2); // 隐藏精灵不计入
        assert_eq!(data.vertex_count(), 8);
        assert_eq!(data.index_count(), 12);
    }

    #[test]
    fn test_sprite_generate_quad() {
        let mut sprite = Sprite::new();
        sprite.set_position(Vec2::new(100.0, 200.0));
        sprite.set_texture_rect(Rect::new(0.0, 0.0, 50.0, 80.0));
        sprite.set_color(Color3B::new(255, 128, 64));
        sprite.set_opacity(200);

        let quad = sprite.generate_quad();

        // 检查左下顶点
        assert!((quad.vertices[0][0] - 100.0).abs() < 0.01);
        assert!((quad.vertices[0][1] - 200.0).abs() < 0.01);

        // 检查颜色（r 约 1.0）
        assert!((quad.color[0] - 1.0).abs() < 0.01);
        // 检查 alpha
        assert!((quad.color[3] - 200.0 / 255.0).abs() < 0.01);
    }

    #[test]
    fn test_sprite_visible() {
        let mut sprite = Sprite::new();
        assert!(sprite.is_visible());
        sprite.set_visible(false);
        assert!(!sprite.is_visible());
    }

    #[test]
    fn test_sprite_uv_flip() {
        let mut sprite = Sprite::new();
        let uv_before = *sprite.get_quad_uv();
        sprite.set_flip_x(true);
        let uv_after = *sprite.get_quad_uv();
        // UV应该发生变化
        assert_ne!(uv_before[0], uv_after[0]);
    }

    #[test]
    fn test_sprite_color4f() {
        let mut sprite = Sprite::new();
        sprite.set_color(Color3B::new(255, 128, 0));
        sprite.set_opacity(200);
        let color4f = sprite.get_color4f();
        assert!((color4f.r - 1.0).abs() < 0.01);
        assert!((color4f.a - 200.0 / 255.0).abs() < 0.01);
    }
}
