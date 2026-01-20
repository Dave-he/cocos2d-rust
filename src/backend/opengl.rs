use super::device::{GraphicsDevice, ShaderProgram, BufferObject, TextureObject, FramebufferObject, BufferType, BufferUsage};

#[derive(Debug)]
pub struct OpenGLBackend {
    device: GraphicsDevice,
    version: (u32, u32),
    renderer: String,
    vendor: String,
}

impl OpenGLBackend {
    pub fn new() -> OpenGLBackend {
        OpenGLBackend {
            device: GraphicsDevice::new(),
            version: (0, 0),
            renderer: String::new(),
            vendor: String::new(),
        }
    }

    pub fn init(&mut self) -> bool {
        true
    }

    pub fn get_device(&self) -> &GraphicsDevice {
        &self.device
    }

    pub fn get_device_mut(&mut self) -> &mut GraphicsDevice {
        &mut self.device
    }

    pub fn get_version(&self) -> (u32, u32) {
        self.version
    }

    pub fn get_renderer(&self) -> &str {
        &self.renderer
    }

    pub fn get_vendor(&self) -> &str {
        &self.vendor
    }

    pub fn create_shader_program(&mut self) -> ShaderProgram {
        self.device.create_shader()
    }

    pub fn shader_source(&self, program: u32, source: &str) {
    }

    pub fn compile_shader(&self, shader: u32) -> bool {
        true
    }

    pub fn link_program(&self, program: u32) -> bool {
        true
    }

    pub fn use_program(&self, program: u32) {
    }

    pub fn get_program_uniform_location(&self, program: u32, name: &str) -> i32 {
        -1
    }

    pub fn set_uniform_int(&self, location: i32, value: i32) {
    }

    pub fn set_uniform_float(&self, location: i32, value: f32) {
    }

    pub fn set_uniform_vec2(&self, location: i32, x: f32, y: f32) {
    }

    pub fn set_uniform_vec3(&self, location: i32, x: f32, y: f32, z: f32) {
    }

    pub fn set_uniform_vec4(&self, location: i32, x: f32, y: f32, z: f32, w: f32) {
    }

    pub fn set_uniform_matrix4(&self, location: i32, transpose: bool, matrix: &[f32]) {
    }

    pub fn create_buffer(&mut self) -> BufferObject {
        self.device.create_buffer()
    }

    pub fn bind_buffer(&self, buffer_type: BufferType, buffer: u32) {
    }

    pub fn buffer_data(&self, buffer_type: BufferType, size: usize, data: &[u8], usage: BufferUsage) {
    }

    pub fn buffer_sub_data(&self, buffer_type: BufferType, offset: usize, data: &[u8]) {
    }

    pub fn create_texture(&mut self) -> TextureObject {
        self.device.create_texture()
    }

    pub fn bind_texture(&self, target: u32, texture: u32) {
    }

    pub fn tex_image_2d(&self, target: u32, level: i32, internal_format: i32, width: u32, height: u32, border: i32, format: u32, data: Option<&[u8]>) {
    }

    pub fn tex_sub_image_2d(&self, target: u32, level: i32, x_offset: i32, y_offset: i32, width: u32, height: u32, format: u32, data: &[u8]) {
    }

    pub fn set_texture_params(&self, min_filter: u32, mag_filter: u32, wrap_s: u32, wrap_t: u32) {
    }

    pub fn generate_mipmap(&self, target: u32) {
    }

    pub fn create_framebuffer(&mut self) -> FramebufferObject {
        self.device.create_framebuffer()
    }

    pub fn bind_framebuffer(&self, target: u32, framebuffer: u32) {
    }

    pub fn framebuffer_texture_2d(&self, target: u32, attachment: u32, tex_target: u32, texture: u32, level: i32) {
    }

    pub fn check_framebuffer_status(&self, target: u32) -> u32 {
        0
    }

    pub fn delete_shader_program(&mut self, program: &mut ShaderProgram) {
        self.device.delete_shader(program);
    }

    pub fn delete_buffer(&mut self, buffer: &mut BufferObject) {
        self.device.delete_buffer(buffer);
    }

    pub fn delete_texture(&mut self, texture: &mut TextureObject) {
        self.device.delete_texture(texture);
    }

    pub fn delete_framebuffer(&mut self, framebuffer: &mut FramebufferObject) {
        self.device.delete_framebuffer(framebuffer);
    }

    pub fn set_viewport(&self, x: i32, y: i32, width: u32, height: u32) {
        self.device.set_viewport(x, y, width, height);
    }

    pub fn set_scissor_test(&self, enabled: bool) {
        self.device.set_scissor_test(enabled);
    }

    pub fn clear(&self, mask: u32) {
        self.device.clear(mask);
    }

    pub fn enable(&self, cap: u32) {
    }

    pub fn disable(&self, cap: u32) {
    }

    pub fn blend_func(&self, sfactor: u32, dfactor: u32) {
    }

    pub fn depth_func(&self, func: u32) {
    }

    pub fn cull_face(&self, mode: u32) {
    }

    pub fn depth_mask(&self, flag: bool) {
    }

    pub fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) {
    }
}
