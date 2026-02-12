use crate::base::types::Color4F;
use crate::math::{Mat4, Rect};

#[derive(Debug, Clone, Copy)]
pub struct InstanceData {
    pub transform: Mat4,
    pub color: Color4F,
    pub uv_rect: Rect,
}

impl InstanceData {
    pub fn new(transform: Mat4, color: Color4F, uv_rect: Rect) -> Self {
        Self {
            transform,
            color,
            uv_rect,
        }
    }

    pub fn with_transform(transform: Mat4) -> Self {
        Self {
            transform,
            color: Color4F::WHITE,
            uv_rect: Rect::new(0.0, 0.0, 1.0, 1.0),
        }
    }

    pub fn with_color(color: Color4F) -> Self {
        Self {
            transform: Mat4::IDENTITY,
            color,
            uv_rect: Rect::new(0.0, 0.0, 1.0, 1.0),
        }
    }
}

impl Default for InstanceData {
    fn default() -> Self {
        Self {
            transform: Mat4::IDENTITY,
            color: Color4F::WHITE,
            uv_rect: Rect::new(0.0, 0.0, 1.0, 1.0),
        }
    }
}

pub struct InstancedRenderer {
    instances: Vec<InstanceData>,
    instance_buffer: u32,
    max_instances: usize,
    instance_count: usize,
}

impl InstancedRenderer {
    pub fn new(max_instances: usize) -> Self {
        Self {
            instances: Vec::with_capacity(max_instances),
            instance_buffer: 0,
            max_instances,
            instance_count: 0,
        }
    }

    pub fn with_buffer_id(max_instances: usize, buffer_id: u32) -> Self {
        Self {
            instances: Vec::with_capacity(max_instances),
            instance_buffer: buffer_id,
            max_instances,
            instance_count: 0,
        }
    }

    pub fn add_instance(&mut self, data: InstanceData) -> bool {
        if self.instances.len() >= self.max_instances {
            return false;
        }
        
        self.instances.push(data);
        self.instance_count += 1;
        true
    }

    pub fn add_instances(&mut self, instances: &[InstanceData]) -> usize {
        let available = self.max_instances - self.instances.len();
        let to_add = instances.len().min(available);
        
        self.instances.extend_from_slice(&instances[..to_add]);
        self.instance_count += to_add;
        to_add
    }

    pub fn remove_instance(&mut self, index: usize) -> Option<InstanceData> {
        if index < self.instances.len() {
            self.instance_count = self.instance_count.saturating_sub(1);
            Some(self.instances.remove(index))
        } else {
            None
        }
    }

    pub fn get_instance(&self, index: usize) -> Option<&InstanceData> {
        self.instances.get(index)
    }

    pub fn get_instance_mut(&mut self, index: usize) -> Option<&mut InstanceData> {
        self.instances.get_mut(index)
    }

    pub fn draw_instances(&mut self, _mesh_id: u32, count: usize) {
        let draw_count = count.min(self.instances.len());
        self.instance_count = draw_count;
    }

    pub fn clear(&mut self) {
        self.instances.clear();
        self.instance_count = 0;
    }

    pub fn reserve(&mut self, additional: usize) {
        let new_capacity = self.instances.len() + additional;
        if new_capacity > self.max_instances {
            self.max_instances = new_capacity;
        }
        self.instances.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.instances.shrink_to_fit();
    }

    pub fn instance_count(&self) -> usize {
        self.instances.len()
    }

    pub fn max_instances(&self) -> usize {
        self.max_instances
    }

    pub fn get_buffer_id(&self) -> u32 {
        self.instance_buffer
    }

    pub fn set_buffer_id(&mut self, buffer_id: u32) {
        self.instance_buffer = buffer_id;
    }

    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.instances.len() >= self.max_instances
    }

    pub fn available_capacity(&self) -> usize {
        self.max_instances - self.instances.len()
    }

    pub fn capacity(&self) -> usize {
        self.instances.capacity()
    }

    pub fn instances(&self) -> &[InstanceData] {
        &self.instances
    }

    pub fn instances_mut(&mut self) -> &mut Vec<InstanceData> {
        &mut self.instances
    }
}

impl Default for InstancedRenderer {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_data_new() {
        let transform = Mat4::IDENTITY;
        let color = Color4F::WHITE;
        let uv_rect = Rect::new(0.0, 0.0, 1.0, 1.0);
        
        let data = InstanceData::new(transform, color, uv_rect);
        assert_eq!(data.color, Color4F::WHITE);
    }

    #[test]
    fn test_instance_data_with_transform() {
        let transform = Mat4::IDENTITY;
        let data = InstanceData::with_transform(transform);
        
        assert_eq!(data.color, Color4F::WHITE);
        assert_eq!(data.uv_rect, Rect::new(0.0, 0.0, 1.0, 1.0));
    }

    #[test]
    fn test_instance_data_with_color() {
        let color = Color4F::RED;
        let data = InstanceData::with_color(color);
        
        assert_eq!(data.color, Color4F::RED);
        assert_eq!(data.transform, Mat4::IDENTITY);
    }

    #[test]
    fn test_instance_data_default() {
        let data = InstanceData::default();
        
        assert_eq!(data.color, Color4F::WHITE);
        assert_eq!(data.transform, Mat4::IDENTITY);
    }

    #[test]
    fn test_instanced_renderer_new() {
        let renderer = InstancedRenderer::new(100);
        
        assert_eq!(renderer.max_instances(), 100);
        assert_eq!(renderer.instance_count(), 0);
        assert!(renderer.is_empty());
    }

    #[test]
    fn test_instanced_renderer_with_buffer_id() {
        let renderer = InstancedRenderer::with_buffer_id(100, 42);
        
        assert_eq!(renderer.get_buffer_id(), 42);
        assert_eq!(renderer.max_instances(), 100);
    }

    #[test]
    fn test_add_instance() {
        let mut renderer = InstancedRenderer::new(10);
        let data = InstanceData::default();
        
        assert!(renderer.add_instance(data));
        assert_eq!(renderer.instance_count(), 1);
        assert!(!renderer.is_empty());
    }

    #[test]
    fn test_add_instance_full() {
        let mut renderer = InstancedRenderer::new(2);
        
        assert!(renderer.add_instance(InstanceData::default()));
        assert!(renderer.add_instance(InstanceData::default()));
        assert!(!renderer.add_instance(InstanceData::default()));
        
        assert!(renderer.is_full());
    }

    #[test]
    fn test_add_instances() {
        let mut renderer = InstancedRenderer::new(10);
        let instances = vec![
            InstanceData::default(),
            InstanceData::default(),
            InstanceData::default(),
        ];
        
        let added = renderer.add_instances(&instances);
        assert_eq!(added, 3);
        assert_eq!(renderer.instance_count(), 3);
    }

    #[test]
    fn test_add_instances_exceeds_capacity() {
        let mut renderer = InstancedRenderer::new(2);
        let instances = vec![
            InstanceData::default(),
            InstanceData::default(),
            InstanceData::default(),
        ];
        
        let added = renderer.add_instances(&instances);
        assert_eq!(added, 2);
        assert_eq!(renderer.instance_count(), 2);
    }

    #[test]
    fn test_remove_instance() {
        let mut renderer = InstancedRenderer::new(10);
        renderer.add_instance(InstanceData::default());
        
        let removed = renderer.remove_instance(0);
        assert!(removed.is_some());
        assert_eq!(renderer.instance_count(), 0);
    }

    #[test]
    fn test_remove_instance_invalid_index() {
        let mut renderer = InstancedRenderer::new(10);
        
        let removed = renderer.remove_instance(0);
        assert!(removed.is_none());
    }

    #[test]
    fn test_get_instance() {
        let mut renderer = InstancedRenderer::new(10);
        renderer.add_instance(InstanceData::default());
        
        let instance = renderer.get_instance(0);
        assert!(instance.is_some());
    }

    #[test]
    fn test_get_instance_mut() {
        let mut renderer = InstancedRenderer::new(10);
        renderer.add_instance(InstanceData::default());
        
        if let Some(instance) = renderer.get_instance_mut(0) {
            instance.color = Color4F::RED;
        }
        
        assert_eq!(renderer.get_instance(0).unwrap().color, Color4F::RED);
    }

    #[test]
    fn test_clear() {
        let mut renderer = InstancedRenderer::new(10);
        renderer.add_instance(InstanceData::default());
        renderer.add_instance(InstanceData::default());
        
        renderer.clear();
        assert_eq!(renderer.instance_count(), 0);
        assert!(renderer.is_empty());
    }

    #[test]
    fn test_draw_instances() {
        let mut renderer = InstancedRenderer::new(10);
        renderer.add_instance(InstanceData::default());
        renderer.add_instance(InstanceData::default());
        
        renderer.draw_instances(1, 2);
        assert_eq!(renderer.instance_count, 2);
    }

    #[test]
    fn test_draw_instances_exceeds_available() {
        let mut renderer = InstancedRenderer::new(10);
        renderer.add_instance(InstanceData::default());
        
        renderer.draw_instances(1, 10);
        assert_eq!(renderer.instance_count, 1);
    }

    #[test]
    fn test_reserve() {
        let mut renderer = InstancedRenderer::new(10);
        renderer.reserve(50);
        
        assert!(renderer.max_instances() >= 50);
    }

    #[test]
    fn test_shrink_to_fit() {
        let mut renderer = InstancedRenderer::new(1000);
        renderer.add_instance(InstanceData::default());
        
        renderer.shrink_to_fit();
        assert_eq!(renderer.capacity(), renderer.instance_count());
    }

    #[test]
    fn test_is_full() {
        let mut renderer = InstancedRenderer::new(1);
        assert!(!renderer.is_full());
        
        renderer.add_instance(InstanceData::default());
        assert!(renderer.is_full());
    }

    #[test]
    fn test_available_capacity() {
        let mut renderer = InstancedRenderer::new(10);
        assert_eq!(renderer.available_capacity(), 10);
        
        renderer.add_instance(InstanceData::default());
        assert_eq!(renderer.available_capacity(), 9);
    }

    #[test]
    fn test_set_buffer_id() {
        let mut renderer = InstancedRenderer::new(10);
        renderer.set_buffer_id(99);
        
        assert_eq!(renderer.get_buffer_id(), 99);
    }

    #[test]
    fn test_instances_access() {
        let mut renderer = InstancedRenderer::new(10);
        renderer.add_instance(InstanceData::default());
        
        let instances = renderer.instances();
        assert_eq!(instances.len(), 1);
    }

    #[test]
    fn test_instances_mut_access() {
        let mut renderer = InstancedRenderer::new(10);
        renderer.add_instance(InstanceData::default());
        
        let instances = renderer.instances_mut();
        instances[0].color = Color4F::BLUE;
        
        assert_eq!(renderer.get_instance(0).unwrap().color, Color4F::BLUE);
    }

    #[test]
    fn test_default_renderer() {
        let renderer = InstancedRenderer::default();
        assert_eq!(renderer.max_instances(), 1000);
    }

    #[test]
    fn test_multiple_operations() {
        let mut renderer = InstancedRenderer::new(10);
        
        renderer.add_instance(InstanceData::with_color(Color4F::RED));
        renderer.add_instance(InstanceData::with_color(Color4F::GREEN));
        renderer.add_instance(InstanceData::with_color(Color4F::BLUE));
        
        assert_eq!(renderer.instance_count(), 3);
        
        renderer.remove_instance(1);
        assert_eq!(renderer.instance_count(), 2);
        
        renderer.clear();
        assert_eq!(renderer.instance_count(), 0);
    }
}
