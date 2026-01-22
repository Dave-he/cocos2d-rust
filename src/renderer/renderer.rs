use crate::base::Ref;
use crate::base::types::Color4F;
use crate::math::Mat4;
use crate::renderer::command::{RenderCommand, CommandType, Triangles, Quad, MeshCommand, GroupCommand};
use crate::renderer::material::Material;
use crate::renderer::pipeline::PipelineState;

#[derive(Debug)]
pub struct Renderer {
    commands: Vec<Box<dyn RenderCommand>>,
    command_queue: Vec<Box<dyn RenderCommand>>,
    current_material: Option<Ref<Material>>,
    current_pipeline: Option<Ref<PipelineState>>,
    is_recording: bool,
    frustum_culled: bool,
    view_projection: Mat4,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            commands: Vec::new(),
            command_queue: Vec::new(),
            current_material: None,
            current_pipeline: None,
            is_recording: false,
            frustum_culled: false,
            view_projection: Mat4::identity(),
        }
    }

    pub fn init(&mut self) {
        self.commands.clear();
        self.command_queue.clear();
    }

    pub fn start_frame(&mut self) {
        self.commands.clear();
        self.is_recording = true;
    }

    pub fn end_frame(&mut self) {
        self.is_recording = false;
    }

    pub fn add_command(&mut self, command: Box<dyn RenderCommand>) {
        if self.is_recording {
            self.command_queue.push(command);
        }
    }

    pub fn push_command(&mut self, command: Box<dyn RenderCommand>) {
        self.command_queue.push(command);
    }

    pub fn pop_command(&mut self) -> Option<Box<dyn RenderCommand>> {
        self.command_queue.pop()
    }

    pub fn render(&mut self) {
        self.start_frame();

        // Sort commands by global order
        self.command_queue.sort_by(|a, b| {
            a.get_global_order().partial_cmp(&b.get_global_order()).unwrap()
        });

        // Execute all commands
        for command in &self.command_queue {
            command.execute(self);
        }

        self.end_frame();
    }

    pub fn clear(&mut self) {
        self.command_queue.clear();
        self.current_material = None;
        self.current_pipeline = None;
    }

    pub fn set_view_projection_matrix(&mut self, mat: Mat4) {
        self.view_projection = mat;
    }

    pub fn get_view_projection_matrix(&self) -> Mat4 {
        self.view_projection
    }

    pub fn draw_triangles(&mut self, triangles: &Triangles, material: Ref<Material>) {
        self.current_material = Some(material);
    }

    pub fn draw_quad(&mut self, quad: &Quad, material: Ref<Material>) {
        self.current_material = Some(material);
    }

    pub fn draw_mesh(&mut self, mesh: &MeshCommand) {
    }

    pub fn draw_group(&mut self, group: &GroupCommand) {
    }

    pub fn set_pipeline(&mut self, pipeline: Ref<PipelineState>) {
        self.current_pipeline = Some(pipeline);
    }

    pub fn get_pipeline(&self) -> Option<&Ref<PipelineState>> {
        self.current_pipeline.as_ref()
    }

    pub fn set_depth_test_enabled(&mut self, enabled: bool) {
    }

    pub fn set_cull_mode(&mut self, mode: CullMode) {
    }

    pub fn set_blend_func(&mut self, src: u32, dst: u32) {
    }

    pub fn get_rendertarget_size(&self) -> (u32, u32) {
        (1920, 1080)
    }

    pub fn get_width(&self) -> u32 {
        1920
    }

    pub fn get_height(&self) -> u32 {
        1080
    }

    pub fn get_scaleX(&self) -> f32 {
        1.0
    }

    pub fn get_scaleY(&self) -> f32 {
        1.0
    }

    pub fn get_gamma_zero(&self) -> f32 {
        1.0
    }

    pub fn get_gamma_squared(&self) -> f32 {
        1.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    None,
    Front,
    Back,
}

impl Default for CullMode {
    fn default() -> Self {
        CullMode::Back
    }
}

#[derive(Debug)]
pub struct ScissorRect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl ScissorRect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> ScissorRect {
        ScissorRect { x, y, width, height }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn is_valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }
}

#[derive(Debug, Clone)]
pub struct ViewPort {
    left: f32,
    bottom: f32,
    width: f32,
    height: f32,
    scale: f32,
}

impl ViewPort {
    pub fn new(left: f32, bottom: f32, width: f32, height: f32) -> ViewPort {
        ViewPort { left, bottom, width, height, scale: 1.0 }
    }

    pub fn get_left(&self) -> f32 {
        self.left
    }

    pub fn get_bottom(&self) -> f32 {
        self.bottom
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }
}
