use crate::backend::opengl::OpenGLBackend;
use crate::base::types::Color4F;
use crate::base::{Ref, RefPtr};
use crate::math::Mat4;
use crate::platform::Image;
use crate::renderer::command::{
    CommandType, GroupCommand, MeshCommand, Quad, RenderCommand, Triangles,
};
use crate::renderer::material::Material;
use crate::renderer::pipeline::PipelineState;
use crate::renderer::texture::{PixelFormat, Texture2D};
use glow::Context;
use std::rc::Rc;

pub struct Renderer {
    commands: Vec<Box<dyn RenderCommand>>,
    command_queue: Vec<Box<dyn RenderCommand>>,
    current_material: Option<RefPtr<Material>>,
    current_pipeline: Option<RefPtr<PipelineState>>,
    is_recording: bool,
    frustum_culled: bool,
    view_projection: Mat4,
    backend: Option<OpenGLBackend>,
    default_program: u32,
    dynamic_vbo: u32,
    dynamic_ibo: u32,
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
            view_projection: Mat4::IDENTITY,
            backend: None,
            default_program: 0,
            dynamic_vbo: 0,
            dynamic_ibo: 0,
        }
    }

    pub fn init_backend(&mut self, context: Rc<Context>) {
        let mut backend = OpenGLBackend::new(context);
        backend.init();

        // Create default resources
        self.default_program = self.create_default_shader(&mut backend);
        let vbo = backend.create_buffer();
        self.dynamic_vbo = vbo.get_id();
        let ibo = backend.create_buffer();
        self.dynamic_ibo = ibo.get_id();

        self.backend = Some(backend);
    }

    fn create_default_shader(&self, backend: &mut OpenGLBackend) -> u32 {
        let vs_source = r#"#version 410 core
        layout (location = 0) in vec3 a_position;
        layout (location = 1) in vec2 a_texCoord;
        layout (location = 2) in vec4 a_color;
        
        out vec2 v_texCoord;
        out vec4 v_color;
        
        uniform mat4 u_MVPMatrix;
        
        void main() {
            gl_Position = u_MVPMatrix * vec4(a_position, 1.0);
            v_texCoord = a_texCoord;
            v_color = a_color;
        }
        "#;

        let fs_source = r#"#version 410 core
        in vec2 v_texCoord;
        in vec4 v_color;
        out vec4 FragColor;
        
        // uniform sampler2D u_texture;
        
        void main() {
            // For now, ignore texture and just use color
            FragColor = v_color; 
        }
        "#;

        let vs = backend.create_shader(glow::VERTEX_SHADER);
        backend.shader_source(vs, vs_source);
        if !backend.compile_shader(vs) {
            log::error!("Failed to compile default vertex shader");
            return 0;
        }

        let fs = backend.create_shader(glow::FRAGMENT_SHADER);
        backend.shader_source(fs, fs_source);
        if !backend.compile_shader(fs) {
            log::error!("Failed to compile default fragment shader");
            return 0;
        }

        let prog = backend.create_program();
        let prog_id = prog.get_id();

        backend.attach_shader(prog_id, vs);
        backend.attach_shader(prog_id, fs);
        if !backend.link_program(prog_id) {
            log::error!("Failed to link default program");
            return 0;
        }

        prog_id
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
        if let Some(backend) = &mut self.backend {
            backend.set_viewport(0, 0, 960, 640); // Hardcoded for now
            backend.clear_color(0.2, 0.3, 0.3, 1.0);
            backend.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }

        self.start_frame();

        // Sort commands by global order
        self.command_queue.sort_by(|a, b| {
            a.get_global_order()
                .partial_cmp(&b.get_global_order())
                .unwrap()
        });

        // Execute all commands
        let commands = std::mem::take(&mut self.command_queue);
        for command in commands {
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

    pub fn draw_triangles(&mut self, triangles: &Triangles) {
        // Simple immediate mode drawing for now
        if let Some(backend) = &mut self.backend {
            if self.default_program == 0 {
                return;
            }

            backend.use_program(self.default_program);

            // Set MVP matrix (Identity for now, or view_projection)
            // Need to locate uniform first.
            let mvp_loc = backend.get_program_uniform_location(self.default_program, "u_MVPMatrix");
            backend.set_uniform_matrix4fv(mvp_loc, false, &self.view_projection);

            // Bind buffers
            backend.bind_buffer(crate::backend::device::BufferType::VERTEX, self.dynamic_vbo);

            let vert_bytes = unsafe {
                std::slice::from_raw_parts(
                    triangles.vertices.as_ptr() as *const u8,
                    triangles.vertices.len()
                        * std::mem::size_of::<crate::renderer::command::Vertex>(),
                )
            };
            backend.buffer_data(
                crate::backend::device::BufferType::VERTEX,
                vert_bytes.len(),
                vert_bytes,
                crate::backend::device::BufferUsage::DYNAMIC,
            );

            // Enable attributes
            // Pos
            backend.enable_vertex_attrib_array(0);
            backend.vertex_attrib_pointer(0, 3, glow::FLOAT, false, 36, 0);
            // Tex
            backend.enable_vertex_attrib_array(1);
            backend.vertex_attrib_pointer(1, 2, glow::FLOAT, false, 36, 12);
            // Color
            backend.enable_vertex_attrib_array(2);
            backend.vertex_attrib_pointer(2, 4, glow::FLOAT, false, 36, 20);

            // Indices
            backend.bind_buffer(crate::backend::device::BufferType::INDEX, self.dynamic_ibo);
            let idx_bytes = unsafe {
                std::slice::from_raw_parts(
                    triangles.indices.as_ptr() as *const u8,
                    triangles.indices.len() * 2,
                )
            };
            backend.buffer_data(
                crate::backend::device::BufferType::INDEX,
                idx_bytes.len(),
                idx_bytes,
                crate::backend::device::BufferUsage::DYNAMIC,
            );

            // Draw
            backend.draw_elements(
                glow::TRIANGLES,
                triangles.indices.len() as i32,
                glow::UNSIGNED_SHORT,
                0,
            );
        }
    }

    pub fn draw_quad(&mut self, quad: &Quad, material: RefPtr<Material>) {
        self.current_material = Some(material);
    }

    pub fn draw_mesh(&mut self, mesh: &MeshCommand) {}

    pub fn draw_group(&mut self, group: &GroupCommand) {}

    pub fn set_pipeline(&mut self, pipeline: RefPtr<PipelineState>) {
        self.current_pipeline = Some(pipeline);
    }

    pub fn get_pipeline(&self) -> Option<&RefPtr<PipelineState>> {
        self.current_pipeline.as_ref()
    }

    pub fn set_depth_test_enabled(&mut self, enabled: bool) {}

    pub fn set_cull_mode(&mut self, mode: CullMode) {}

    pub fn set_blend_func(&mut self, src: u32, dst: u32) {}

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
    pub fn create_texture_from_image(&mut self, image: &Image) -> Option<Texture2D> {
        if let Some(backend) = &mut self.backend {
            let texture_obj = backend.create_texture();

            // Map Image format to PixelFormat/GL format
            // For now assume RGBA8888
            let pixel_format = PixelFormat::RGBA8888;

            backend.bind_texture(glow::TEXTURE_2D, texture_obj.get_id());

            // Upload data
            backend.set_texture_params(
                glow::LINEAR,
                glow::LINEAR,
                glow::CLAMP_TO_EDGE,
                glow::CLAMP_TO_EDGE,
            );

            backend.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                image.get_width(),
                image.get_height(),
                0,
                glow::RGBA,
                Some(image.get_data()),
            );

            let mut texture = Texture2D::new();
            texture.set_name(texture_obj.get_id());
            texture.update(
                image.get_data(),
                image.get_width(),
                image.get_height(),
                pixel_format,
            );

            Some(texture)
        } else {
            None
        }
    }
}

impl std::fmt::Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Renderer")
            .field("commands_count", &self.commands.len())
            .field("command_queue_count", &self.command_queue.len())
            .field("current_material", &self.current_material)
            .field("current_pipeline", &self.current_pipeline)
            .field("is_recording", &self.is_recording)
            .field("frustum_culled", &self.frustum_culled)
            .field("view_projection", &self.view_projection)
            .field("backend", &self.backend)
            .finish()
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
        ScissorRect {
            x,
            y,
            width,
            height,
        }
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
        ViewPort {
            left,
            bottom,
            width,
            height,
            scale: 1.0,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_new() {
        let renderer = Renderer::new();
        assert_eq!(renderer.commands.len(), 0);
        assert_eq!(renderer.command_queue.len(), 0);
        assert!(!renderer.is_recording);
        assert!(!renderer.frustum_culled);
    }

    #[test]
    fn test_renderer_start_end_frame() {
        let mut renderer = Renderer::new();
        assert!(!renderer.is_recording);

        renderer.start_frame();
        assert!(renderer.is_recording);
        assert_eq!(renderer.commands.len(), 0);

        renderer.end_frame();
        assert!(!renderer.is_recording);
    }

    #[test]
    fn test_renderer_clear() {
        let mut renderer = Renderer::new();
        renderer.clear();
        assert_eq!(renderer.command_queue.len(), 0);
        assert!(renderer.current_material.is_none());
        assert!(renderer.current_pipeline.is_none());
    }

    #[test]
    fn test_renderer_view_projection_matrix() {
        let mut renderer = Renderer::new();
        let matrix = Mat4::IDENTITY;
        renderer.set_view_projection_matrix(matrix);
        assert_eq!(renderer.get_view_projection_matrix(), Mat4::IDENTITY);
    }

    #[test]
    fn test_renderer_get_rendertarget_size() {
        let renderer = Renderer::new();
        let (width, height) = renderer.get_rendertarget_size();
        assert_eq!(width, 1920);
        assert_eq!(height, 1080);
    }

    #[test]
    fn test_renderer_dimensions() {
        let renderer = Renderer::new();
        assert_eq!(renderer.get_width(), 1920);
        assert_eq!(renderer.get_height(), 1080);
    }

    #[test]
    fn test_renderer_scale() {
        let renderer = Renderer::new();
        assert!((renderer.get_scaleX() - 1.0).abs() < 0.001);
        assert!((renderer.get_scaleY() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_renderer_gamma() {
        let renderer = Renderer::new();
        assert!((renderer.get_gamma_zero() - 1.0).abs() < 0.001);
        assert!((renderer.get_gamma_squared() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_cull_mode_default() {
        assert_eq!(CullMode::default(), CullMode::Back);
    }

    #[test]
    fn test_cull_mode_variants() {
        assert_eq!(CullMode::None, CullMode::None);
        assert_eq!(CullMode::Front, CullMode::Front);
        assert_eq!(CullMode::Back, CullMode::Back);
    }

    #[test]
    fn test_scissor_rect_new() {
        let rect = ScissorRect::new(0, 0, 100, 200);
        assert_eq!(rect.get_x(), 0);
        assert_eq!(rect.get_y(), 0);
        assert_eq!(rect.get_width(), 100);
        assert_eq!(rect.get_height(), 200);
    }

    #[test]
    fn test_scissor_rect_is_valid() {
        let rect_valid = ScissorRect::new(0, 0, 100, 200);
        assert!(rect_valid.is_valid());

        let rect_invalid_width = ScissorRect::new(0, 0, 0, 200);
        assert!(!rect_invalid_width.is_valid());

        let rect_invalid_height = ScissorRect::new(0, 0, 100, 0);
        assert!(!rect_invalid_height.is_valid());
    }

    #[test]
    fn test_viewport_new() {
        let vp = ViewPort::new(0.0, 0.0, 960.0, 640.0);
        assert_eq!(vp.get_left(), 0.0);
        assert_eq!(vp.get_bottom(), 0.0);
        assert_eq!(vp.get_width(), 960.0);
        assert_eq!(vp.get_height(), 640.0);
        assert!((vp.get_scale() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_viewport_set_scale() {
        let mut vp = ViewPort::new(0.0, 0.0, 960.0, 640.0);
        vp.set_scale(2.0);
        assert!((vp.get_scale() - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_renderer_pipeline_operations() {
        let mut renderer = Renderer::new();
        assert!(renderer.get_pipeline().is_none());

        let pipeline = RefPtr::new(PipelineState::new());
        renderer.set_pipeline(pipeline.clone());
        assert!(renderer.get_pipeline().is_some());
        assert_eq!(renderer.get_pipeline().unwrap().borrow().get_name(), "");
    }

    #[test]
    fn test_renderer_debug_format() {
        let renderer = Renderer::new();
        let debug_str = format!("{:?}", renderer);
        assert!(debug_str.contains("Renderer"));
        assert!(debug_str.contains("commands_count"));
    }
}
