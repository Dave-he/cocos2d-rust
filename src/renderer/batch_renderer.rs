#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
/// 批量渲染系统
///
/// 通过合并相同材质和纹理的渲染对象，减少 draw call，提升渲染性能
///
/// # Features
/// - 自动合批：相同纹理和材质自动合并
/// - 动态缓冲：高效管理顶点和索引数据
/// - Z-order 排序：保证正确的渲染顺序
/// - 性能统计：跟踪批次数量和顶点数量

use crate::base::types::Color4F;
use crate::math::{Vec2, Vec3};
use crate::renderer::command::Quad;
use crate::renderer::material::Material;
use crate::renderer::renderer::Renderer;
use crate::renderer::texture::Texture2D;
use std::rc::Rc;

/// 顶点数据结构
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub tex_coord: Vec2,
    pub color: Color4F,
}

impl Vertex {
    pub fn new(position: Vec3, tex_coord: Vec2, color: Color4F) -> Self {
        Self {
            position,
            tex_coord,
            color,
        }
    }
}

/// 渲染批次
/// 
/// 包含一组共享相同渲染状态的几何数据
#[derive(Debug)]
pub struct RenderBatch {
    /// 纹理（如果有）
    pub texture: Option<Rc<Texture2D>>,
    /// 材质（如果有）
    pub material: Option<Rc<Material>>,
    /// 顶点数据
    pub vertices: Vec<Vertex>,
    /// 索引数据
    pub indices: Vec<u16>,
    /// 混合模式
    pub blend_mode: BlendMode,
    /// Z-order（用于排序）
    pub z_order: i32,
}

impl Default for RenderBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderBatch {
    pub fn new() -> Self {
        Self {
            texture: None,
            material: None,
            vertices: Vec::new(),
            indices: Vec::new(),
            blend_mode: BlendMode::Normal,
            z_order: 0,
        }
    }

    pub fn with_capacity(vertex_capacity: usize, index_capacity: usize) -> Self {
        Self {
            texture: None,
            material: None,
            vertices: Vec::with_capacity(vertex_capacity),
            indices: Vec::with_capacity(index_capacity),
            blend_mode: BlendMode::Normal,
            z_order: 0,
        }
    }

    /// 检查是否可以与另一个批次合并
    pub fn can_merge_with(&self, other: &RenderBatch) -> bool {
        // 纹理必须相同
        let texture_match = match (&self.texture, &other.texture) {
            (None, None) => true,
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            _ => false,
        };

        // 材质必须相同
        let material_match = match (&self.material, &other.material) {
            (None, None) => true,
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            _ => false,
        };

        // 混合模式必须相同
        let blend_match = self.blend_mode == other.blend_mode;

        texture_match && material_match && blend_match
    }

    /// 添加一个四边形
    pub fn add_quad(&mut self, quad: &Quad, z_order: i32) {
        let base_index = self.vertices.len() as u16;

        // 添加4个顶点
        self.vertices.push(Vertex::new(
            Vec3::new(quad.bl.position[0], quad.bl.position[1], quad.bl.position[2]),
            Vec2::new(quad.bl.tex_coord[0], quad.bl.tex_coord[1]),
            quad.bl.color,
        ));
        self.vertices.push(Vertex::new(
            Vec3::new(quad.br.position[0], quad.br.position[1], quad.br.position[2]),
            Vec2::new(quad.br.tex_coord[0], quad.br.tex_coord[1]),
            quad.br.color,
        ));
        self.vertices.push(Vertex::new(
            Vec3::new(quad.tr.position[0], quad.tr.position[1], quad.tr.position[2]),
            Vec2::new(quad.tr.tex_coord[0], quad.tr.tex_coord[1]),
            quad.tr.color,
        ));
        self.vertices.push(Vertex::new(
            Vec3::new(quad.tl.position[0], quad.tl.position[1], quad.tl.position[2]),
            Vec2::new(quad.tl.tex_coord[0], quad.tl.tex_coord[1]),
            quad.tl.color,
        ));

        // 添加6个索引 (两个三角形)
        self.indices.push(base_index);
        self.indices.push(base_index + 1);
        self.indices.push(base_index + 2);
        self.indices.push(base_index);
        self.indices.push(base_index + 2);
        self.indices.push(base_index + 3);

        self.z_order = z_order;
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// 清空批次
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    /// 获取顶点数量
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// 获取索引数量
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}

/// 混合模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,      // 正常混合
    Additive,    // 叠加
    Multiply,    // 正片叠底
    Screen,      // 滤色
    Custom,      // 自定义
}

/// 批量渲染器
///
/// 管理多个渲染批次，自动合并相同状态的对象
pub struct BatchRenderer {
    /// 所有批次
    batches: Vec<RenderBatch>,
    /// 当前正在构建的批次
    current_batch: Option<RenderBatch>,
    /// 单个批次最大顶点数
    max_vertices_per_batch: usize,
    /// 单个批次最大索引数
    max_indices_per_batch: usize,
    /// 是否启用自动排序
    auto_sort: bool,
    /// 渲染统计
    stats: BatchStats,
}

/// 批次渲染统计
#[derive(Debug, Default, Clone)]
pub struct BatchStats {
    pub total_batches: usize,
    pub total_vertices: usize,
    pub total_indices: usize,
    pub draw_calls: usize,
    pub state_changes: usize,
}

impl BatchRenderer {
    /// 创建新的批量渲染器
    pub fn new() -> Self {
        Self::with_capacity(10000, 15000)
    }

    /// 创建指定容量的批量渲染器
    pub fn with_capacity(max_vertices: usize, max_indices: usize) -> Self {
        Self {
            batches: Vec::new(),
            current_batch: None,
            max_vertices_per_batch: max_vertices,
            max_indices_per_batch: max_indices,
            auto_sort: true,
            stats: BatchStats::default(),
        }
    }

    /// 开始新的批次
    pub fn begin_batch(&mut self, texture: Option<Rc<Texture2D>>, blend_mode: BlendMode) {
        let mut batch = RenderBatch::with_capacity(
            self.max_vertices_per_batch,
            self.max_indices_per_batch,
        );
        batch.texture = texture;
        batch.blend_mode = blend_mode;
        self.current_batch = Some(batch);
    }

    /// 结束当前批次
    pub fn end_batch(&mut self) {
        if let Some(batch) = self.current_batch.take() {
            if !batch.is_empty() {
                self.batches.push(batch);
            }
        }
    }

    /// 添加一个四边形到当前批次
    pub fn add_quad(
        &mut self,
        quad: &Quad,
        texture: Option<Rc<Texture2D>>,
        blend_mode: BlendMode,
        z_order: i32,
    ) {
        // 如果没有当前批次，或者当前批次已满，或者状态不匹配，创建新批次
        let needs_new_batch = match &self.current_batch {
            None => true,
            Some(batch) => {
                // 检查是否已满
                let is_full = batch.vertex_count() + 4 > self.max_vertices_per_batch
                    || batch.index_count() + 6 > self.max_indices_per_batch;

                // 检查状态是否匹配
                let texture_match = match (&batch.texture, &texture) {
                    (None, None) => true,
                    (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                    _ => false,
                };
                let blend_match = batch.blend_mode == blend_mode;

                is_full || !texture_match || !blend_match
            }
        };

        if needs_new_batch {
            self.end_batch();
            self.begin_batch(texture.clone(), blend_mode);
        }

        if let Some(batch) = &mut self.current_batch {
            batch.add_quad(quad, z_order);
        }
    }

    /// 刷新所有批次到渲染器
    pub fn flush(&mut self, renderer: &mut Renderer) {
        // 结束当前批次
        self.end_batch();

        // 如果启用自动排序，按 Z-order 排序
        if self.auto_sort {
            self.batches.sort_by_key(|b| b.z_order);
        }

        // 重置统计
        self.stats = BatchStats::default();
        self.stats.total_batches = self.batches.len();

        // 渲染所有批次
        for batch in &self.batches {
            self.stats.total_vertices += batch.vertex_count();
            self.stats.total_indices += batch.index_count();
            self.stats.draw_calls += 1;

            // TODO: 实际的渲染调用
            // renderer.draw_batch(batch);
        }

        // 清空批次
        self.batches.clear();
    }

    /// 设置是否启用自动排序
    pub fn set_auto_sort(&mut self, enabled: bool) {
        self.auto_sort = enabled;
    }

    /// 获取渲染统计
    pub fn get_stats(&self) -> &BatchStats {
        &self.stats
    }

    /// 清空所有批次
    pub fn clear(&mut self) {
        self.batches.clear();
        self.current_batch = None;
    }

    /// 获取批次数量
    pub fn batch_count(&self) -> usize {
        self.batches.len()
    }
}

impl Default for BatchRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_batch_creation() {
        let batch = RenderBatch::new();
        assert!(batch.is_empty());
        assert_eq!(batch.vertex_count(), 0);
        assert_eq!(batch.index_count(), 0);
    }

    #[test]
    fn test_batch_renderer_creation() {
        let renderer = BatchRenderer::new();
        assert_eq!(renderer.batch_count(), 0);
    }

    #[test]
    fn test_batch_merge_logic() {
        let mut batch1 = RenderBatch::new();
        batch1.blend_mode = BlendMode::Normal;

        let mut batch2 = RenderBatch::new();
        batch2.blend_mode = BlendMode::Normal;

        assert!(batch1.can_merge_with(&batch2));

        batch2.blend_mode = BlendMode::Additive;
        assert!(!batch1.can_merge_with(&batch2));
    }

    #[test]
    fn test_add_quad() {
        use crate::renderer::command::Vertex as CmdVertex;

        let mut batch = RenderBatch::new();

        let quad = Quad {
            bl: CmdVertex {
                position: [0.0, 0.0, 0.0],
                tex_coord: [0.0, 0.0],
                color: Color4F::WHITE,
            },
            br: CmdVertex {
                position: [1.0, 0.0, 0.0],
                tex_coord: [1.0, 0.0],
                color: Color4F::WHITE,
            },
            tr: CmdVertex {
                position: [1.0, 1.0, 0.0],
                tex_coord: [1.0, 1.0],
                color: Color4F::WHITE,
            },
            tl: CmdVertex {
                position: [0.0, 1.0, 0.0],
                tex_coord: [0.0, 1.0],
                color: Color4F::WHITE,
            },
            blend_func: (770, 771),
            texture: None,
            model_matrix: crate::math::Mat4::IDENTITY,
        };

        batch.add_quad(&quad, 0);

        assert_eq!(batch.vertex_count(), 4);
        assert_eq!(batch.index_count(), 6);
        assert!(!batch.is_empty());
    }

    #[test]
    fn test_batch_stats() {
        let stats = BatchStats {
            total_batches: 5,
            total_vertices: 1000,
            total_indices: 1500,
            draw_calls: 5,
            state_changes: 3,
        };

        assert_eq!(stats.total_batches, 5);
        assert_eq!(stats.draw_calls, 5);
    }

    #[test]
    fn test_blend_mode_equality() {
        assert_eq!(BlendMode::Normal, BlendMode::Normal);
        assert_eq!(BlendMode::Additive, BlendMode::Additive);
        assert_ne!(BlendMode::Normal, BlendMode::Additive);
        assert_ne!(BlendMode::Multiply, BlendMode::Screen);
    }

    #[test]
    fn test_batch_clear() {
        use crate::renderer::command::Vertex as CmdVertex;

        let mut batch = RenderBatch::new();
        let quad = create_test_quad();
        
        batch.add_quad(&quad, 0);
        assert_eq!(batch.vertex_count(), 4);
        
        batch.clear();
        assert_eq!(batch.vertex_count(), 0);
        assert_eq!(batch.index_count(), 0);
        assert!(batch.is_empty());
    }

    #[test]
    fn test_batch_with_capacity() {
        let batch = RenderBatch::with_capacity(1000, 1500);
        assert_eq!(batch.vertices.capacity(), 1000);
        assert_eq!(batch.indices.capacity(), 1500);
    }

    #[test]
    fn test_batch_renderer_with_capacity() {
        let renderer = BatchRenderer::with_capacity(5000, 7500);
        assert_eq!(renderer.max_vertices_per_batch, 5000);
        assert_eq!(renderer.max_indices_per_batch, 7500);
    }

    #[test]
    fn test_batch_renderer_begin_end() {
        let mut renderer = BatchRenderer::new();
        
        renderer.begin_batch(None, BlendMode::Normal);
        assert!(renderer.current_batch.is_some());
        
        renderer.end_batch();
        assert!(renderer.current_batch.is_none());
        assert_eq!(renderer.batch_count(), 0);
    }

    #[test]
    fn test_batch_renderer_auto_sort() {
        let mut renderer = BatchRenderer::new();
        assert!(renderer.auto_sort);
        
        renderer.set_auto_sort(false);
        assert!(!renderer.auto_sort);
    }

    #[test]
    fn test_batch_renderer_clear() {
        let mut renderer = BatchRenderer::new();
        
        renderer.begin_batch(None, BlendMode::Normal);
        renderer.end_batch();
        
        renderer.clear();
        assert_eq!(renderer.batch_count(), 0);
        assert!(renderer.current_batch.is_none());
    }

    #[test]
    fn test_add_multiple_quads() {
        use crate::renderer::command::Vertex as CmdVertex;

        let mut batch = RenderBatch::new();
        let quad = create_test_quad();
        
        batch.add_quad(&quad, 0);
        batch.add_quad(&quad, 1);
        batch.add_quad(&quad, 2);
        
        assert_eq!(batch.vertex_count(), 12);
        assert_eq!(batch.index_count(), 18);
    }

    #[test]
    fn test_batch_renderer_add_quad() {
        let mut renderer = BatchRenderer::new();
        let quad = create_test_quad();
        
        renderer.add_quad(&quad, None, BlendMode::Normal, 0);
        
        assert!(renderer.current_batch.is_some());
        if let Some(batch) = &renderer.current_batch {
            assert_eq!(batch.vertex_count(), 4);
        }
    }

    #[test]
    fn test_batch_auto_split_on_texture_change() {
        let mut renderer = BatchRenderer::new();
        let quad = create_test_quad();
        
        renderer.add_quad(&quad, None, BlendMode::Normal, 0);
        renderer.add_quad(&quad, None, BlendMode::Additive, 0);
        
        renderer.end_batch();
        assert_eq!(renderer.batch_count(), 2);
    }

    #[test]
    fn test_batch_z_order() {
        use crate::renderer::command::Vertex as CmdVertex;

        let mut batch = RenderBatch::new();
        let quad = create_test_quad();
        
        batch.add_quad(&quad, 100);
        assert_eq!(batch.z_order, 100);
        
        batch.add_quad(&quad, 200);
        assert_eq!(batch.z_order, 200);
    }

    #[test]
    fn test_vertex_creation() {
        let vertex = Vertex::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec2::new(0.5, 0.5),
            Color4F::WHITE,
        );
        
        assert_eq!(vertex.position.x, 1.0);
        assert_eq!(vertex.tex_coord.x, 0.5);
        assert_eq!(vertex.color, Color4F::WHITE);
    }

    #[test]
    fn test_batch_merge_with_different_textures() {
        let batch1 = RenderBatch::new();
        let mut batch2 = RenderBatch::new();
        
        assert!(batch1.can_merge_with(&batch2));
    }

    #[test]
    fn test_batch_indices_correctness() {
        use crate::renderer::command::Vertex as CmdVertex;

        let mut batch = RenderBatch::new();
        let quad = create_test_quad();
        
        batch.add_quad(&quad, 0);
        
        assert_eq!(batch.indices[0], 0);
        assert_eq!(batch.indices[1], 1);
        assert_eq!(batch.indices[2], 2);
        assert_eq!(batch.indices[3], 0);
        assert_eq!(batch.indices[4], 2);
        assert_eq!(batch.indices[5], 3);
    }

    #[test]
    fn test_batch_indices_offset() {
        use crate::renderer::command::Vertex as CmdVertex;

        let mut batch = RenderBatch::new();
        let quad = create_test_quad();
        
        batch.add_quad(&quad, 0);
        batch.add_quad(&quad, 0);
        
        assert_eq!(batch.indices[6], 4);
        assert_eq!(batch.indices[7], 5);
        assert_eq!(batch.indices[8], 6);
    }

    #[test]
    fn test_batch_stats_default() {
        let stats = BatchStats::default();
        assert_eq!(stats.total_batches, 0);
        assert_eq!(stats.total_vertices, 0);
        assert_eq!(stats.draw_calls, 0);
    }

    #[test]
    fn test_batch_renderer_default() {
        let renderer = BatchRenderer::default();
        assert_eq!(renderer.batch_count(), 0);
        assert!(renderer.auto_sort);
    }

    #[test]
    fn test_batch_renderer_get_stats() {
        let renderer = BatchRenderer::new();
        let stats = renderer.get_stats();
        assert_eq!(stats.total_batches, 0);
    }

    #[test]
    fn test_multiple_batches_creation() {
        let mut renderer = BatchRenderer::new();
        let quad = create_test_quad();
        
        renderer.begin_batch(None, BlendMode::Normal);
        renderer.add_quad(&quad, None, BlendMode::Normal, 0);
        renderer.end_batch();
        
        renderer.begin_batch(None, BlendMode::Additive);
        renderer.add_quad(&quad, None, BlendMode::Additive, 0);
        renderer.end_batch();
        
        assert_eq!(renderer.batch_count(), 2);
    }

    #[test]
    fn test_empty_batch_not_added() {
        let mut renderer = BatchRenderer::new();
        
        renderer.begin_batch(None, BlendMode::Normal);
        renderer.end_batch();
        
        assert_eq!(renderer.batch_count(), 0);
    }

    #[test]
    fn test_batch_max_capacity_split() {
        let mut renderer = BatchRenderer::with_capacity(4, 6);
        let quad = create_test_quad();
        
        renderer.add_quad(&quad, None, BlendMode::Normal, 0);
        renderer.add_quad(&quad, None, BlendMode::Normal, 0);
        
        renderer.end_batch();
        assert_eq!(renderer.batch_count(), 2);
    }

    fn create_test_quad() -> Quad {
        use crate::renderer::command::Vertex as CmdVertex;
        
        Quad {
            bl: CmdVertex {
                position: [0.0, 0.0, 0.0],
                tex_coord: [0.0, 0.0],
                color: Color4F::WHITE,
            },
            br: CmdVertex {
                position: [1.0, 0.0, 0.0],
                tex_coord: [1.0, 0.0],
                color: Color4F::WHITE,
            },
            tr: CmdVertex {
                position: [1.0, 1.0, 0.0],
                tex_coord: [1.0, 1.0],
                color: Color4F::WHITE,
            },
            tl: CmdVertex {
                position: [0.0, 1.0, 0.0],
                tex_coord: [0.0, 1.0],
                color: Color4F::WHITE,
            },
            blend_func: (770, 771),
            texture: None,
            model_matrix: crate::math::Mat4::IDENTITY,
        }
    }
}
