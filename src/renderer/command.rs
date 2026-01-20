use crate::math::Mat4;
use crate::base::Ref;
use crate::base::types::Color4F;

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

#[derive(Debug, Clone)]
pub struct Triangles {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub blend_func: (u32, u32),
    pub texture: Option<Ref<Texture>>,
    pub model_matrix: Mat4,
}

impl Triangles {
    pub fn new() -> Triangles {
        Triangles {
            vertices: Vec::new(),
            indices: Vec::new(),
            blend_func: (770, 771),
            texture: None,
            model_matrix: Mat4::identity(),
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
pub struct Quad {
    pub tl: Vertex,
    pub tr: Vertex,
    pub bl: Vertex,
    pub br: Vertex,
    pub blend_func: (u32, u32),
    pub texture: Option<Ref<Texture>>,
    pub model_matrix: Mat4,
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
            model_matrix: Mat4::identity(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
pub struct MeshCommand {
    command_type: CommandType,
    global_order: f32,
    material_id: u32,
    mesh_data: Vec<f32>,
    indices_data: Vec<u16>,
    transform: Mat4,
}

impl MeshCommand {
    pub fn new() -> MeshCommand {
        MeshCommand {
            command_type: CommandType::Mesh,
            global_order: 0.0,
            material_id: 0,
            mesh_data: Vec::new(),
            indices_data: Vec::new(),
            transform: Mat4::identity(),
        }
    }

    pub fn init(&mut self, material_id: u32, mesh_data: Vec<f32>, indices_data: Vec<u16>, transform: Mat4) {
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
        // Implementation in Renderer::draw_mesh
    }
}

#[derive(Debug, Clone)]
pub struct GroupCommand {
    command_type: CommandType,
    global_order: f32,
    group_id: i32,
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
        // Implementation in Renderer
    }
}

#[derive(Debug, Clone)]
pub struct CallbackCommand {
    command_type: CommandType,
    global_order: f32,
    callback: Box<dyn Fn(&mut Renderer)>,
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

#[derive(Debug, Clone)]
pub struct CustomCommand {
    command_type: CommandType,
    global_order: f32,
    depth: f32,
    callback: Box<dyn Fn(&mut Renderer)>,
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
