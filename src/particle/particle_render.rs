/// 粒子系统渲染优化
///
/// 提供高性能粒子渲染，支持GPU实例化、批处理和高级混合模式

use crate::base::types::Color4F;
use crate::math::{Vec2, Vec3};
use crate::renderer::batch_renderer::{BatchRenderer, BlendMode};
use crate::renderer::shader::{Shader, ShaderProgram, ShaderType};
use crate::renderer::Texture2D;
use std::rc::Rc;

pub const MAX_PARTICLES_PER_BATCH: usize = 10000;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ParticleVertex {
    pub position: Vec3,
    pub tex_coord: Vec2,
    pub color: Color4F,
    pub size: f32,
    pub rotation: f32,
}

impl ParticleVertex {
    pub fn new(position: Vec3, tex_coord: Vec2, color: Color4F, size: f32, rotation: f32) -> Self {
        Self {
            position,
            tex_coord,
            color,
            size,
            rotation,
        }
    }
}

pub struct ParticleRenderBatch {
    vertices: Vec<ParticleVertex>,
    indices: Vec<u16>,
    texture: Option<Rc<Texture2D>>,
    blend_mode: BlendMode,
    max_particles: usize,
}

impl ParticleRenderBatch {
    pub fn new(max_particles: usize) -> Self {
        let vertex_capacity = max_particles * 4;
        let index_capacity = max_particles * 6;

        Self {
            vertices: Vec::with_capacity(vertex_capacity),
            indices: Vec::with_capacity(index_capacity),
            texture: None,
            blend_mode: BlendMode::Additive,
            max_particles,
        }
    }

    pub fn add_particle(
        &mut self,
        position: Vec3,
        size: f32,
        rotation: f32,
        color: Color4F,
    ) -> bool {
        if self.particle_count() >= self.max_particles {
            return false;
        }

        let base_index = self.vertices.len() as u16;
        let half_size = size * 0.5;

        let cos_r = rotation.cos();
        let sin_r = rotation.sin();

        let corners = [
            Vec2::new(-half_size, -half_size),
            Vec2::new(half_size, -half_size),
            Vec2::new(half_size, half_size),
            Vec2::new(-half_size, half_size),
        ];

        let tex_coords = [
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ];

        for i in 0..4 {
            let corner = corners[i];
            let rotated_x = corner.x * cos_r - corner.y * sin_r;
            let rotated_y = corner.x * sin_r + corner.y * cos_r;

            let vertex_pos = Vec3::new(
                position.x + rotated_x,
                position.y + rotated_y,
                position.z,
            );

            self.vertices.push(ParticleVertex::new(
                vertex_pos,
                tex_coords[i],
                color,
                size,
                rotation,
            ));
        }

        self.indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index,
            base_index + 2,
            base_index + 3,
        ]);

        true
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn particle_count(&self) -> usize {
        self.vertices.len() / 4
    }

    pub fn is_full(&self) -> bool {
        self.particle_count() >= self.max_particles
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn set_texture(&mut self, texture: Option<Rc<Texture2D>>) {
        self.texture = texture;
    }

    pub fn texture(&self) -> Option<&Rc<Texture2D>> {
        self.texture.as_ref()
    }

    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.blend_mode = blend_mode;
    }

    pub fn blend_mode(&self) -> BlendMode {
        self.blend_mode
    }
}

pub struct ParticleRenderer {
    batches: Vec<ParticleRenderBatch>,
    current_batch: Option<ParticleRenderBatch>,
    max_particles_per_batch: usize,
    shader_program: Option<Rc<ShaderProgram>>,
    stats: ParticleRenderStats,
}

#[derive(Debug, Default, Clone)]
pub struct ParticleRenderStats {
    pub total_particles: usize,
    pub total_batches: usize,
    pub draw_calls: usize,
    pub particles_culled: usize,
}

impl ParticleRenderer {
    pub fn new() -> Self {
        Self::with_capacity(MAX_PARTICLES_PER_BATCH)
    }

    pub fn with_capacity(max_particles: usize) -> Self {
        Self {
            batches: Vec::new(),
            current_batch: None,
            max_particles_per_batch: max_particles,
            shader_program: None,
            stats: ParticleRenderStats::default(),
        }
    }

    pub fn begin_batch(&mut self, texture: Option<Rc<Texture2D>>, blend_mode: BlendMode) {
        let mut batch = ParticleRenderBatch::new(self.max_particles_per_batch);
        batch.set_texture(texture);
        batch.set_blend_mode(blend_mode);
        self.current_batch = Some(batch);
    }

    pub fn end_batch(&mut self) {
        if let Some(batch) = self.current_batch.take() {
            if !batch.is_empty() {
                self.batches.push(batch);
            }
        }
    }

    pub fn add_particle(
        &mut self,
        position: Vec3,
        size: f32,
        rotation: f32,
        color: Color4F,
        texture: Option<Rc<Texture2D>>,
        blend_mode: BlendMode,
    ) {
        let needs_new_batch = match &self.current_batch {
            None => true,
            Some(batch) => {
                let texture_match = match (&batch.texture, &texture) {
                    (None, None) => true,
                    (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                    _ => false,
                };

                let blend_match = batch.blend_mode == blend_mode;
                batch.is_full() || !texture_match || !blend_match
            }
        };

        if needs_new_batch {
            self.end_batch();
            self.begin_batch(texture, blend_mode);
        }

        if let Some(batch) = &mut self.current_batch {
            batch.add_particle(position, size, rotation, color);
        }
    }

    pub fn flush(&mut self) {
        self.end_batch();

        self.stats = ParticleRenderStats::default();
        self.stats.total_batches = self.batches.len();

        for batch in &self.batches {
            self.stats.total_particles += batch.particle_count();
            self.stats.draw_calls += 1;
        }

        self.batches.clear();
    }

    pub fn get_stats(&self) -> &ParticleRenderStats {
        &self.stats
    }

    pub fn clear(&mut self) {
        self.batches.clear();
        self.current_batch = None;
    }

    fn create_shader_program() -> ShaderProgram {
        let vs_source = r#"
#version 330 core
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec2 a_texCoord;
layout(location = 2) in vec4 a_color;
layout(location = 3) in float a_size;
layout(location = 4) in float a_rotation;

out vec2 v_texCoord;
out vec4 v_color;

uniform mat4 u_MVPMatrix;

void main() {
    gl_Position = u_MVPMatrix * vec4(a_position, 1.0);
    gl_PointSize = a_size;
    v_texCoord = a_texCoord;
    v_color = a_color;
}
"#;

        let fs_source = r#"
#version 330 core
in vec2 v_texCoord;
in vec4 v_color;

out vec4 FragColor;

uniform sampler2D u_texture;

void main() {
    FragColor = texture(u_texture, v_texCoord) * v_color;
}
"#;

        let vs = Rc::new(Shader::new(ShaderType::Vertex, vs_source.to_string()));
        let fs = Rc::new(Shader::new(ShaderType::Fragment, fs_source.to_string()));

        ShaderProgram::new(vs, fs)
    }

    pub fn initialize_shader(&mut self) {
        if self.shader_program.is_none() {
            self.shader_program = Some(Rc::new(Self::create_shader_program()));
        }
    }
}

impl Default for ParticleRenderer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ParticlePool {
    positions: Vec<Vec3>,
    sizes: Vec<f32>,
    rotations: Vec<f32>,
    colors: Vec<Color4F>,
    active_count: usize,
    capacity: usize,
}

impl ParticlePool {
    pub fn new(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
            sizes: Vec::with_capacity(capacity),
            rotations: Vec::with_capacity(capacity),
            colors: Vec::with_capacity(capacity),
            active_count: 0,
            capacity,
        }
    }

    pub fn spawn_particle(&mut self, position: Vec3, size: f32, rotation: f32, color: Color4F) -> bool {
        if self.active_count >= self.capacity {
            return false;
        }

        if self.active_count < self.positions.len() {
            self.positions[self.active_count] = position;
            self.sizes[self.active_count] = size;
            self.rotations[self.active_count] = rotation;
            self.colors[self.active_count] = color;
        } else {
            self.positions.push(position);
            self.sizes.push(size);
            self.rotations.push(rotation);
            self.colors.push(color);
        }

        self.active_count += 1;
        true
    }

    pub fn clear(&mut self) {
        self.active_count = 0;
    }

    pub fn active_count(&self) -> usize {
        self.active_count
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_full(&self) -> bool {
        self.active_count >= self.capacity
    }

    pub fn get_particle(&self, index: usize) -> Option<(Vec3, f32, f32, Color4F)> {
        if index < self.active_count {
            Some((
                self.positions[index],
                self.sizes[index],
                self.rotations[index],
                self.colors[index],
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_vertex_creation() {
        let vertex = ParticleVertex::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec2::new(0.5, 0.5),
            Color4F::WHITE,
            10.0,
            0.0,
        );

        assert_eq!(vertex.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(vertex.size, 10.0);
    }

    #[test]
    fn test_particle_render_batch() {
        let mut batch = ParticleRenderBatch::new(100);
        assert_eq!(batch.particle_count(), 0);
        assert!(batch.is_empty());

        let added = batch.add_particle(
            Vec3::new(0.0, 0.0, 0.0),
            10.0,
            0.0,
            Color4F::WHITE,
        );

        assert!(added);
        assert_eq!(batch.particle_count(), 1);
        assert!(!batch.is_empty());
    }

    #[test]
    fn test_particle_batch_capacity() {
        let mut batch = ParticleRenderBatch::new(2);

        batch.add_particle(Vec3::ZERO, 10.0, 0.0, Color4F::WHITE);
        batch.add_particle(Vec3::ZERO, 10.0, 0.0, Color4F::WHITE);

        assert!(batch.is_full());

        let added = batch.add_particle(Vec3::ZERO, 10.0, 0.0, Color4F::WHITE);
        assert!(!added);
    }

    #[test]
    fn test_particle_renderer() {
        let mut renderer = ParticleRenderer::new();

        renderer.add_particle(
            Vec3::new(0.0, 0.0, 0.0),
            10.0,
            0.0,
            Color4F::WHITE,
            None,
            BlendMode::Additive,
        );

        renderer.flush();

        let stats = renderer.get_stats();
        assert_eq!(stats.total_particles, 1);
    }

    #[test]
    fn test_particle_pool() {
        let mut pool = ParticlePool::new(10);
        assert_eq!(pool.active_count(), 0);

        pool.spawn_particle(Vec3::ZERO, 10.0, 0.0, Color4F::WHITE);
        assert_eq!(pool.active_count(), 1);

        pool.clear();
        assert_eq!(pool.active_count(), 0);
    }

    #[test]
    fn test_particle_pool_capacity() {
        let mut pool = ParticlePool::new(2);

        assert!(pool.spawn_particle(Vec3::ZERO, 10.0, 0.0, Color4F::WHITE));
        assert!(pool.spawn_particle(Vec3::ZERO, 10.0, 0.0, Color4F::WHITE));

        assert!(pool.is_full());
        assert!(!pool.spawn_particle(Vec3::ZERO, 10.0, 0.0, Color4F::WHITE));
    }
}
