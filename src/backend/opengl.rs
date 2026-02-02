use super::device::{GraphicsDevice, ShaderProgram, BufferObject, TextureObject, FramebufferObject, BufferType, BufferUsage};
use std::rc::Rc;
use glow::{Context, HasContext};

#[derive(Debug)]
pub struct OpenGLBackend {
    device: GraphicsDevice,
    version: (u32, u32),
    renderer: String,
    vendor: String,
    current_program: u32,
}

impl OpenGLBackend {
    pub fn new(context: Rc<Context>) -> OpenGLBackend {
        OpenGLBackend {
            device: GraphicsDevice::new(context),
            version: (0, 0),
            renderer: String::new(),
            vendor: String::new(),
            current_program: 0,
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

    // Program management
    pub fn create_program(&mut self) -> ShaderProgram {
        self.device.create_program()
    }

    pub fn delete_program(&mut self, program: u32) {
        self.device.delete_program(program);
    }

    pub fn use_program(&mut self, program: u32) {
        if program == 0 {
             unsafe { self.device.context.use_program(None) };
             self.current_program = 0;
        } else if let Some(prog) = self.device.get_program(program) {
            if let Some(native) = prog.get_native() {
                unsafe { self.device.context.use_program(Some(native)) };
                self.current_program = program;
            }
        }
    }

    pub fn link_program(&mut self, program: u32) -> bool {
        if let Some(prog) = self.device.get_program(program) {
            if let Some(native) = prog.get_native() {
                unsafe {
                    self.device.context.link_program(native);
                    if !self.device.context.get_program_link_status(native) {
                        let log = self.device.context.get_program_info_log(native);
                        log::error!("Failed to link program: {}", log);
                        return false;
                    }
                }
            }
        }
        // Cache uniforms after linking
        self.device.cache_uniforms(program);
        true
    }

    // Shader management
    pub fn create_shader(&mut self, shader_type: u32) -> u32 {
        self.device.create_shader(shader_type)
    }

    pub fn delete_shader(&mut self, shader: u32) {
        self.device.delete_shader(shader);
    }

    pub fn shader_source(&self, shader: u32, source: &str) {
        if let Some(sh) = self.device.get_shader(shader) {
            if let Some(native) = sh.get_native() {
                unsafe { self.device.context.shader_source(native, source) };
            }
        }
    }

    pub fn compile_shader(&self, shader: u32) -> bool {
        if let Some(sh) = self.device.get_shader(shader) {
            if let Some(native) = sh.get_native() {
                unsafe {
                    self.device.context.compile_shader(native);
                    if !self.device.context.get_shader_compile_status(native) {
                        let log = self.device.context.get_shader_info_log(native);
                        log::error!("Failed to compile shader: {}", log);
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn attach_shader(&self, program: u32, shader: u32) {
        let prog_native = self.device.get_program(program).and_then(|p| p.get_native());
        let shader_native = self.device.get_shader(shader).and_then(|s| s.get_native());

        if let (Some(p), Some(s)) = (prog_native, shader_native) {
            unsafe { self.device.context.attach_shader(p, s) };
        }
    }

    pub fn detach_shader(&self, program: u32, shader: u32) {
        let prog_native = self.device.get_program(program).and_then(|p| p.get_native());
        let shader_native = self.device.get_shader(shader).and_then(|s| s.get_native());

        if let (Some(p), Some(s)) = (prog_native, shader_native) {
            unsafe { self.device.context.detach_shader(p, s) };
        }
    }


    // Uniforms
    pub fn get_program_uniform_location(&self, program: u32, name: &str) -> i32 {
        if let Some(prog) = self.device.get_program(program) {
            if let Some(loc) = prog.get_uniform_location(name) {
                return loc;
            }
        }
        -1
    }

    pub fn set_uniform_int(&self, program: u32, location: i32, value: i32) {
        if let Some(native_loc) = self.device.get_native_uniform(program, location) {
            unsafe { self.device.context.uniform_1_i32(Some(&native_loc), value) };
        }
    }

    pub fn set_uniform_matrix4fv(&self, location: i32, transpose: bool, matrix: &crate::math::Mat4) {
        if self.current_program != 0 {
            if let Some(native_loc) = self.device.get_native_uniform(self.current_program, location) {
                // glow Mat4 is [f32; 16], crate::math::Mat4 is likely distinct.
                // Assuming crate::math::Mat4 is column-major array of f32.
                let slice = matrix.to_array(); // Need to ensure Mat4 has to_array or is compatible
                unsafe { self.device.context.uniform_matrix_4_f32_slice(Some(&native_loc), transpose, &slice) };
            }
        }
    }
    // For now, implement simple ones for testing.

    // Buffers
    pub fn create_buffer(&mut self) -> BufferObject {
        self.device.create_buffer()
    }

    pub fn bind_buffer(&self, buffer_type: BufferType, buffer: u32) {
        let target = buffer_type.to_gl_enum();
        if buffer == 0 {
            unsafe { self.device.context.bind_buffer(target, None) };
        } else if let Some(buf) = self.device.get_buffer(buffer) {
            if let Some(native) = buf.get_native() {
                unsafe { self.device.context.bind_buffer(target, Some(native)) };
            }
        }
    }

    pub fn buffer_data(&self, buffer_type: BufferType, _size: usize, data: &[u8], usage: BufferUsage) {
        let target = buffer_type.to_gl_enum();
        let usage = usage.to_gl_enum();
        unsafe { self.device.context.buffer_data_u8_slice(target, data, usage) };
    }

    pub fn buffer_sub_data(&self, buffer_type: BufferType, offset: usize, data: &[u8]) {
        let target = buffer_type.to_gl_enum();
        unsafe { self.device.context.buffer_sub_data_u8_slice(target, offset as i32, data) };
    }

    pub fn delete_buffer(&mut self, buffer: &mut BufferObject) {
        self.device.delete_buffer(buffer);
    }
    
    // Vertex Arrays (if we use them, but glow might not expose VAO easily in all backends? 
    // glow has create_vertex_array. For now let's just use VBOs and enable vertex attribs manually.)
    
    pub fn enable_vertex_attrib_array(&self, index: u32) {
        unsafe { self.device.context.enable_vertex_attrib_array(index) };
    }
    
    pub fn disable_vertex_attrib_array(&self, index: u32) {
        unsafe { self.device.context.disable_vertex_attrib_array(index) };
    }
    
    pub fn vertex_attrib_pointer(&self, index: u32, size: i32, data_type: u32, normalized: bool, stride: i32, offset: i32) {
        unsafe { self.device.context.vertex_attrib_pointer_f32(index, size, data_type, normalized, stride, offset) };
    }
    
    pub fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        unsafe { self.device.context.draw_arrays(mode, first, count) };
    }
    
    pub fn draw_elements(&self, mode: u32, count: i32, type_: u32, offset: i32) {
        unsafe { self.device.context.draw_elements(mode, count, type_, offset) };
    }

    // Textures
    pub fn create_texture(&mut self) -> TextureObject {
        self.device.create_texture()
    }

    pub fn bind_texture(&self, target: u32, texture: u32) {
        if texture == 0 {
            unsafe { self.device.context.bind_texture(target, None) };
        } else if let Some(tex) = self.device.get_texture(texture) {
            if let Some(native) = tex.get_native() {
                unsafe { self.device.context.bind_texture(target, Some(native)) };
            }
        }
    }
    
    pub fn delete_texture(&mut self, texture: &mut TextureObject) {
        self.device.delete_texture(texture);
    }

    pub fn tex_image_2d(&self, target: u32, level: i32, internal_format: i32, width: u32, height: u32, border: i32, format: u32, data: Option<&[u8]>) {
        unsafe {
             self.device.context.tex_image_2d(target, level, internal_format, width as i32, height as i32, border, format, glow::UNSIGNED_BYTE, data);
        }
    }

    pub fn tex_sub_image_2d(&self, target: u32, level: i32, x_offset: i32, y_offset: i32, width: u32, height: u32, format: u32, data: &[u8]) {
        unsafe {
             self.device.context.tex_sub_image_2d(target, level, x_offset, y_offset, width as i32, height as i32, format, glow::UNSIGNED_BYTE, glow::PixelUnpackData::Slice(data));
        }
    }

    pub fn set_texture_params(&self, min_filter: u32, mag_filter: u32, wrap_s: u32, wrap_t: u32) {
        // This usually applies to currently bound texture.
        unsafe {
            self.device.context.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, min_filter as i32);
            self.device.context.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, mag_filter as i32);
            self.device.context.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, wrap_s as i32);
            self.device.context.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, wrap_t as i32);
        }
    }

    pub fn generate_mipmap(&self, target: u32) {
        unsafe { self.device.context.generate_mipmap(target) };
    }

    // Framebuffers
    pub fn create_framebuffer(&mut self) -> FramebufferObject {
        self.device.create_framebuffer()
    }

    pub fn bind_framebuffer(&self, target: u32, framebuffer: u32) {
         if framebuffer == 0 {
            unsafe { self.device.context.bind_framebuffer(target, None) };
        } else if let Some(fb) = self.device.get_framebuffer(framebuffer) {
            if let Some(native) = fb.get_native() {
                unsafe { self.device.context.bind_framebuffer(target, Some(native)) };
            }
        }
    }

    
    pub fn delete_framebuffer(&mut self, framebuffer: &mut FramebufferObject) {
        self.device.delete_framebuffer(framebuffer);
    }

    pub fn framebuffer_texture_2d(&self, target: u32, attachment: u32, tex_target: u32, texture: u32, level: i32) {
         if let Some(tex) = self.device.get_texture(texture) {
            if let Some(native) = tex.get_native() {
                unsafe { self.device.context.framebuffer_texture_2d(target, attachment, tex_target, Some(native), level) };
            }
        }
    }

    pub fn check_framebuffer_status(&self, target: u32) -> u32 {
        unsafe { self.device.context.check_framebuffer_status(target) }
    }

    // Misc
    pub fn set_viewport(&self, x: i32, y: i32, width: u32, height: u32) {
        self.device.set_viewport(x, y, width, height);
    }

    pub fn set_scissor_test(&self, enabled: bool) {
        self.device.set_scissor_test(enabled);
    }

    pub fn clear(&self, mask: u32) {
        self.device.clear(mask);
    }
    
    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { self.device.context.clear_color(r, g, b, a) };
    }

    pub fn enable(&self, cap: u32) {
        unsafe { self.device.context.enable(cap) };
    }

    pub fn disable(&self, cap: u32) {
        unsafe { self.device.context.disable(cap) };
    }

    pub fn blend_func(&self, sfactor: u32, dfactor: u32) {
        unsafe { self.device.context.blend_func(sfactor, dfactor) };
    }
}
