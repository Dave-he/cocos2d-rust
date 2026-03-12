#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::{Renderer, Texture2D};
use crate::base::types::Color4F;
use crate::base::RefPtr;
use crate::math::Mat4;
use crate::renderer::material::Material;

pub trait RenderCommand {
    fn get_command_type(&self) -> CommandType;
    fn get_global_order(&self) -> f32;
    fn execute(&self, renderer: &mut Renderer);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandType {
    Unknown,
    Triangles,
    Quad,
    Mesh,
    Group,
    Custom,
    Callback,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 2],
    pub color: Color4F,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: [0.0, 0.0, 0.0],
            tex_coord: [0.0, 0.0],
            color: Color4F::WHITE,
        }
    }
}

impl Vertex {
    pub fn with_position(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            position: [x, y, z],
            tex_coord: [0.0, 0.0],
            color: Color4F::WHITE,
        }
    }

    pub fn with_tex_coord(u: f32, v: f32) -> Vertex {
        Vertex {
            position: [0.0, 0.0, 0.0],
            tex_coord: [u, v],
            color: Color4F::WHITE,
        }
    }

    pub fn with_color(color: Color4F) -> Vertex {
        Vertex {
            position: [0.0, 0.0, 0.0],
            tex_coord: [0.0, 0.0],
            color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Triangles {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub blend_func: (u32, u32),
    pub texture: Option<RefPtr<Texture2D>>,
    pub model_matrix: Mat4,
}

impl Default for Triangles {
    fn default() -> Self {
        Self::new()
    }
}

impl Triangles {
    pub fn new() -> Triangles {
        Triangles {
            vertices: Vec::new(),
            indices: Vec::new(),
            blend_func: (770, 771),
            texture: None,
            model_matrix: Mat4::IDENTITY,
        }
    }

    pub fn get_vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn get_index_count(&self) -> usize {
        self.indices.len()
    }
}

#[derive(Debug, Clone)]
pub struct TrianglesCommand {
    command_type: CommandType,
    global_order: f32,
    triangles: Triangles,
    material: Option<RefPtr<Material>>,
}

impl Default for TrianglesCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl TrianglesCommand {
    pub fn new() -> TrianglesCommand {
        TrianglesCommand {
            command_type: CommandType::Triangles,
            global_order: 0.0,
            triangles: Triangles::new(),
            material: None,
        }
    }

    pub fn init(
        &mut self,
        global_order: f32,
        texture: Option<RefPtr<Texture2D>>,
        vertices: Vec<Vertex>,
        indices: Vec<u16>,
        blend_func: (u32, u32),
        model_matrix: Mat4,
    ) {
        self.global_order = global_order;
        self.triangles.texture = texture;
        self.triangles.vertices = vertices;
        self.triangles.indices = indices;
        self.triangles.blend_func = blend_func;
        self.triangles.model_matrix = model_matrix;
    }
}

impl RenderCommand for TrianglesCommand {
    fn get_command_type(&self) -> CommandType {
        self.command_type
    }

    fn get_global_order(&self) -> f32 {
        self.global_order
    }

    fn execute(&self, renderer: &mut Renderer) {
        renderer.draw_triangles(&self.triangles);
    }
}

#[derive(Debug, Clone)]
pub struct Quad {
    pub tl: Vertex,
    pub tr: Vertex,
    pub bl: Vertex,
    pub br: Vertex,
    pub blend_func: (u32, u32),
    pub texture: Option<RefPtr<Texture2D>>,
    pub model_matrix: Mat4,
}

impl Default for Quad {
    fn default() -> Self {
        Self::new()
    }
}

impl Quad {
    pub fn new() -> Quad {
        Quad {
            tl: Vertex::default(),
            tr: Vertex::default(),
            bl: Vertex::default(),
            br: Vertex::default(),
            blend_func: (770, 771),
            texture: None,
            model_matrix: Mat4::IDENTITY,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeshCommand {
    command_type: CommandType,
    global_order: f32,
    material_id: u32,
    mesh_data: Vec<f32>,
    indices_data: Vec<u16>,
    transform: Mat4,
}

impl Default for MeshCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshCommand {
    pub fn new() -> MeshCommand {
        MeshCommand {
            command_type: CommandType::Mesh,
            global_order: 0.0,
            material_id: 0,
            mesh_data: Vec::new(),
            indices_data: Vec::new(),
            transform: Mat4::IDENTITY,
        }
    }

    pub fn init(
        &mut self,
        material_id: u32,
        mesh_data: Vec<f32>,
        indices_data: Vec<u16>,
        transform: Mat4,
    ) {
        self.material_id = material_id;
        self.mesh_data = mesh_data;
        self.indices_data = indices_data;
        self.transform = transform;
    }
}

impl RenderCommand for MeshCommand {
    fn get_command_type(&self) -> CommandType {
        self.command_type
    }

    fn get_global_order(&self) -> f32 {
        self.global_order
    }

    fn execute(&self, _renderer: &mut Renderer) {
    }
}

#[derive(Debug, Clone)]
pub struct GroupCommand {
    command_type: CommandType,
    global_order: f32,
    group_id: i32,
}

impl Default for GroupCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl GroupCommand {
    pub fn new() -> GroupCommand {
        GroupCommand {
            command_type: CommandType::Group,
            global_order: 0.0,
            group_id: 0,
        }
    }
}

impl RenderCommand for GroupCommand {
    fn get_command_type(&self) -> CommandType {
        self.command_type
    }

    fn get_global_order(&self) -> f32 {
        self.global_order
    }

    fn execute(&self, _renderer: &mut Renderer) {
    }
}

pub struct CallbackCommand {
    command_type: CommandType,
    global_order: f32,
    callback: Box<dyn Fn(&mut Renderer)>,
}

impl Default for CallbackCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl CallbackCommand {
    pub fn new() -> CallbackCommand {
        CallbackCommand {
            command_type: CommandType::Callback,
            global_order: 0.0,
            callback: Box::new(|_renderer| {}),
        }
    }

    pub fn init<F: Fn(&mut Renderer) + 'static>(&mut self, callback: F) {
        self.callback = Box::new(callback);
    }
}

impl RenderCommand for CallbackCommand {
    fn get_command_type(&self) -> CommandType {
        self.command_type
    }

    fn get_global_order(&self) -> f32 {
        self.global_order
    }

    fn execute(&self, renderer: &mut Renderer) {
        (self.callback)(renderer);
    }
}

pub struct CustomCommand {
    command_type: CommandType,
    global_order: f32,
    depth: f32,
    callback: Box<dyn Fn(&mut Renderer)>,
}

impl std::fmt::Debug for CustomCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomCommand")
            .field("command_type", &self.command_type)
            .field("global_order", &self.global_order)
            .field("depth", &self.depth)
            .finish()
    }
}

impl Default for CustomCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomCommand {
    pub fn new() -> CustomCommand {
        CustomCommand {
            command_type: CommandType::Custom,
            global_order: 0.0,
            depth: 0.0,
            callback: Box::new(|_renderer| {}),
        }
    }

    pub fn init<F: Fn(&mut Renderer) + 'static>(&mut self, callback: F) {
        self.callback = Box::new(callback);
    }

    pub fn set_depth(&mut self, depth: f32) {
        self.depth = depth;
    }
}

impl RenderCommand for CustomCommand {
    fn get_command_type(&self) -> CommandType {
        self.command_type
    }

    fn get_global_order(&self) -> f32 {
        self.global_order
    }

    fn execute(&self, renderer: &mut Renderer) {
        (self.callback)(renderer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_type_traits() {
        assert_eq!(CommandType::Triangles, CommandType::Triangles);
        assert_ne!(CommandType::Triangles, CommandType::Quad);
        assert_eq!(CommandType::Unknown as u8, 0);
    }

    #[test]
    fn test_vertex_default() {
        let vertex = Vertex::default();
        assert_eq!(vertex.position, [0.0, 0.0, 0.0]);
        assert_eq!(vertex.tex_coord, [0.0, 0.0]);
    }

    #[test]
    fn test_vertex_with_position() {
        let vertex = Vertex::with_position(1.0, 2.0, 3.0);
        assert_eq!(vertex.position, [1.0, 2.0, 3.0]);
        assert_eq!(vertex.tex_coord, [0.0, 0.0]);
    }

    #[test]
    fn test_vertex_with_tex_coord() {
        let vertex = Vertex::with_tex_coord(0.5, 0.75);
        assert_eq!(vertex.position, [0.0, 0.0, 0.0]);
        assert_eq!(vertex.tex_coord, [0.5, 0.75]);
    }

    #[test]
    fn test_vertex_with_color() {
        let color = Color4F::RED;
        let vertex = Vertex::with_color(color);
        assert_eq!(vertex.color.r, 1.0);
        assert_eq!(vertex.color.g, 0.0);
    }

    #[test]
    fn test_triangles_new() {
        let triangles = Triangles::new();
        assert_eq!(triangles.get_vertex_count(), 0);
        assert_eq!(triangles.get_index_count(), 0);
        assert_eq!(triangles.blend_func, (770, 771));
    }

    #[test]
    fn test_triangles_with_data() {
        let mut triangles = Triangles::new();
        triangles.vertices.push(Vertex::with_position(0.0, 0.0, 0.0));
        triangles.vertices.push(Vertex::with_position(1.0, 0.0, 0.0));
        triangles.vertices.push(Vertex::with_position(0.0, 1.0, 0.0));
        triangles.indices.push(0);
        triangles.indices.push(1);
        triangles.indices.push(2);

        assert_eq!(triangles.get_vertex_count(), 3);
        assert_eq!(triangles.get_index_count(), 3);
    }

    #[test]
    fn test_triangles_command_new() {
        let cmd = TrianglesCommand::new();
        assert_eq!(cmd.get_command_type(), CommandType::Triangles);
        assert_eq!(cmd.get_global_order(), 0.0);
    }

    #[test]
    fn test_triangles_command_init() {
        let mut cmd = TrianglesCommand::new();
        let vertices = vec![
            Vertex::with_position(0.0, 0.0, 0.0),
            Vertex::with_position(1.0, 0.0, 0.0),
            Vertex::with_position(0.0, 1.0, 0.0),
        ];
        let indices = vec![0, 1, 2];
        let matrix = Mat4::IDENTITY;

        cmd.init(1.5, None, vertices, indices, (770, 771), matrix);

        assert_eq!(cmd.get_global_order(), 1.5);
        assert_eq!(cmd.triangles.get_vertex_count(), 3);
        assert_eq!(cmd.triangles.get_index_count(), 3);
    }

    #[test]
    fn test_quad_new() {
        let quad = Quad::new();
        assert_eq!(quad.blend_func, (770, 771));
    }

    #[test]
    fn test_mesh_command_new() {
        let cmd = MeshCommand::new();
        assert_eq!(cmd.get_command_type(), CommandType::Mesh);
        assert_eq!(cmd.get_global_order(), 0.0);
    }

    #[test]
    fn test_mesh_command_init() {
        let mut cmd = MeshCommand::new();
        let mesh_data = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let indices_data = vec![0, 1, 2];
        let matrix = Mat4::IDENTITY;

        cmd.init(42, mesh_data, indices_data, matrix);

        assert_eq!(cmd.material_id, 42);
        assert_eq!(cmd.mesh_data.len(), 6);
        assert_eq!(cmd.indices_data.len(), 3);
    }

    #[test]
    fn test_group_command_new() {
        let cmd = GroupCommand::new();
        assert_eq!(cmd.get_command_type(), CommandType::Group);
        assert_eq!(cmd.get_global_order(), 0.0);
        assert_eq!(cmd.group_id, 0);
    }

    #[test]
    fn test_callback_command_new() {
        let cmd = CallbackCommand::new();
        assert_eq!(cmd.get_command_type(), CommandType::Callback);
        assert_eq!(cmd.get_global_order(), 0.0);
    }

    #[test]
    fn test_callback_command_init() {
        let mut cmd = CallbackCommand::new();
        let called = std::cell::RefCell::new(false);
        let called_clone = called.clone();
        let callback = Box::new(move |_renderer: &mut Renderer| {
            *called_clone.borrow_mut() = true;
        });
        cmd.init(callback);
        assert!(!*called.borrow());
    }

    #[test]
    fn test_custom_command_new() {
        let cmd = CustomCommand::new();
        assert_eq!(cmd.get_command_type(), CommandType::Custom);
        assert_eq!(cmd.get_global_order(), 0.0);
        assert_eq!(cmd.depth, 0.0);
    }

    #[test]
    fn test_custom_command_set_depth() {
        let mut cmd = CustomCommand::new();
        cmd.set_depth(5.0);
        assert_eq!(cmd.depth, 5.0);
    }

    #[test]
    fn test_custom_command_init() {
        let mut cmd = CustomCommand::new();
        let called = std::cell::RefCell::new(false);
        let called_clone = called.clone();
        cmd.init(move |_renderer| {
            *called_clone.borrow_mut() = true;
        });
        assert!(!*called.borrow());
    }

    #[test]
    fn test_all_command_types() {
        let types = [
            CommandType::Unknown,
            CommandType::Triangles,
            CommandType::Quad,
            CommandType::Mesh,
            CommandType::Group,
            CommandType::Custom,
            CommandType::Callback,
        ];
        assert_eq!(types.len(), 7);
    }

    #[test]
    fn test_vertex_debug_format() {
        let vertex = Vertex::with_position(1.0, 2.0, 3.0);
        let debug_str = format!("{:?}", vertex);
        assert!(debug_str.contains("1"));
        assert!(debug_str.contains("2"));
        assert!(debug_str.contains("3"));
    }
}
